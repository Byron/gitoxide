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
use crate::read::{self, read_blob_to_buf_with_meta};

/// How the mode of an index entry has changed
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ModeChange {
    /// Shown as `typechange` in git status
    /// For example if a normal file was replaced with a symlink.
    /// Note: Except for submodules only files/symlinks are present in the
    /// the index so anything turning into a directory is counted as a removal
    TypeChange,
    /// The executable bit of a file changed
    ExecutableChange,
}

/// How a worktree file changed compared to an index entry
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct FileModification {
    /// How the mode has changed
    pub mode_change: Option<ModeChange>,
    /// mtime/ctime changed. If this is false then we can assume
    /// that the file is uncahged (with the exception of racy timestamps).
    /// If this is true however the file might still be unchaged. We need
    /// to read the file from disk and compare it to the object in
    /// index.
    pub stat_changed: bool,
    /// The data of this entry has changed. This can be quickly
    /// determined if the size of the stat data is mismatched.
    /// Otherwise a data change must be detected by reading the file
    /// from disk and comparing it to the file stored in the index
    /// (only needs to be done if `self.stat_changed` is true)
    pub data_changed: bool,
}

impl FileModification {
    /// Computes the status of an entry by comparing its stat to `symlink_metadata()`
    pub fn from_stat(
        entry: &index::Entry,
        fs_stat: &std::fs::Metadata,
        capabilites: &fs::Capabilities,
    ) -> Result<FileModification, SystemTimeError> {
        #[cfg(unix)]
        use std::os::unix::fs::MetadataExt;

        let mode_change = match entry.mode {
            index::entry::Mode::FILE if !fs_stat.is_file() => Some(ModeChange::TypeChange),
            #[cfg(unix)]
            index::entry::Mode::FILE if capabilites.executable_bit && fs_stat.mode() & 0o111 != 0 => {
                Some(ModeChange::ExecutableChange)
            }
            #[cfg(unix)]
            index::entry::Mode::FILE_EXECUTABLE if capabilites.executable_bit && fs_stat.mode() & 0o111 == 0 => {
                Some(ModeChange::ExecutableChange)
            }
            index::entry::Mode::SYMLINK if capabilites.symlink && !fs_stat.is_symlink() => Some(ModeChange::TypeChange),
            index::entry::Mode::SYMLINK if !capabilites.symlink && !fs_stat.is_file() => Some(ModeChange::TypeChange),
            index::entry::Mode::COMMIT if !fs_stat.is_dir() => Some(ModeChange::TypeChange),
            _ => None, // TODO: log/errror invalid file type
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

    /// Marks this entries stats as changed if there is a potential fs race condition
    pub fn detect_racy_stat(&mut self, index: &index::State, index_entry: &index::Entry) {
        self.stat_changed = self.stat_changed || index_entry.stat.mtime >= index.timestamp()
    }

    /// returns true if this entry has any changes
    /// usually `detect_racy_stat` should be called first to avoid race condition
    pub fn changed(&self) -> bool {
        self.mode_change.is_some() || self.stat_changed || self.data_changed
    }

    /// Reads the worktree file from the disk and compares it to
    /// the index entries oid to check if the actual data of the file is changed
    /// and sets [`Entry::data_changed`] accordingly
    pub fn compare_data(
        &mut self,
        worktree_path: &Path,
        index_entry: &index::Entry,
        buf: &mut Vec<u8>,
        capabilities: &fs::Capabilities,
    ) -> Result<(), read::Error> {
        if self.mode_change.is_some() || !self.stat_changed || self.data_changed {
            return Ok(());
        }
        let data = read_blob_to_buf_with_meta(
            worktree_path,
            index_entry.mode.contains(index::entry::Mode::SYMLINK),
            buf,
            capabilities,
        )?;
        let header = loose_header(gix_object::Kind::Blob, data.len());
        let hash_changed = match index_entry.id {
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

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not convert path to UTF8 {path}")]
    IllformedUtf8 { path: BString },
    #[error("The clock was off when reading file related metadata after updating a file on disk")]
    Time(#[from] std::time::SystemTimeError),
    #[error("IO error while writing blob or reading file metadata or changing filetype")]
    Io(#[from] io::Error),
}

#[derive(Clone, Debug)]
/// A change between the index and the worktree computed by [`compate_to_index`]
pub struct Change<'a> {
    /// The index entry that changed
    pub index_entry: &'a index::Entry,
    /// The on-disk worktree path corresponding to this entry
    pub worktree_path: PathBuf,
    /// How this index entry changed
    pub kind: ChangeKind,
    /// file metadata that can be reused (optimization)
    pub fstat: Option<std::fs::Metadata>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
///
pub enum ChangeKind {
    /// An index entry has no corresponding file in the worktree
    Removed,
    /// Ar new files that has been marked with git add but has not yet been
    /// checked in yet. No diff is computed for these files because whatever is
    /// on disk at commit time will be used
    Added,
    /// Called for files that may have changed in some form as indicated by `change`.
    /// Note that this doesn't necessarily mean that the *content* of the file changed
    /// see [`FileStatus`] for details
    Modified {
        /// How the file was modified exactly
        modification: FileModification,
        /// Whether this (changed) file also has an unresolved merge conflict
        conflict: bool,
    },
    /// There are unresolved merge conflicts for this file
    /// but it has not changed on disk
    Conflict,
}

/// Computes the changes between the index and the worktree
pub fn compare_to_index<'a: 'b, 'b>(
    index: &'a index::State,
    // TODO: use worktree cache instead
    worktree: &'b Path,
    capabilities: &'b fs::Capabilities,
) -> impl Iterator<Item = Result<Change<'a>, Error>> + 'b {
    // TODO: parallel with rayon
    index.entries().iter().filter_map(|index_entry| {
        let conflict = match index_entry.stage() {
            0 => false,
            1 => true,
            _ => return None,
        };
        let git_path = index_entry.path(index);
        if index_entry.flags.intersects(
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
            // we need to use fs::Cache for that
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
                    index_entry,
                    worktree_path,
                    fstat: Some(metadata),
                }));
            }
            Ok(metdata) => metdata,
            Err(err) if err.kind() == ErrorKind::NotFound => {
                return Some(Ok(Change {
                    kind: ChangeKind::Removed,
                    index_entry,
                    worktree_path,
                    fstat: None,
                }))
            }
            Err(err) => {
                // TODO: strict mode?
                return Some(Err(err.into()));
            }
        };
        if index_entry.flags.contains(index::entry::Flags::INTENT_TO_ADD) {
            return Some(Ok(Change {
                kind: ChangeKind::Added,
                index_entry,
                worktree_path,
                fstat: None,
            }));
        }
        let mut change = match FileModification::from_stat(index_entry, &metadata, capabilities) {
            Ok(change) => change,
            Err(err) => return Some(Err(err.into())),
        };
        change.detect_racy_stat(index, index_entry);

        let kind = if change.changed() {
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
            index_entry,
            worktree_path,
            fstat: Some(metadata),
        }))
    })
}
