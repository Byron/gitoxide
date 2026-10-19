//! **Windows-only** worktree metadata caching. During per-entry modification
//! checks, parent directories can be enumerated lazily so that
//! `index_as_worktree` can reuse stat results instead of issuing a per-file
//! `lstat`. This trade only pays off where per-file stat is expensive
//! and where the dirwalk returns basic stat information, so only on Windows.
//!
//! [`FsCache`] lazily enumerates parent directories with
//! `std::fs::read_dir()` and keeps the results in the status worker
//! that asked for them. It is not a long-lived cache: it is built during one
//! status call and discarded with the worker state. Lookups are transparent:
//! misses fall through to a live syscall, so cache misses affect speed only.
//! Cached metadata uses [`gix_index::fs::Metadata`] so classification, mode
//! changes and index-stat normalization share the live path's interpretation.
//!
//! On Windows, `std::fs::DirEntry::metadata()` reuses enumeration data without
//! an additional per-entry metadata call. Enumeration throughput may differ
//! from the previous custom 64-KiB batching; equivalent performance is not
//! assumed. This cache has no invalidation and is intended only for status.

use std::path::{Path, PathBuf};

use bstr::{BStr, BString, ByteSlice};

/// Metadata for one enumerated directory, keyed by direct filename.
type DirectoryEntries = hashbrown::HashMap<BString, gix_index::fs::Metadata>;

/// Thread-local, lazily populated worktree metadata cache.
///
/// Each lookup caches the entry's parent directory, keyed by the worktree
/// relative directory path. Directory entries are keyed by filename only. This
/// keeps the cache independent from pathspecs and excludes while avoiding a
/// live `lstat` for every tracked file in directories that have already been
/// enumerated.
pub struct FsCache {
    /// Absolute worktree root used to turn repository-relative directory keys
    /// into filesystem paths for Windows directory enumeration.
    ///
    /// The cache never changes this root and never stores absolute paths as
    /// keys. Keeping the root separate lets lookups operate on Git-style
    /// forward-slashed paths while `directory_entries()` performs the
    /// platform-specific conversion at the boundary.
    worktree: PathBuf,
    /// Cache of direct directory listings, keyed by worktree-relative parent
    /// directory path.
    ///
    /// Keys use the same byte representation as index paths: forward slashes,
    /// no leading slash, and an empty key for the worktree root. Values are
    /// `Some(entries)` when that directory was enumerated successfully and
    /// `None` when enumeration failed, for example because the directory was
    /// removed or inaccessible. Caching failed enumerations prevents repeated
    /// directory-open attempts for multiple tracked entries in the same missing
    /// directory; callers still fall through to live per-file metadata and
    /// preserve correctness.
    ///
    /// `DirectoryEntries` itself is keyed only by direct filename, not by full
    /// worktree-relative path. This keeps each cached listing small and mirrors
    /// the Windows API result shape: first resolve the parent directory, then
    /// look up the requested basename in that listing.
    directories: hashbrown::HashMap<BString, Option<DirectoryEntries>>,
}

impl FsCache {
    /// Create an empty cache rooted at `worktree`.
    pub fn new(worktree: &Path) -> Self {
        FsCache {
            worktree: worktree.to_owned(),
            directories: Default::default(),
        }
    }

    /// Return cached metadata for `rela_path`, populating its parent directory
    /// on first use. `None` means the cache couldn't help and callers should
    /// fall back to a live `lstat`.
    pub(crate) fn get(&mut self, rela_path: &BStr) -> Option<&gix_index::fs::Metadata> {
        let (dir, filename) = match rela_path.rfind_byte(b'/') {
            Some(pos) => (&rela_path[..pos], &rela_path[pos + 1..]),
            None => (BStr::new(b""), rela_path),
        };
        if filename.is_empty() {
            return None;
        }

        self.directories
            .entry_ref(dir)
            .or_insert_with(|| directory_entries(&self.worktree, dir))
            .as_ref()?
            .get(filename)
    }
}

/// Read direct entries, retaining usable results even if individual entries fail.
/// Missing names always fall through to live metadata in the status worker.
fn directory_entries(worktree: &Path, rel_dir: &BStr) -> Option<DirectoryEntries> {
    let dir = if rel_dir.is_empty() {
        worktree.to_owned()
    } else {
        worktree.join(gix_path::from_bstr(rel_dir).ok()?)
    };
    let mut entries = DirectoryEntries::default();
    for entry in std::fs::read_dir(dir).ok()? {
        let Ok(entry) = entry else { continue };
        let filename = entry.file_name();
        let Some(filename) = filename.to_str() else { continue };
        // On Windows this reuses the directory-enumeration result, without a
        // per-entry metadata syscall, and does not follow symlinks or junctions.
        let Ok(metadata) = entry.metadata() else { continue };
        let metadata = gix_index::fs::Metadata::from(metadata);
        // Use shared index normalization to reject unusable timestamps and retain
        // the live fallback for them. Unavailable timestamps use the epoch, just
        // like live metadata; pre-epoch timestamps remain errors on Windows.
        if gix_index::entry::Stat::from_fs(&metadata).is_err() {
            continue;
        }
        entries.insert(filename.as_bytes().into(), metadata);
    }
    Some(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gix_error::TestResult;
    use std::os::windows::{
        ffi::OsStringExt,
        fs::{FileTimesExt, symlink_dir, symlink_file},
    };

    fn fixture() -> TestResult<PathBuf> {
        let (path, ()) = gix_testtools::scripted_fixture_read_only_with_post("fscache.sh", 2, |fixture| {
            if let gix_testtools::FixtureState::Uninitialized(path) = fixture {
                // Ensure mtime differs from creation time, including a subsecond component.
                std::fs::File::options()
                    .write(true)
                    .open(path.join("file"))?
                    .set_times(
                        std::fs::FileTimes::new()
                            .set_created(std::time::UNIX_EPOCH + std::time::Duration::new(1_600_000_000, 765_432_100)),
                    )?;
                filetime::set_file_mtime(
                    path.join("file"),
                    filetime::FileTime::from_unix_time(1_700_000_000, 123_456_700),
                )?;
                std::fs::write(path.join("before-epoch"), b"old")?;
                filetime::set_file_mtime(path.join("before-epoch"), filetime::FileTime::from_unix_time(-1, 0))?;
                std::fs::File::create(path.join("before-epoch-created"))?.set_times(
                    std::fs::FileTimes::new().set_created(std::time::UNIX_EPOCH - std::time::Duration::from_secs(1)),
                )?;
                // Mark the empty file sparse before extending it, avoiding a 4-GiB allocation.
                // Windows fixture generation never creates archives of this logical size.
                let sparse_path = path.join("sparse");
                let sparse = std::fs::File::create(&sparse_path)?;
                let output = std::process::Command::new("fsutil.exe")
                    .args(["sparse", "setflag"])
                    .arg(&sparse_path)
                    .output()?;
                if output.status.success() {
                    sparse.set_len((1_u64 << 32) + 123)?;
                } else {
                    eprintln!("Skipping large-file comparison: sparse files unavailable: {output:?}");
                }
                std::fs::write(
                    path.join(std::ffi::OsString::from_wide(&[0xd800])),
                    b"unpaired surrogate",
                )?;
                for result in [
                    symlink_file("file", path.join("file-link")),
                    symlink_dir("directory", path.join("directory-link")),
                ] {
                    match result {
                        Ok(()) => {}
                        Err(err) if err.raw_os_error() == Some(1314) => {
                            eprintln!("Skipping symlink comparison: symlink creation requires a privilege");
                        }
                        Err(err) => return Err(err.into()),
                    }
                }
                let output = std::process::Command::new("cmd.exe")
                    .args(["/D", "/C", "mklink", "/J", "junction", "directory"])
                    .current_dir(path)
                    .output()?;
                assert!(output.status.success(), "junction creation failed: {output:?}");
            }
            Ok(())
        })?;
        Ok(path)
    }

    fn assert_matches_live(cache: &mut FsCache, name: &str) -> TestResult {
        let live = gix_index::fs::Metadata::from_path_no_follow(&cache.worktree.join(name))?;
        let cached = cache.get(name.as_bytes().as_bstr()).expect("fixture entry is cached");
        assert_eq!(
            cached.is_file(),
            live.is_file(),
            "regular file classification for {name}"
        );
        assert_eq!(cached.is_dir(), live.is_dir(), "directory classification for {name}");
        assert_eq!(
            cached.is_symlink(),
            live.is_symlink(),
            "symlink classification for {name}"
        );
        assert_eq!(cached.len(), live.len(), "full file size for {name}");
        assert_eq!(
            gix_index::entry::Stat::from_fs(cached)?,
            gix_index::entry::Stat::from_fs(&live)?,
            "index size, mtime and creation time for {name}"
        );
        for mode in [
            gix_index::entry::Mode::FILE,
            gix_index::entry::Mode::FILE_EXECUTABLE,
            gix_index::entry::Mode::DIR,
            gix_index::entry::Mode::COMMIT,
            gix_index::entry::Mode::SYMLINK,
        ] {
            for has_symlinks in [false, true] {
                for executable_bit in [false, true] {
                    assert_eq!(
                        mode.change_to_match_fs(cached, has_symlinks, executable_bit),
                        mode.change_to_match_fs(&live, has_symlinks, executable_bit),
                        "mode changes for {name} with {mode:?}, symlinks={has_symlinks}, executable_bit={executable_bit}"
                    );
                }
            }
        }
        Ok(())
    }

    #[test]
    fn cached_metadata_matches_live() -> TestResult {
        let worktree = fixture()?;
        let mut cache = FsCache::new(&worktree);
        assert!(cache.directories.is_empty(), "construction does not enumerate");
        for name in ["file", "directory", "directory/file", "junction"] {
            assert_matches_live(&mut cache, name)?;
        }
        for name in ["file-link", "directory-link"] {
            if worktree.join(name).symlink_metadata().is_ok() {
                assert_matches_live(&mut cache, name)?;
                let stat = cache.get(name.as_bytes().as_bstr()).expect("link is cached");
                assert!(stat.is_symlink() && !stat.is_dir(), "links are never directories");
            } else {
                eprintln!("Skipping {name} comparison: fixture could not create symlink (privilege required)");
            }
        }
        let stat = gix_index::entry::Stat::from_fs(cache.get(b"file".as_bstr()).expect("file is cached"))?;
        assert_ne!(
            stat.ctime, stat.mtime,
            "ctime uses creation time, not modification time"
        );
        assert_eq!(stat.mtime.nsecs, 123_456_700, "subsecond precision is preserved");
        assert_eq!(
            stat.ctime.nsecs, 765_432_100,
            "creation-time subsecond precision is preserved"
        );
        assert_eq!(cache.directories.len(), 2, "only requested parents are enumerated");
        let junction = cache.get(b"junction".as_bstr()).expect("junction is cached");
        assert!(
            junction.is_symlink() && !junction.is_dir(),
            "junctions are symlinks, never directories"
        );
        Ok(())
    }

    #[test]
    fn unrepresentable_metadata_and_names_are_skipped() -> TestResult {
        let mut cache = FsCache::new(&fixture()?);
        for name in ["before-epoch", "before-epoch-created"] {
            assert!(
                cache.get(name.as_bytes().as_bstr()).is_none(),
                "pre-epoch timestamps must fall back, not saturate to zero: {name}"
            );
            let live = gix_index::fs::Metadata::from_path_no_follow(&cache.worktree.join(name))?;
            let err = gix_index::entry::Stat::from_fs(&live).expect_err("pre-epoch timestamps remain errors");
            assert_eq!(
                err.duration(),
                std::time::Duration::from_secs(1),
                "the original timestamp error survives"
            );
        }
        assert!(
            cache.get("\u{fffd}".as_bytes().as_bstr()).is_none(),
            "unpaired surrogates must not become replacement-character keys"
        );
        assert_matches_live(&mut cache, "file")?;
        Ok(())
    }

    #[test]
    fn full_size_is_preserved_beyond_the_index_limit() -> TestResult {
        let mut cache = FsCache::new(&fixture()?);
        let metadata = cache.get(b"sparse".as_bstr()).expect("sparse file is cached");
        if metadata.len() == 0 {
            eprintln!("Skipping large-file comparison: fixture filesystem could not mark the file sparse");
            return Ok(());
        }
        assert_eq!(
            metadata.len(),
            (1_u64 << 32) + 123,
            "full file size is available to content comparison"
        );
        assert_eq!(
            gix_index::entry::Stat::from_fs(metadata)?.size,
            123,
            "only the index size is truncated"
        );
        assert_matches_live(&mut cache, "sparse")
    }

    #[test]
    fn missing_entries_and_failed_enumerations_still_allow_live_metadata() -> TestResult {
        // This exercises writes after enumeration, so it needs a private writable fixture.
        let dir = gix_testtools::tempfile::tempdir()?;
        let mut cache = FsCache::new(dir.path());
        assert!(
            cache.get(b"file".as_bstr()).is_none(),
            "an absent entry misses the cache"
        );
        std::fs::write(dir.path().join("file"), b"new")?;
        assert!(
            cache.get(b"file".as_bstr()).is_none(),
            "a cached listing is not refreshed"
        );
        assert_eq!(
            gix_index::fs::Metadata::from_path_no_follow(&dir.path().join("file"))?.len(),
            3,
            "a cache miss must allow the live path to find a newly created entry"
        );

        assert!(
            cache.get(b"missing/file".as_bstr()).is_none(),
            "a missing parent cannot be enumerated"
        );
        assert!(
            cache
                .directories
                .get(b"missing".as_bstr())
                .expect("failed enumeration is cached")
                .is_none(),
            "failed enumeration remains cached separately from an empty listing"
        );
        std::fs::create_dir(dir.path().join("missing"))?;
        std::fs::write(dir.path().join("missing/file"), b"later")?;
        assert!(
            cache.get(b"missing/file".as_bstr()).is_none(),
            "failed enumeration is not retried"
        );
        assert_eq!(
            gix_index::fs::Metadata::from_path_no_follow(&dir.path().join("missing/file"))?.len(),
            5,
            "failed enumeration must still allow successful live metadata"
        );
        Ok(())
    }

    #[test]
    fn differently_cased_names_miss_the_cache() -> TestResult {
        let worktree = fixture()?;
        let mut cache = FsCache::new(&worktree);
        assert!(cache.get(b"File".as_bstr()).is_none(), "cache keys match exact names");
        let live = gix_index::fs::Metadata::from_path_no_follow(&worktree.join("File"));
        match live {
            Ok(live) => assert_eq!(live.len(), 5, "case-insensitive filesystems resolve the live fallback"),
            Err(err) => assert_eq!(
                err.kind(),
                std::io::ErrorKind::NotFound,
                "case-sensitive filesystems retain their native result"
            ),
        }
        assert_matches_live(&mut cache, "file")?;
        Ok(())
    }
}
