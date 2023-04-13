use std::io;
use std::marker::PhantomData;
use std::path::Path;

use crate::index::status::diff::{self, Diff};
use crate::index::status::{Collector, Status};
use crate::{fs, read};
use bstr::BStr;
use filetime::FileTime;
use gix_features::parallel::{in_parallel_if, Reduce};
use gix_index as index;
use gix_path as path;

mod untracked;

/// The error returned by [`compare_to_index()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Could not convert path to UTF8")]
    IllformedUtf8,
    #[error("The clock was off when reading file related metadata after updating a file on disk")]
    Time(#[from] std::time::SystemTimeError),
    #[error("IO error while writing blob or reading file metadata or changing filetype")]
    Io(#[from] io::Error),
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
    pub stat: index::entry::stat::Options,
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
            stat: index::entry::stat::Options::default(),
            check_added: true,
        }
    }
}

/// Calculates the status of worktree
pub fn status<'index, T: Send>(
    index: &'index mut index::State,
    worktree: &Path,
    collector: &mut impl Collector<'index, Diff = T>,
    diff: &impl Diff<Output = T>,
    options: Options,
) -> Result<(), Error> {
    // the order is absoluty critical here
    // we use the old timestamp to detect racy index entries
    // (modified at or after the last index update) during
    // the index update we then set those entries size to 0 (see below)
    // to ensure they keep showing up as racy and reset the timestamp
    let timestamp = index.timestamp();
    index.set_timestamp(FileTime::now());
    let (chunk_size, thread_limit, _) = gix_features::parallel::optimize_chunk_size_and_thread_limit(
        100,
        index.entries().len().into(),
        options.thread_limit,
        None,
    );
    let path_backing = index.path_backing.as_slice();
    in_parallel_if(
        || true, // TODO: heuristic: when is parallelization not worth it?
        index.entries.chunks_mut(chunk_size),
        thread_limit,
        |_| State {
            buf: Vec::new(),
            timestamp,
            path_backing,
            worktree,
            options: &options,
        },
        |entries, state| {
            entries
                .iter_mut()
                .filter_map(|entry| state.process(entry, diff))
                .collect()
        },
        Reducer {
            collector,
            phantom: PhantomData,
        },
    )
}
struct State<'a, 'b> {
    buf: Vec<u8>,
    timestamp: FileTime,
    // path_cache: fs::Cache TODO path cache
    path_backing: &'b [u8],
    worktree: &'a Path,
    options: &'a Options,
}

type StatusResult<'index, T> = Result<(&'index index::Entry, &'index BStr, Status<T>, bool), Error>;

impl<'index> State<'_, 'index> {
    fn process<T>(
        &mut self,
        entry: &'index mut index::Entry,
        diff: &impl Diff<Output = T>,
    ) -> Option<StatusResult<'index, T>> {
        let conflict = match entry.stage() {
            0 => false,
            1 => true,
            _ => return None,
        };
        if entry.flags.intersects(
            index::entry::Flags::UPTODATE
                | index::entry::Flags::SKIP_WORKTREE
                | index::entry::Flags::ASSUME_VALID
                | index::entry::Flags::FSMONITOR_VALID,
        ) {
            return None;
        }
        let path = entry.path_in(self.path_backing);
        let status = self.compute_status(&mut *entry, path, diff);
        Some(status.map(move |status| (&*entry, path, status, conflict)))
    }

    fn compute_status<T>(
        &mut self,
        entry: &mut index::Entry,
        git_path: &BStr,
        diff: &impl Diff<Output = T>,
    ) -> Result<Status<T>, Error> {
        // TODO fs caache
        let worktree_path = path::try_from_bstr(git_path).map_err(|_| Error::IllformedUtf8)?;
        let worktree_path = self.worktree.join(worktree_path);
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
                return Ok(Status::Removed);
            }
            Ok(metadata) => metadata,
            Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(Status::Removed),
            Err(err) => {
                return Err(err.into());
            }
        };
        if self.options.check_added && entry.flags.contains(index::entry::Flags::INTENT_TO_ADD) {
            return Ok(Status::Added);
        }
        let new_stat = index::entry::Stat::from_fs(&metadata)?;
        let executable_bit_changed =
            match entry
                .mode
                .change_to_match_fs(&metadata, self.options.fs.symlink, self.options.fs.executable_bit)
            {
                Some(index::entry::mode::Change::Type { .. }) => return Ok(Status::TypeChange),
                Some(index::entry::mode::Change::ExecutableBit) => true,
                None => false,
            };

        // Here we implement racy-git. See racy-git.txt in the git documentation for a detailed documentation
        // A file is racy if:
        // * it's mtime is at/after the last index timestamp, it's entry stat information
        //   matches the on-disk file but the file contents are actually modified
        // * it's size is 0 (set after detecting a file was racy previously)
        //
        // The first case is detected below (by checking the timestamp if the file is marekd umodified).
        // The second case is usually detected either because the on-disk file
        // is not empty (hence the basic stat match fails) or by checking
        // whether the size doesn't fit the oid (the oid of an empty blob is a constant)
        let mut racy_clean = false;
        if !executable_bit_changed
            && new_stat.matches(&entry.stat, self.options.stat)
            && (!entry.id.is_empty_blob() || entry.stat.size == 0)
        {
            racy_clean = new_stat.is_racy(self.timestamp, self.options.stat);
            if !racy_clean {
                return Ok(Status::Unchanged);
            }
        }

        let file = WorktreeFile {
            buf: &mut self.buf,
            path: &worktree_path,
            entry,
            options: self.options,
        };
        let diff = diff.content_changed::<Error>(entry, metadata.len() as usize, file, |_| Ok(&[]))?;
        // this file is racy clean! Set the size to 0 so we keep detecting this
        // as the file is updated
        if diff.is_some() && racy_clean {
            entry.stat.size = 0;
        }
        if diff.is_some() || executable_bit_changed {
            Ok(Status::Modified {
                executable_bit_changed,
                diff,
            })
        } else {
            // don't diff against this file next time since
            // we know the file is unchanged
            entry.stat = new_stat;
            Ok(Status::Unchanged)
        }
    }
}

struct Reducer<'a, 'index, T: Collector<'index>> {
    collector: &'a mut T,
    phantom: PhantomData<fn(&'index ())>,
}

impl<'index, T, C: Collector<'index, Diff = T>> Reduce for Reducer<'_, 'index, C> {
    type Input = Vec<StatusResult<'index, T>>;

    type FeedProduce = ();

    type Output = ();

    type Error = Error;

    fn feed(&mut self, items: Self::Input) -> Result<Self::FeedProduce, Self::Error> {
        for item in items {
            let (entry, path, status, conflict) = item?;
            self.collector.visit_entry(entry, path, status, conflict);
        }
        Ok(())
    }

    fn finalize(self) -> Result<Self::Output, Self::Error> {
        Ok(())
    }
}
struct WorktreeFile<'a> {
    buf: &'a mut Vec<u8>,
    path: &'a Path,
    entry: &'a index::Entry,
    options: &'a Options,
}

impl<'a> diff::LazyBlob<'a, Error> for WorktreeFile<'a> {
    fn read(self) -> Result<&'a [u8], Error> {
        let res = read::data_to_buf_with_meta(
            self.path,
            self.buf,
            self.entry.mode == index::entry::Mode::SYMLINK,
            &self.options.fs,
        )?;
        Ok(res)
    }
}
