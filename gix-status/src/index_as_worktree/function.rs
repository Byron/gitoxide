use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::{io, marker::PhantomData, path::Path};

use bstr::BStr;
use filetime::FileTime;
use gix_features::parallel::{in_parallel_if, Reduce};

use crate::{
    index_as_worktree::{
        traits,
        traits::{CompareBlobs, SubmoduleStatus},
        types::{Error, Options},
        Change, Outcome, VisitEntry,
    },
    read, Pathspec,
};

/// Calculates the changes that need to be applied to an `index` to match the state of the `worktree` and makes them
/// observable in `collector`, along with information produced by `compare` which gets to see blobs that may have changes, and
/// `submodule` which can take a look at submodules in detail to produce status information.
/// `options` are used to configure the operation.
///
/// Note that `index` is updated with the latest seen stat information from the worktree, and its timestamp is adjusted to
/// the current time for which it will be considered fresh as long as it is included which depends on `pathspec`.
///
/// `should_interrupt` can be used to stop all processing.
///
/// ### Note
///
/// Technically, this function does more as this also provides additional information, like whether a file has conflicts,
/// and files that were added with `git add` are shown as a special as well. It also updates index entry stats like `git status` would
/// if it had to determine the hash. If that happened, the index should be written back, see [Outcome::skipped]
/// The latter is a 'change' that is not technically requiring a change to the index since `git add` already added the
/// file to the index, but didn't hash it.
///
/// Thus some care has to be taken to do the right thing when letting the index match the worktree by evaluating the changes observed
/// by the `collector`.
#[allow(clippy::too_many_arguments)]
pub fn index_as_worktree<'index, T, U, Find, E1, E2>(
    index: &'index mut gix_index::State,
    worktree: &Path,
    collector: &mut impl VisitEntry<'index, ContentChange = T, SubmoduleStatus = U>,
    compare: impl CompareBlobs<Output = T> + Send + Clone,
    submodule: impl SubmoduleStatus<Output = U, Error = E2> + Send + Clone,
    find: Find,
    progress: &mut dyn gix_features::progress::Progress,
    pathspec: impl Pathspec + Send + Clone,
    should_interrupt: &AtomicBool,
    options: Options,
) -> Result<Outcome, Error>
where
    T: Send,
    U: Send,
    E1: std::error::Error + Send + Sync + 'static,
    E2: std::error::Error + Send + Sync + 'static,
    Find: for<'a> FnMut(&gix_hash::oid, &'a mut Vec<u8>) -> Result<gix_object::BlobRef<'a>, E1> + Send + Clone,
{
    // the order is absolutely critical here we use the old timestamp to detect racy index entries
    // (modified at or after the last index update) during the index update we then set those
    // entries size to 0 (see below) to ensure they keep showing up as racy and reset the timestamp.
    let timestamp = index.timestamp();
    index.set_timestamp(FileTime::now());
    let (chunk_size, thread_limit, _) = gix_features::parallel::optimize_chunk_size_and_thread_limit(
        500, // just like git
        index.entries().len().into(),
        options.thread_limit,
        None,
    );

    let range = index
        .prefixed_entries_range(pathspec.common_prefix())
        .unwrap_or(0..index.entries().len());
    let (entries, path_backing) = index.entries_mut_and_pathbacking();
    let mut num_entries = entries.len();
    let entries = &mut entries[range];

    let _span = gix_features::trace::detail!("gix_status::index_as_worktree", 
                                             num_entries = entries.len(), 
                                             chunk_size = chunk_size,
                                             thread_limit = ?thread_limit);

    let entries_skipped_by_common_prefix = num_entries - entries.len();
    let (skipped_by_pathspec, skipped_by_entry_flags, symlink_metadata_calls, entries_updated) = Default::default();
    let (worktree_bytes, worktree_reads, odb_bytes, odb_reads, racy_clean) = Default::default();

    num_entries = entries.len();
    progress.init(entries.len().into(), gix_features::progress::count("files"));
    let count = progress.counter();

    in_parallel_if(
        || true, // TODO: heuristic: when is parallelization not worth it? Git says 500 items per thread, but to 20 threads, we can be more fine-grained though.
        gix_features::interrupt::Iter::new(entries.chunks_mut(chunk_size), should_interrupt),
        thread_limit,
        {
            let options = &options;
            let (skipped_by_pathspec, skipped_by_entry_flags) = (&skipped_by_pathspec, &skipped_by_entry_flags);
            let (symlink_metadata_calls, entries_updated) = (&symlink_metadata_calls, &entries_updated);
            let (racy_clean, worktree_bytes) = (&racy_clean, &worktree_bytes);
            let (worktree_reads, odb_bytes, odb_reads) = (&worktree_reads, &odb_bytes, &odb_reads);
            move |_| {
                (
                    State {
                        buf: Vec::new(),
                        odb_buf: Vec::new(),
                        path_stack: crate::SymlinkCheck::new(worktree.to_owned()),
                        timestamp,
                        path_backing,
                        options,

                        skipped_by_pathspec,
                        skipped_by_entry_flags,
                        symlink_metadata_calls,
                        entries_updated,
                        racy_clean,
                        worktree_reads,
                        worktree_bytes,
                        odb_reads,
                        odb_bytes,
                    },
                    compare,
                    submodule,
                    find,
                    pathspec,
                )
            }
        },
        |entries, (state, blobdiff, submdule, find, pathspec)| {
            entries
                .iter_mut()
                .filter_map(|entry| {
                    let res = state.process(entry, blobdiff, submdule, find, pathspec);
                    count.fetch_add(1, Ordering::Relaxed);
                    res
                })
                .collect()
        },
        ReduceChange {
            collector,
            phantom: PhantomData,
        },
    )?;

    Ok(Outcome {
        entries_to_process: num_entries,
        entries_processed: count.load(Ordering::Relaxed),
        entries_skipped_by_common_prefix,
        entries_skipped_by_pathspec: skipped_by_pathspec.load(Ordering::Relaxed),
        entries_skipped_by_entry_flags: skipped_by_entry_flags.load(Ordering::Relaxed),
        entries_updated: entries_updated.load(Ordering::Relaxed),
        symlink_metadata_calls: symlink_metadata_calls.load(Ordering::Relaxed),
        racy_clean: racy_clean.load(Ordering::Relaxed),
        worktree_files_read: worktree_reads.load(Ordering::Relaxed),
        worktree_bytes: worktree_bytes.load(Ordering::Relaxed),
        odb_objects_read: odb_reads.load(Ordering::Relaxed),
        odb_bytes: odb_bytes.load(Ordering::Relaxed),
    })
}

struct State<'a, 'b> {
    buf: Vec<u8>,
    odb_buf: Vec<u8>,
    timestamp: FileTime,
    path_stack: crate::SymlinkCheck,
    path_backing: &'b [u8],
    options: &'a Options,

    skipped_by_pathspec: &'a AtomicUsize,
    skipped_by_entry_flags: &'a AtomicUsize,
    symlink_metadata_calls: &'a AtomicUsize,
    entries_updated: &'a AtomicUsize,
    racy_clean: &'a AtomicUsize,
    worktree_bytes: &'a AtomicU64,
    worktree_reads: &'a AtomicUsize,
    odb_bytes: &'a AtomicU64,
    odb_reads: &'a AtomicUsize,
}

type StatusResult<'index, T, U> = Result<(&'index gix_index::Entry, &'index BStr, Option<Change<T, U>>, bool), Error>;

impl<'index> State<'_, 'index> {
    fn process<T, U, Find, E1, E2>(
        &mut self,
        entry: &'index mut gix_index::Entry,
        diff: &mut impl CompareBlobs<Output = T>,
        submodule: &mut impl SubmoduleStatus<Output = U, Error = E2>,
        find: &mut Find,
        pathspec: &mut impl Pathspec,
    ) -> Option<StatusResult<'index, T, U>>
    where
        E1: std::error::Error + Send + Sync + 'static,
        E2: std::error::Error + Send + Sync + 'static,
        Find: for<'a> FnMut(&gix_hash::oid, &'a mut Vec<u8>) -> Result<gix_object::BlobRef<'a>, E1>,
    {
        let conflict = match entry.stage() {
            0 => false,
            1 => true,
            _ => return None,
        };
        if entry.flags.intersects(
            gix_index::entry::Flags::UPTODATE
                | gix_index::entry::Flags::SKIP_WORKTREE
                | gix_index::entry::Flags::ASSUME_VALID
                | gix_index::entry::Flags::FSMONITOR_VALID,
        ) {
            self.skipped_by_entry_flags.fetch_add(1, Ordering::Relaxed);
            return None;
        }
        let path = entry.path_in(self.path_backing);
        if !pathspec.is_included(path, Some(false)) {
            self.skipped_by_pathspec.fetch_add(1, Ordering::Relaxed);
            return None;
        }
        let status = self.compute_status(&mut *entry, path, diff, submodule, find);
        Some(status.map(move |status| (&*entry, path, status, conflict)))
    }

    /// # On how racy-git is handled here
    ///
    /// Basically the racy detection is a safety mechanism that ensures we can always just compare the stat
    /// information between index and worktree and if they match we don't need to look at the content.
    /// This usually just works but if a file updates quickly we could run into the following situation:
    ///
    /// * save file version `A` from disk into worktree (git add)
    /// * file is changed so fast that the mtime doesn't change - *we only looks at seconds by default*
    /// * file contents change but file-size stays the same, so `"foo" -> "bar"` has the same size but different content
    ///
    /// Now both `mtime` and `size`, and all other stat information, is the same but the file has actually changed.
    /// This case is called *racily clean*. *The file should show up as changed but due to a data race it doesn't.*
    /// This is the racy git problem.
    ///
    /// To solve this we do the following trick: Whenever we modify the index, which includes `git status`, we save the
    /// current timestamp before the modification starts. This timestamp fundamentally represents a checkpoint of sorts.
    /// We "promise" ourselves that after the modification finishes all entries modified before this timestamp have the
    /// racy git problem resolved.
    ///
    /// So now when we modify the index we must resolve the racy git problem somehow. To do that we only need to look at
    /// unchanged entries. Changed entries are not interesting since they are already showing up as changed anyway so there
    /// isn't really a race-condition to worry about. This also explains why removing the `return` here doesn't have an apparent effect.
    /// This entire branch here is just the optimization of "don't even look at index entries where the stat hasn't changed".
    /// If we don't have this optimization the result shouldn't change, our status implementation will just be super slow :D

    /// We calculate whether this change is `racy_clean`, so if the last `timestamp` is before or the same as the `mtime` of the entry
    /// which is what `new_stat.is_racy(..)` does in the branch, and only if we are sure that there is no race condition
    /// do we `return` early. Since we don't `return` early we just do a full content comparison below,
    /// which always yields the correct result, there is no race condition there.
    ///
    /// If a file showed up as racily clean and didn't change then we don't need to do anything. After this status check is
    /// complete and the file won't show up as racily clean anymore, since it's mtime is now before the new timestamp.
    /// However if the file did actually change then we really ran into one of those rare race conditions in that case we,
    /// and git does the same, set the size of the file in the index to 0. This will always make the file show up as changed.
    /// This adds the need to treat all files of size 0 in the index as changed. This is not quite right of course because 0 sized files
    /// could be entirely valid and unchanged. Therefore this only applies if the oid doesn't match the oid of an empty file,
    /// which is a constant.
    ///
    /// Adapted from [here](https://github.com/Byron/gitoxide/pull/805#discussion_r1164676777).
    fn compute_status<T, U, Find, E1, E2>(
        &mut self,
        entry: &mut gix_index::Entry,
        rela_path: &BStr,
        diff: &mut impl CompareBlobs<Output = T>,
        submodule: &mut impl SubmoduleStatus<Output = U, Error = E2>,
        find: &mut Find,
    ) -> Result<Option<Change<T, U>>, Error>
    where
        E1: std::error::Error + Send + Sync + 'static,
        E2: std::error::Error + Send + Sync + 'static,
        Find: for<'a> FnMut(&gix_hash::oid, &'a mut Vec<u8>) -> Result<gix_object::BlobRef<'a>, E1>,
    {
        let worktree_path = gix_path::try_from_bstr(rela_path).map_err(|_| Error::IllformedUtf8)?;
        let worktree_path = match self.path_stack.verified_path(worktree_path.as_ref()) {
            Ok(path) => path,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Some(Change::Removed)),
            Err(err) => return Err(Error::Io(err)),
        };
        self.symlink_metadata_calls.fetch_add(1, Ordering::Relaxed);
        let metadata = match worktree_path.symlink_metadata() {
            Ok(metadata) if metadata.is_dir() => {
                // index entries are normally only for files/symlinks
                // if a file turned into a directory it was removed
                // the only exception here are submodules which are
                // part of the index despite being directories
                if entry.mode.is_submodule() {
                    let status = submodule
                        .status(entry, rela_path)
                        .map_err(|err| Error::SubmoduleStatus {
                            rela_path: rela_path.into(),
                            source: Box::new(err),
                        })?;
                    return Ok(status.map(|status| Change::SubmoduleModification(status)));
                } else {
                    return Ok(Some(Change::Removed));
                }
            }
            Ok(metadata) => metadata,
            Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(Some(Change::Removed)),
            Err(err) => {
                return Err(err.into());
            }
        };
        if entry.flags.contains(gix_index::entry::Flags::INTENT_TO_ADD) {
            return Ok(Some(Change::IntentToAdd));
        }
        let new_stat = gix_index::entry::Stat::from_fs(&metadata)?;
        let executable_bit_changed =
            match entry
                .mode
                .change_to_match_fs(&metadata, self.options.fs.symlink, self.options.fs.executable_bit)
            {
                Some(gix_index::entry::mode::Change::Type { .. }) => return Ok(Some(Change::Type)),
                Some(gix_index::entry::mode::Change::ExecutableBit) => true,
                None => false,
            };

        // Here we implement racy-git. See racy-git.txt in the git documentation for a detailed documentation.
        //
        // A file is racy if:
        // 1. its `mtime` is at or after the last index timestamp and its entry stat information
        //   matches the on-disk file but the file contents are actually modified
        // 2. it's size is 0 (set after detecting a file was racy previously)
        //
        // The first case is detected below by checking the timestamp if the file is marked unmodified.
        // The second case is usually detected either because the on-disk file is not empty, hence
        // the basic stat match fails, or by checking whether the size doesn't fit the oid.
        let mut racy_clean = false;
        if !executable_bit_changed
            && new_stat.matches(&entry.stat, self.options.stat)
            // TODO: find a test for the following line or remove it. Is this more often hit with smudge/clean filters?
            && (!entry.id.is_empty_blob() || entry.stat.size == 0)
        {
            racy_clean = new_stat.is_racy(self.timestamp, self.options.stat);
            if !racy_clean {
                return Ok(None);
            } else {
                self.racy_clean.fetch_add(1, Ordering::Relaxed);
            }
        }

        self.buf.clear();
        let read_file = WorktreeBlob {
            buf: &mut self.buf,
            path: worktree_path,
            entry,
            options: self.options,
        };
        self.odb_buf.clear();
        let read_blob = OdbBlob {
            buf: &mut self.odb_buf,
            id: &entry.id,
            find,
        };
        let content_change = diff.compare_blobs(entry, metadata.len() as usize, read_file, read_blob)?;
        if !self.buf.is_empty() {
            self.worktree_reads.fetch_add(1, Ordering::Relaxed);
            self.worktree_bytes.fetch_add(self.buf.len() as u64, Ordering::Relaxed);
        }
        if !self.odb_buf.is_empty() {
            self.odb_reads.fetch_add(1, Ordering::Relaxed);
            self.odb_bytes.fetch_add(self.odb_buf.len() as u64, Ordering::Relaxed);
        }
        // This file is racy clean! Set the size to 0 so we keep detecting this as the file is updated.
        if content_change.is_some() && racy_clean {
            entry.stat.size = 0;
        }
        if content_change.is_some() || executable_bit_changed {
            Ok(Some(Change::Modification {
                executable_bit_changed,
                content_change,
            }))
        } else {
            // don't diff against this file next time since we know the file is unchanged.
            entry.stat = new_stat;
            self.entries_updated.fetch_add(1, Ordering::Relaxed);
            Ok(None)
        }
    }
}

struct ReduceChange<'a, 'index, T: VisitEntry<'index>> {
    collector: &'a mut T,
    phantom: PhantomData<fn(&'index ())>,
}

impl<'index, T, U, C: VisitEntry<'index, ContentChange = T, SubmoduleStatus = U>> Reduce
    for ReduceChange<'_, 'index, C>
{
    type Input = Vec<StatusResult<'index, T, U>>;

    type FeedProduce = ();

    type Output = ();

    type Error = Error;

    fn feed(&mut self, items: Self::Input) -> Result<Self::FeedProduce, Self::Error> {
        for item in items {
            let (entry, path, change, conflict) = item?;
            self.collector.visit_entry(entry, path, change, conflict);
        }
        Ok(())
    }

    fn finalize(self) -> Result<Self::Output, Self::Error> {
        Ok(())
    }
}

struct WorktreeBlob<'a> {
    buf: &'a mut Vec<u8>,
    path: &'a Path,
    entry: &'a gix_index::Entry,
    options: &'a Options,
}

struct OdbBlob<'a, Find, E>
where
    E: std::error::Error + Send + Sync + 'static,
    Find: FnMut(&gix_hash::oid, &'a mut Vec<u8>) -> Result<gix_object::BlobRef<'a>, E>,
{
    buf: &'a mut Vec<u8>,
    id: &'a gix_hash::oid,
    find: Find,
}

impl<'a> traits::ReadDataOnce<'a> for WorktreeBlob<'a> {
    fn read_data(self) -> Result<&'a [u8], Error> {
        let res = read::data_to_buf_with_meta(
            self.path,
            self.buf,
            self.entry.mode == gix_index::entry::Mode::SYMLINK,
            &self.options.fs,
        )?;
        Ok(res)
    }
}

impl<'a, Find, E> traits::ReadDataOnce<'a> for OdbBlob<'a, Find, E>
where
    E: std::error::Error + Send + Sync + 'static,
    Find: FnMut(&gix_hash::oid, &'a mut Vec<u8>) -> Result<gix_object::BlobRef<'a>, E>,
{
    fn read_data(mut self) -> Result<&'a [u8], Error> {
        (self.find)(self.id, self.buf)
            .map(|b| b.data)
            .map_err(move |err| Error::Find(Box::new(err)))
    }
}
