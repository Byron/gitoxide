use std::io;
use std::path::Path;
use std::time::{Duration, SystemTimeError};

use crate::index::status::visit::worktree::Visit;
use crate::index::status::visit::worktree::{Error, Status};
use crate::index::status::visit::{ModeChange, Modification};
use crate::index::status::IndexStatus;
use crate::{fs, read};
use gix_features::hash;
use gix_hash::ObjectId;
use gix_index as index;
use gix_object::encode::loose_header;
use gix_path as path;

mod untracked;

/// Instantiation
impl Modification {
    /// Computes the status of an `entry` by comparing it with its `fs_stat` while respecting filesystem `capabilities`.
    ///
    /// It does so exclusively by looking at the filesystem stats.
    pub fn from_fstat(
        entry: &index::Entry,
        fs_stat: &std::fs::Metadata,
        capabilities: &fs::Capabilities,
    ) -> Result<Self, SystemTimeError> {
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
    ) -> io::Result<()> {
        if self.mode_change == Some(ModeChange::TypeChange) || self.data_changed {
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

impl<'index> IndexStatus<'index> {
    /// Calculates the status of worktree
    pub fn of_worktree(
        self,
        worktree: &Path,
        visit: &mut impl Visit<'index>,
        visit_added: bool,
        fs_capabilities: &fs::Capabilities,
    ) {
        for entry in self.index.entries() {
            let conflict = match entry.stage() {
                0 => false,
                1 => true,
                _ => continue,
            };
            if entry.flags.intersects(
                index::entry::Flags::UPTODATE
                    | index::entry::Flags::SKIP_WORKTREE
                    | index::entry::Flags::ASSUME_VALID
                    | index::entry::Flags::FSMONITOR_VALID,
            ) {
                continue;
            }

            let git_path = entry.path(self.index);
            let path = path::try_from_bstr(git_path).map(|path| worktree.join(path));
            let status = match &path {
                Ok(path) => self.of_file(entry, path, visit_added, fs_capabilities),
                Err(_) => Err(Error::IllformedUtf8),
            };

            visit.visit_entry(entry, status, path.as_deref().map_err(|_| git_path), conflict);
        }
    }

    fn of_file(
        &self,
        entry: &'index index::Entry,
        path: &Path,
        visit_added: bool,
        capabilities: &fs::Capabilities,
    ) -> Result<Status, Error> {
        let metadata = match path.symlink_metadata() {
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
                return Ok(Status::Removed);
            }
            Ok(metadata) => metadata,
            Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(Status::Removed),
            Err(err) => {
                return Err(err.into());
            }
        };
        if visit_added && entry.flags.contains(index::entry::Flags::INTENT_TO_ADD) {
            return Ok(Status::Added);
        }
        let mut modification = Modification::from_fstat(entry, &metadata, capabilities)?;
        modification.detect_racy_stat(self.index, entry);
        if modification.is_changed() {
            Ok(Status::Modified(modification))
        } else {
            Ok(Status::Unchanged)
        }
    }
}

impl Status {
    /// Checks if files with stat changes have changed content by reading their
    /// contents from the disk. If the file content is unchanged and there
    /// are no mode change the Status is changed to Unchanged
    pub fn compare_data(
        &mut self,
        path: &Path,
        entry: &index::Entry,
        buf: &mut Vec<u8>,
        capabilities: &fs::Capabilities,
    ) -> io::Result<()> {
        if let Status::Modified(status) = self {
            status.compare_data(path, entry, buf, capabilities)?;
            if !status.data_changed && status.mode_change.is_none() {
                *self = Status::Unchanged
            }
        }
        Ok(())
    }
}
