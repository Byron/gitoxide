use std::io;
use std::path::Path;
use std::time::SystemTimeError;

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
    fn from_fstat(
        entry: &index::Entry,
        fstat: &std::fs::Metadata,
        Options { fs, stat_options, .. }: &Options,
    ) -> Result<Self, SystemTimeError> {
        #[cfg(unix)]
        use std::os::unix::fs::MetadataExt;

        let mode_change = match entry.mode {
            index::entry::Mode::FILE if !fstat.is_file() => Some(ModeChange::TypeChange),
            #[cfg(unix)]
            index::entry::Mode::FILE if fs.executable_bit && fstat.mode() & 0o111 != 0 => {
                Some(ModeChange::ExecutableChange)
            }
            #[cfg(unix)]
            index::entry::Mode::FILE_EXECUTABLE if fs.executable_bit && fstat.mode() & 0o111 == 0 => {
                Some(ModeChange::ExecutableChange)
            }
            index::entry::Mode::SYMLINK if fs.symlink && !fstat.is_symlink() => Some(ModeChange::TypeChange),
            index::entry::Mode::SYMLINK if !fs.symlink && !fstat.is_file() => Some(ModeChange::TypeChange),
            index::entry::Mode::COMMIT if !fstat.is_dir() => Some(ModeChange::TypeChange),
            _ => None, // TODO: log/error invalid file type
        };

        let data_changed = entry.stat.size as u64 != fstat.len();
        let stat_changed = index::entry::Stat::from_fs(fstat)?.matches(&entry.stat, *stat_options);

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

#[derive(Clone)]
/// Options that control how the index status of a worktree is computed
pub struct Options {
    /// capabilities of the file system
    pub fs: crate::fs::Capabilities,
    /// If set, don't use more than this amount of threads.
    /// Otherwise, usually use as many threads as there are logical cores.
    /// A value of 0 is interpreted as no-limit
    pub thread_limit: Option<usize>,
    /// If true, default false, try don't abort on first error which isn't
    /// due to a conflict.
    /// The checkout operation will never fail, but count the encountered errors instead along with their paths.
    pub keep_going: bool,
    /// Options that control how stat comparisons are made
    /// when checking if a file is fresh
    pub stat_options: index::entry::stat::Options,
    // /// A group of attribute patterns that are applied globally, i.e. aren't rooted within the repository itself.
    // pub attribute_globals: gix_attributes::MatchGroup<Attributes>,
    /// Untracked files that were added to the index with git add but not yet committed
    /// are marked with special flags and usually receive special treatment. If this option
    /// is enabled (default true) added events are generated for these files, otherwise
    /// these files are treated the same as other entries
    pub check_added: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            fs: fs::Capabilities::default(),
            thread_limit: None,
            keep_going: false,
            stat_options: index::entry::stat::Options::default(),
            check_added: true,
        }
    }
}

impl<'index> IndexStatus<'index> {
    /// Calculates the status of worktree
    pub fn of_worktree(self, worktree: &Path, visit: &mut impl Visit<'index>, options: Options) {
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
                Ok(path) => self.of_file(entry, path, &options),
                Err(_) => Err(Error::IllformedUtf8),
            };

            visit.visit_entry(entry, status, path.as_deref().map_err(|_| git_path), conflict);
        }
    }

    fn of_file(&self, entry: &'index index::Entry, path: &Path, options: &Options) -> Result<Status, Error> {
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
        if options.check_added && entry.flags.contains(index::entry::Flags::INTENT_TO_ADD) {
            return Ok(Status::Added);
        }
        let mut modification = Modification::from_fstat(entry, &metadata, options)?;
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
