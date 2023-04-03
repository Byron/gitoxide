use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTimeError};

use bstr::BString;
use gix_features::hash;
use gix_hash::ObjectId;
use gix_index as index;
use gix_object::encode::loose_header;
use gix_path as path;

use crate::fs;
use crate::read;

/// How the mode of an index entry has changed.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ModeChange {
    /// Shown as `typechange` in `git status`.
    ///
    /// For example, this happens if a normal file was replaced with a symlink.
    /// **Note**: A directory turning into a file or vice-versa is not counted as `TypeChange`,
    /// but as addition and removal respectively.
    TypeChange,
    /// The executable bit of a file changed.
    ExecutableChange,
}

/// How a worktree file changed compared to an index entry.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct FileModification {
    /// If not `None`, the file mode was changed.
    pub mode_change: Option<ModeChange>,
    /// The `mtime` or `ctime` changed.
    ///
    /// If this is `false` then we can assume the file is unchanged
    /// assuming that timestamps where not racy (see [`detect_racy_stat()`][Self::detect_racy_stat()]).
    /// If this is `true`, the file might still be unchanged, and to be perfectly sure we would need
    /// to read the file from disk and compare it to the object in index.
    pub stat_changed: bool,
    /// The data of this entry has changed.
    ///
    /// This can be quickly determined if the size of the stat data is mismatched.
    /// Otherwise a data change must be detected by reading the file
    /// from disk and comparing it to the file stored in the index
    /// This only needs to be done if `stat_changed` is `true`.
    pub data_changed: bool,
}

/// Instantiation
impl FileModification {
    /// Computes the status of an `entry` by comparing it with its `fs_stat` while respecting filesystem `capabilities`.
    ///
    /// It does so exclusively by looking at the filesystem stats.
    pub fn from_stat(
        entry: &index::Entry,
        fs_stat: &std::fs::Metadata,
        capabilities: &fs::Capabilities,
    ) -> Result<FileModification, SystemTimeError> {
        #[cfg(unix)]
        use std::os::unix::fs::MetadataExt;

        let mode_change = match entry.mode {
            index::entry::Mode::FILE if !fs_stat.is_file() => Some(ModeChange::TypeChange),
            #[cfg(unix)]
            index::entry::Mode::FILE if capabilities.executable_bit && fs_stat.mode() & 0o111 != 0 => {
                Some(ModeChange::ExecutableChange)
            }
            #[cfg(unix)]
            index::entry::Mode::FILE_EXECUTABLE if capabilities.executable_bit && fs_stat.mode() & 0o111 == 0 => {
                Some(ModeChange::ExecutableChange)
            }
            index::entry::Mode::SYMLINK if capabilities.symlink && !fs_stat.is_symlink() => {
                Some(ModeChange::TypeChange)
            }
            index::entry::Mode::SYMLINK if !capabilities.symlink && !fs_stat.is_file() => Some(ModeChange::TypeChange),
            index::entry::Mode::COMMIT if !fs_stat.is_dir() => Some(ModeChange::TypeChange),
            _ => None, // TODO: log/error invalid file type
        };

        let data_changed = entry.stat.size as u64 != fs_stat.len();

        let ctime = fs_stat
            .created()
            .map_or(Ok(Duration::default()), |x| x.duration_since(std::time::UNIX_EPOCH))?;
        let mtime = fs_stat
            .modified()
            .map_or(Ok(Duration::default()), |x| x.duration_since(std::time::UNIX_EPOCH))?;

        let stat = &entry.stat;
        let stat_changed = stat.mtime.secs
            != mtime
                .as_secs()
                .try_into()
                .expect("by 2038 we found a solution for this")
            || stat.mtime.nsecs != mtime.subsec_nanos()
            || stat.ctime.secs
                != ctime
                    .as_secs()
                    .try_into()
                    .expect("by 2038 we found a solution for this")
            || stat.ctime.nsecs != ctime.subsec_nanos();

        Ok(Self {
            mode_change,
            stat_changed,
            data_changed,
        })
    }
}

/// Modification
impl FileModification {
    /// Marks this entry's stats as changed if there is a potential filesystem race condition.
    pub fn detect_racy_stat(&mut self, index: &index::State, index_entry: &index::Entry) {
        self.stat_changed = self.stat_changed || index_entry.stat.mtime >= index.timestamp()
    }

    /// Returns true if this instance has any changes.
    ///
    /// The [`detect_racy_stat()`][Self::detect_racy_stat()] method should be called first to account for  race conditions.
    pub fn is_changed(&self) -> bool {
        self.mode_change.is_some() || self.stat_changed || self.data_changed
    }

    /// Read the worktree file denoted by `entry` from the disk rooted at `worktree_path` into `buf` and compare
    /// it to the index entry's hash to check if the actual data of the file is changed to set [`Self::data_changed`] accordingly,
    /// while respecting the filesystem's `capabilities`.
    ///
    /// Does no computation if we are already sure that the file has or hasn't changed.
    pub fn compare_data(
        &mut self,
        worktree_path: &Path,
        entry: &index::Entry,
        buf: &mut Vec<u8>,
        capabilities: &fs::Capabilities,
    ) -> Result<(), read::Error> {
        if self.mode_change.is_some() || !self.stat_changed || self.data_changed {
            return Ok(());
        }
        let data = read::data_with_buf_and_meta(
            worktree_path,
            buf,
            entry.mode.contains(index::entry::Mode::SYMLINK),
            capabilities,
        )?;
        let header = loose_header(gix_object::Kind::Blob, data.len());
        let hash_changed = match entry.id {
            ObjectId::Sha1(entry_hash) => {
                let mut file_hash = hash::Sha1::default();
                file_hash.update(&header);
                file_hash.update(&data);
                let file_hash = file_hash.digest();
                entry_hash != file_hash
            }
        };
        self.data_changed = hash_changed;
        Ok(())
    }
}

/// The error returned by [`compare_to_index()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Could not convert path to UTF8 {path}")]
    IllformedUtf8 { path: BString },
    #[error("The clock was off when reading file related metadata after updating a file on disk")]
    Time(#[from] std::time::SystemTimeError),
    #[error("IO error while writing blob or reading file metadata or changing filetype")]
    Io(#[from] io::Error),
}

/// A change between the index and the worktree computed by [`compare_to_index`].
#[derive(Clone, Debug)]
pub struct Change<'a> {
    /// The index entry that changed.
    pub entry: &'a index::Entry,
    /// The on-disk worktree path corresponding to this entry.
    pub worktree_path: PathBuf,
    /// How this index entry changed.
    pub kind: ChangeKind,
    /// File metadata observed from disk that can be reused (optimization).
    pub fstat: Option<std::fs::Metadata>,
}

/// The nature of a the difference between the index and the worktree.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ChangeKind {
    /// An index entry has no corresponding file in the worktree.
    Removed,
    /// A new file that has been marked with git add but has not yet been
    /// checked in yet. No diff is computed for these files because whatever is
    /// on disk at commit time will be used.
    Added,
    /// Called for files that may have changed in some form as indicated by `change`.
    ///
    /// Note that this doesn't necessarily mean that the *content* of the file changed.
    Modified {
        /// How the file was modified exactly
        modification: FileModification,
        /// Whether this changed file also has an unresolved merge conflict.
        conflict: bool,
    },
    /// There are unresolved merge conflicts for this file but it has not changed on disk.
    Conflict,
}

/// Computes the changes needed to turn the `index` into the `worktree` (as identified by its root),
/// while respecting the filesystem's `capabilities`.
pub fn compare_to_index<'a: 'b, 'b>(
    index: &'a index::State,
    // TODO: use worktree cache instead
    worktree: &'b Path,
    capabilities: &'b fs::Capabilities,
) -> impl Iterator<Item = Result<Change<'a>, Error>> + 'b {
    index.entries().iter().filter_map(|entry| {
        let git_path = entry.path(index);
        if entry.flags.intersects(
            index::entry::Flags::UPTODATE
                | index::entry::Flags::SKIP_WORKTREE
                | index::entry::Flags::ASSUME_VALID
                | index::entry::Flags::FSMONITOR_VALID,
        ) {
            return None;
        }

        let path = if let Ok(path) = path::try_from_bstr(git_path) {
            path
        } else {
            return Some(Err(Error::IllformedUtf8 {
                path: git_path.to_owned(),
            }));
        };

        let worktree_path = worktree.join(path);
        let metadata = match worktree_path.symlink_metadata() {
            // TODO: check if any parent directory is a symlink
            //       we need to use fs::Cache for that
            Ok(metadata) if metadata.is_dir() => {
                // index entries are normally only for files/symlinks
                // if a file turned into a directory it was removed
                // the only exception here are submodules which are
                // part of the index despite being directories
                //
                // TODO: submodules:
                //   if entry.mode.contains(Mode::COMMIT) &&
                //     resolve_gitlink_ref(ce->name, "HEAD", &sub))
                return Some(Ok(Change {
                    kind: ChangeKind::Removed,
                    entry,
                    worktree_path,
                    fstat: Some(metadata),
                }));
            }
            Ok(metadata) => metadata,
            Err(err) if err.kind() == ErrorKind::NotFound => {
                return Some(Ok(Change {
                    kind: ChangeKind::Removed,
                    entry,
                    worktree_path,
                    fstat: None,
                }))
            }
            Err(err) => {
                // TODO: strict mode?
                return Some(Err(err.into()));
            }
        };
        if entry.flags.contains(index::entry::Flags::INTENT_TO_ADD) {
            return Some(Ok(Change {
                kind: ChangeKind::Added,
                entry,
                worktree_path,
                fstat: None,
            }));
        }
        let mut change = match FileModification::from_stat(entry, &metadata, capabilities) {
            Ok(change) => change,
            Err(err) => return Some(Err(err.into())),
        };
        change.detect_racy_stat(index, entry);

        let conflict = match entry.stage() {
            0 => false,
            1 => true,
            _ => return None,
        };
        let kind = if change.is_changed() {
            ChangeKind::Modified {
                modification: change,
                conflict,
            }
        } else if conflict {
            ChangeKind::Conflict
        } else {
            return None;
        };

        Some(Ok(Change {
            kind,
            entry,
            worktree_path,
            fstat: Some(metadata),
        }))
    })
}
