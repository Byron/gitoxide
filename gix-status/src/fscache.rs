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
//!
//! On Windows, `std::fs::DirEntry::metadata()` reuses enumeration data without
//! an additional per-entry metadata call. Enumeration throughput may differ
//! from the previous custom 64-KiB batching; equivalent performance is not
//! assumed. This cache has no invalidation and is intended only for status.

use std::path::{Path, PathBuf};

use bstr::{BStr, BString, ByteSlice};

/// File metadata produced by the lazy cache for one worktree entry.
///
/// Carries enough information to determine file type, detect mode changes,
/// build a [`gix_index::entry::Stat`] for comparison, and short-circuit content
/// reads via file size.
///
/// Windows-only fields: this module is `#[cfg(windows)]`, and Windows batch
/// directory enumeration doesn't expose `dev`/`ino`/`uid`/`gid` or the
/// executable bit. The status pipeline's stat comparison on Windows compares
/// those `Stat` fields against matching zeros from
/// [`gix_index::entry::Stat::from_fs`]'s Windows branch, and git on Windows
/// defaults to `core.filemode=false`, so all five are simply omitted here.
#[derive(Debug, Clone, Copy)]
pub struct Stat {
    /// Whether this is a directory, with the same semantics as
    /// `std::fs::symlink_metadata`: `false` for directory symlinks and junctions
    /// even though the filesystem also marks those with the directory attribute.
    pub is_dir: bool,
    /// Whether this is a symlink, with the same semantics as
    /// `std::fs::symlink_metadata`: `true` only for name-surrogate reparse points
    /// (symlinks and junctions/mount points). Other reparse points — OneDrive
    /// cloud placeholders, ProjFS, app-exec links — are regular files/dirs, as
    /// the live `lstat` fallback would report them.
    pub is_symlink: bool,
    /// File size in bytes.
    pub size: u64,
    /// Modification time — seconds since Unix epoch.
    pub mtime_secs: u32,
    /// Modification time — nanoseconds component.
    pub mtime_nsecs: u32,
    /// Status/creation time — seconds since Unix epoch.
    ///
    /// On Windows this must be populated from the real `CreationTime`, not `mtime`:
    /// the stat comparison in the status pipeline compares `ctime.secs` by default
    /// (`trust_ctime=true`), and faking `ctime=mtime` causes spurious mismatches
    /// for any file whose creation-time and modification-time differ.
    pub ctime_secs: u32,
    /// Status/creation time — nanoseconds component.
    pub ctime_nsecs: u32,
}

impl Stat {
    /// Use the same time conversion as the live Windows index-stat path.
    /// Unavailable or unrepresentable times must use the live fallback instead.
    fn from_metadata(metadata: &std::fs::Metadata) -> Option<Self> {
        let mtime = gix_index::entry::stat::Time::try_from(metadata.modified().ok()?).ok()?;
        let ctime = gix_index::entry::stat::Time::try_from(metadata.created().ok()?).ok()?;
        Some(Stat {
            is_dir: metadata.is_dir(),
            is_symlink: metadata.is_symlink(),
            size: metadata.len(),
            mtime_secs: mtime.secs,
            mtime_nsecs: mtime.nsecs,
            ctime_secs: ctime.secs,
            ctime_nsecs: ctime.nsecs,
        })
    }

    /// Convert to gitoxide's [`Stat`](gix_index::entry::Stat) struct for index comparison.
    ///
    /// Truncates `size` from 64 to 32 bits — matching what
    /// [`gix_index::entry::Stat::from_fs`] does, so both code
    /// paths compare the same quantities. `dev`/`ino`/`uid`/`gid` are zeroed
    /// here to match what `from_fs` produces on Windows.
    pub(crate) fn to_stat(self) -> gix_index::entry::Stat {
        gix_index::entry::Stat {
            mtime: gix_index::entry::stat::Time {
                secs: self.mtime_secs,
                nsecs: self.mtime_nsecs,
            },
            ctime: gix_index::entry::stat::Time {
                secs: self.ctime_secs,
                nsecs: self.ctime_nsecs,
            },
            dev: 0,
            ino: 0,
            uid: 0,
            gid: 0,
            size: self.size as u32,
        }
    }
}

/// Metadata for one enumerated directory, keyed by direct filename.
type DirectoryEntries = hashbrown::HashMap<BString, Stat>;

/// Either a live `lstat` result or a precomputed [`Stat`] from the lazy
/// cache. Lets [`crate::index_as_worktree`] treat both shapes uniformly without
/// branching at every per-entry use site.
pub(crate) enum Metadata {
    Live(gix_index::fs::Metadata),
    Cached(Stat),
}

impl Metadata {
    pub(crate) fn is_dir(&self) -> bool {
        match self {
            Self::Live(m) => m.is_dir(),
            Self::Cached(c) => c.is_dir,
        }
    }

    pub(crate) fn is_symlink(&self) -> bool {
        match self {
            Self::Live(m) => m.is_symlink(),
            Self::Cached(c) => c.is_symlink,
        }
    }

    pub(crate) fn len(&self) -> u64 {
        match self {
            Self::Live(m) => m.len(),
            Self::Cached(c) => c.size,
        }
    }

    pub(crate) fn to_stat(&self) -> Result<gix_index::entry::Stat, std::time::SystemTimeError> {
        match self {
            Self::Live(m) => gix_index::entry::Stat::from_fs(m),
            Self::Cached(c) => Ok(c.to_stat()),
        }
    }

    pub(crate) fn mode_change(
        &self,
        entry_mode: gix_index::entry::Mode,
        has_symlinks: bool,
        executable_bit: bool,
    ) -> Option<gix_index::entry::mode::Change> {
        match self {
            Self::Live(m) => entry_mode.change_to_match_fs(m, has_symlinks, executable_bit),
            // Windows batch enumeration doesn't expose the executable bit; pass `false`.
            // Git on Windows defaults to `core.filemode=false` so this is unused anyway.
            // TODO(correctness): it seems we'd have to try to support this at some point,
            //                    depending on Git for Windows' implementation.
            Self::Cached(c) => entry_mode.change_to_match_fs_with_values(
                !c.is_dir && !c.is_symlink, // is_file: regular file (not dir, not symlink)
                c.is_dir,
                c.is_symlink,
                false,
                has_symlinks,
                executable_bit,
            ),
        }
    }
}

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
    pub(crate) fn get(&mut self, rela_path: &BStr) -> Option<Stat> {
        let (dir, filename) = match rela_path.rfind_byte(b'/') {
            Some(pos) => (&rela_path[..pos], &rela_path[pos + 1..]),
            None => (BStr::new(b""), rela_path),
        };
        if filename.is_empty() {
            return None;
        }

        if let Some(entries) = self.directories.get(dir) {
            return entries.as_ref().and_then(|entries| entries.get(filename)).copied();
        }

        let entries = directory_entries(&self.worktree, dir);
        let stat = entries.as_ref().and_then(|entries| entries.get(filename)).copied();
        self.directories.insert(dir.into(), entries);
        stat
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
        let Some(stat) = entry
            .metadata()
            .ok()
            .and_then(|metadata| Stat::from_metadata(&metadata))
        else {
            continue;
        };
        entries.insert(filename.as_bytes().into(), stat);
    }
    Some(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gix_testtools::TestResult;
    use std::os::windows::{
        ffi::OsStringExt,
        fs::{symlink_dir, symlink_file},
    };

    fn fixture() -> TestResult<PathBuf> {
        let (path, ()) = gix_testtools::scripted_fixture_read_only_with_post("fscache.sh", 1, |fixture| {
            if let gix_testtools::FixtureState::Uninitialized(path) = fixture {
                // Ensure mtime differs from creation time, including a subsecond component.
                filetime::set_file_mtime(
                    path.join("file"),
                    filetime::FileTime::from_unix_time(1_700_000_000, 123_456_700),
                )?;
                std::fs::write(path.join("before-epoch"), b"old")?;
                filetime::set_file_mtime(path.join("before-epoch"), filetime::FileTime::from_unix_time(-1, 0))?;
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
        let cached = Metadata::Cached(cache.get(name.as_bytes().as_bstr()).expect("fixture entry is cached"));
        let live = Metadata::Live(gix_index::fs::Metadata::from_path_no_follow(
            &cache.worktree.join(name),
        )?);
        assert_eq!(cached.is_dir(), live.is_dir(), "directory classification for {name}");
        assert_eq!(
            cached.is_symlink(),
            live.is_symlink(),
            "symlink classification for {name}"
        );
        assert_eq!(cached.len(), live.len(), "full file size for {name}");
        assert_eq!(
            cached.to_stat()?,
            live.to_stat()?,
            "index size, mtime and creation time for {name}"
        );
        for mode in [
            gix_index::entry::Mode::FILE,
            gix_index::entry::Mode::DIR,
            gix_index::entry::Mode::SYMLINK,
        ] {
            for has_symlinks in [false, true] {
                assert_eq!(
                    cached.mode_change(mode, has_symlinks, false),
                    live.mode_change(mode, has_symlinks, false),
                    "mode changes for {name} with {mode:?}"
                );
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
                assert!(stat.is_symlink && !stat.is_dir, "links are never directories");
            }
        }
        let stat = cache.get(b"file".as_bstr()).expect("file is cached").to_stat();
        assert_ne!(
            stat.ctime, stat.mtime,
            "ctime uses creation time, not modification time"
        );
        assert_eq!(stat.mtime.nsecs, 123_456_700, "subsecond precision is preserved");
        assert_eq!(cache.directories.len(), 2, "only requested parents are enumerated");
        let junction = cache.get(b"junction".as_bstr()).expect("junction is cached");
        assert!(
            junction.is_symlink && !junction.is_dir,
            "junctions are symlinks, never directories"
        );
        Ok(())
    }

    #[test]
    fn unrepresentable_metadata_and_names_are_skipped() -> TestResult {
        let mut cache = FsCache::new(&fixture()?);
        assert!(
            cache.get(b"before-epoch".as_bstr()).is_none(),
            "pre-epoch timestamps must fall back, not saturate to zero"
        );
        let live = gix_index::fs::Metadata::from_path_no_follow(&cache.worktree.join("before-epoch"))?;
        assert!(
            gix_index::entry::Stat::from_fs(&live).is_err(),
            "the live path preserves the time conversion error"
        );
        assert!(
            cache.get("\u{fffd}".as_bytes().as_bstr()).is_none(),
            "unpaired surrogates must not become replacement-character keys"
        );
        assert_matches_live(&mut cache, "file")?;
        Ok(())
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
