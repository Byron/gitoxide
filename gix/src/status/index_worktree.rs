use crate::bstr::{BStr, BString};
use crate::{config, Repository};
use gix_status::index_as_worktree::traits::{CompareBlobs, SubmoduleStatus};
use std::sync::atomic::AtomicBool;

/// The error returned by [Repository::index_worktree_status()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("A working tree is required to perform a directory walk")]
    MissingWorkDir,
    #[error(transparent)]
    AttributesAndExcludes(#[from] crate::repository::attributes::Error),
    #[error(transparent)]
    Pathspec(#[from] crate::pathspec::init::Error),
    #[error(transparent)]
    Prefix(#[from] gix_path::realpath::Error),
    #[error(transparent)]
    FilesystemOptions(#[from] config::boolean::Error),
    #[error(transparent)]
    IndexAsWorktreeWithRenames(#[from] gix_status::index_as_worktree_with_renames::Error),
    #[error(transparent)]
    StatOptions(#[from] config::stat_options::Error),
    #[error(transparent)]
    ResourceCache(#[from] crate::diff::resource_cache::Error),
}

/// Options for use with [Repository::index_worktree_status()].
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Options {
    /// The way all output should be sorted.
    ///
    /// If `None`, and depending on the `rewrites` field, output will be immediate but the output order
    /// isn't determined, and may differ between two runs. `rewrites` also depend on the order of entries that
    /// are presented to it, hence for deterministic results, sorting needs to be enabled.
    ///
    /// If `Some(_)`, all entries are collected beforehand, so they can be sorted before outputting any of them
    /// to the user.
    ///
    /// If immediate output of entries in any order is desired, this should be `None`,
    /// along with `rewrites` being `None` as well.
    pub sorting: Option<gix_status::index_as_worktree_with_renames::Sorting>,
    /// If not `None`, the options to configure the directory walk, determining how its results will look like.
    ///
    /// If `None`, only modification checks are performed.
    ///
    /// Can be instantiated with [Repository::dirwalk_options()].
    pub dirwalk_options: Option<crate::dirwalk::Options>,
    /// If `Some(_)`, along with `Some(_)` in `dirwalk_options`, rewrite tracking will be performed between the
    /// index and the working tree.
    /// Note that there is no git-configuration specific to index-worktree rename tracking.
    /// When rewrite tracking is enabled, there will be a delay for some entries as they partake in the rename-analysis.
    pub rewrites: Option<gix_diff::Rewrites>,
    /// If set, don't use more than this amount of threads for the tracked modification check.
    /// Otherwise, usually use as many threads as there are logical cores.
    /// A value of 0 is interpreted as no-limit
    pub thread_limit: Option<usize>,
}

impl Repository {
    /// Obtain the status between the index and the worktree, involving modification checks
    /// for all tracked files along with information about untracked (and posisbly ignored) files (if configured).
    ///
    /// * `index`
    ///     - The index to use for modification checks, and to know which files are tacked when applying the dirwalk.
    /// * `patterns`
    ///     - Optional patterns to use to limit the paths to look at. If empty, all paths are considered.
    /// * `delegate`
    ///     - The sink for receiving all status data.
    /// * `compare`
    ///     - The implementations for fine-grained control over what happens if a hash must be recalculated.
    /// * `submodule`
    ///      - Control what kind of information to retrieve when a submodule is encountered while traversing the index.
    /// * `progress`
    ///     - A progress indication for index modification checks.
    /// * `should_interrupt`
    ///     - A flag to stop the whole operation.
    /// * `options`
    ///     - Additional configuration for all parts of the operation.
    ///
    /// ### Note
    ///
    /// This is a lower-level method, prefer the [`status`](Repository::status()) method for greater ease of use.
    #[allow(clippy::too_many_arguments)]
    pub fn index_worktree_status<'index, T, U, E>(
        &self,
        index: &'index gix_index::State,
        patterns: impl IntoIterator<Item = impl AsRef<BStr>>,
        delegate: &mut impl gix_status::index_as_worktree_with_renames::VisitEntry<
            'index,
            ContentChange = T,
            SubmoduleStatus = U,
        >,
        compare: impl CompareBlobs<Output = T> + Send + Clone,
        submodule: impl SubmoduleStatus<Output = U, Error = E> + Send + Clone,
        progress: &mut dyn gix_features::progress::Progress,
        should_interrupt: &AtomicBool,
        options: Options,
    ) -> Result<gix_status::index_as_worktree_with_renames::Outcome, Error>
    where
        T: Send + Clone,
        U: Send + Clone,
        E: std::error::Error + Send + Sync + 'static,
    {
        let _span = gix_trace::coarse!("gix::index_worktree_status");
        let workdir = self.work_dir().ok_or(Error::MissingWorkDir)?;
        let attrs_and_excludes = self.attributes(
            index,
            crate::worktree::stack::state::attributes::Source::WorktreeThenIdMapping,
            crate::worktree::stack::state::ignore::Source::WorktreeThenIdMappingIfNotSkipped,
            None,
        )?;
        let pathspec = crate::Pathspec::new(
            self,
            options
                .dirwalk_options
                .as_ref()
                .map_or(false, |opts| opts.empty_patterns_match_prefix),
            patterns,
            true, /* inherit ignore case */
            || Ok(attrs_and_excludes.clone()),
        )?;

        let cwd = self.current_dir();
        let git_dir_realpath = crate::path::realpath_opts(self.git_dir(), cwd, crate::path::realpath::MAX_SYMLINKS)?;
        let fs_caps = self.filesystem_options()?;
        let accelerate_lookup = fs_caps.ignore_case.then(|| index.prepare_icase_backing());
        let resource_cache = crate::diff::resource_cache(
            self,
            gix_diff::blob::pipeline::Mode::ToGit,
            attrs_and_excludes.inner,
            gix_diff::blob::pipeline::WorktreeRoots {
                old_root: None,
                new_root: Some(workdir.to_owned()),
            },
        )?;

        let out = gix_status::index_as_worktree_with_renames(
            index,
            workdir,
            delegate,
            compare,
            submodule,
            self.objects.clone().into_arc().expect("arc conversion always works"),
            progress,
            gix_status::index_as_worktree_with_renames::Context {
                pathspec: pathspec.search,
                resource_cache,
                should_interrupt,
                dirwalk: gix_status::index_as_worktree_with_renames::DirwalkContext {
                    git_dir_realpath: git_dir_realpath.as_path(),
                    current_dir: cwd,
                    ignore_case_index_lookup: accelerate_lookup.as_ref(),
                },
            },
            gix_status::index_as_worktree_with_renames::Options {
                sorting: options.sorting,
                object_hash: self.object_hash(),
                tracked_file_modifications: gix_status::index_as_worktree::Options {
                    fs: fs_caps,
                    thread_limit: options.thread_limit,
                    stat: self.stat_options()?,
                },
                dirwalk: options.dirwalk_options.map(Into::into),
                rewrites: options.rewrites,
            },
        )?;
        Ok(out)
    }
}

/// An implementation of a trait to use with [`Repository::index_worktree_status()`] to compute the submodule status
/// using [Submodule::status()](crate::Submodule::status()).
#[derive(Clone)]
pub struct BuiltinSubmoduleStatus {
    mode: crate::status::Submodule,
    #[cfg(feature = "parallel")]
    repo: crate::ThreadSafeRepository,
    #[cfg(not(feature = "parallel"))]
    git_dir: std::path::PathBuf,
    submodule_paths: Vec<BString>,
}

///
#[allow(clippy::empty_docs)]
mod submodule_status {
    use crate::bstr;
    use crate::bstr::BStr;
    use crate::status::index_worktree::BuiltinSubmoduleStatus;
    use crate::status::Submodule;
    use std::borrow::Cow;

    impl BuiltinSubmoduleStatus {
        /// Create a new instance from a `repo` and a `mode` to control how the submodule status will be obtained.
        pub fn new(
            repo: crate::ThreadSafeRepository,
            mode: Submodule,
        ) -> Result<Self, crate::submodule::modules::Error> {
            let local_repo = repo.to_thread_local();
            let submodule_paths = match local_repo.submodules()? {
                Some(sm) => {
                    let mut v: Vec<_> = sm.filter_map(|sm| sm.path().ok().map(Cow::into_owned)).collect();
                    v.sort();
                    v
                }
                None => Vec::new(),
            };
            Ok(Self {
                mode,
                #[cfg(feature = "parallel")]
                repo,
                #[cfg(not(feature = "parallel"))]
                git_dir: local_repo.git_dir().to_owned(),
                submodule_paths,
            })
        }
    }

    /// The error returned submodule status checks.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        SubmoduleStatus(#[from] crate::submodule::status::Error),
        #[error(transparent)]
        IgnoreConfig(#[from] crate::submodule::config::Error),
    }

    impl gix_status::index_as_worktree::traits::SubmoduleStatus for BuiltinSubmoduleStatus {
        type Output = crate::submodule::Status;
        type Error = Error;

        fn status(&mut self, _entry: &gix_index::Entry, rela_path: &BStr) -> Result<Option<Self::Output>, Self::Error> {
            use bstr::ByteSlice;
            if self
                .submodule_paths
                .binary_search_by(|path| path.as_bstr().cmp(rela_path))
                .is_err()
            {
                return Ok(None);
            }
            #[cfg(feature = "parallel")]
            let repo = self.repo.to_thread_local();
            #[cfg(not(feature = "parallel"))]
            let Ok(repo) = crate::open(&self.git_dir) else {
                return Ok(None);
            };
            let Ok(Some(mut submodules)) = repo.submodules() else {
                return Ok(None);
            };
            let Some(sm) = submodules.find(|sm| sm.path().map_or(false, |path| path == rela_path)) else {
                return Ok(None);
            };
            let (ignore, check_dirty) = match self.mode {
                Submodule::AsConfigured { check_dirty } => (sm.ignore()?.unwrap_or_default(), check_dirty),
                Submodule::Given { ignore, check_dirty } => (ignore, check_dirty),
            };
            let status = sm.status(ignore, check_dirty)?;
            Ok(status.is_dirty().and_then(|dirty| dirty.then_some(status)))
        }
    }
}

/// An iterator for changes between the index and the worktree.
///
/// Note that depending on the underlying configuration, there might be a significant delay until the first
/// item is received due to the buffering necessary to perform rename tracking and/or sorting.
///
/// ### Submodules
///
/// Note that submodules can be set to 'inactive', which will not exclude them from the status operation, similar to
/// how `git status` includes them.
///
/// ### Index Changes
///
/// Changes to the index are collected and it's possible to write the index back using [iter::Outcome::write_changes()].
/// Note that these changes are not observable, they will always be kept.
///
/// ### Parallel Operation
///
/// Note that without the `parallel` feature, the iterator becomes 'serial', which means all status will be computed in advance
/// and it's non-interruptible, yielding worse performance for is-dirty checks for instance as interruptions won't happen.
/// It's a crutch that is just there to make single-threaded applications possible at all, as it's not really an iterator
/// anymore. If this matters, better run [Repository::index_worktree_status()] by hand as it provides all control one would need,
/// just not as an iterator.
///
/// Also, even with `parallel` set, the first call to `next()` will block until there is an item available, without a chance
/// to interrupt unless [`status::Platform::should_interrupt_*()`](crate::status::Platform::should_interrupt_shared()) was
/// configured.
pub struct Iter {
    #[cfg(feature = "parallel")]
    #[allow(clippy::type_complexity)]
    rx_and_join: Option<(
        std::sync::mpsc::Receiver<iter::Item>,
        std::thread::JoinHandle<Result<iter::Outcome, crate::status::index_worktree::Error>>,
    )>,
    #[cfg(feature = "parallel")]
    should_interrupt: crate::status::OwnedOrStaticAtomicBool,
    /// Without parallelization, the iterator has to buffer all changes in advance.
    #[cfg(not(feature = "parallel"))]
    items: std::vec::IntoIter<iter::Item>,
    /// The outcome of the operation, only available once the operation has ended.
    out: Option<iter::Outcome>,
    /// The set of `(entry_index, change)` we extracted in order to potentially write back the index with the changes applied.
    changes: Vec<(usize, iter::ApplyChange)>,
}

///
#[allow(clippy::empty_docs)]
pub mod iter {
    use crate::bstr::{BStr, BString};
    use crate::config::cache::util::ApplyLeniencyDefault;
    use crate::status::index_worktree::{iter, BuiltinSubmoduleStatus};
    use crate::status::{index_worktree, Platform};
    use crate::worktree::IndexPersistedOrInMemory;
    use gix_status::index_as_worktree::{Change, EntryStatus};

    pub use gix_status::index_as_worktree_with_renames::Summary;

    pub(super) enum ApplyChange {
        SetSizeToZero,
        NewStat(crate::index::entry::Stat),
    }

    /// The data the thread sends over to the receiving iterator.
    pub struct Outcome {
        /// The outcome of the index-to-worktree comparison operation.
        pub index_worktree: gix_status::index_as_worktree_with_renames::Outcome,
        /// The index that was used for the operation.
        pub index: crate::worktree::IndexPersistedOrInMemory,
        skip_hash: bool,
        changes: Option<Vec<(usize, iter::ApplyChange)>>,
    }

    impl Outcome {
        /// Returns `true` if the index has received currently unapplied changes that *should* be written back.
        ///
        /// If they are not written back, subsequent `status` operations will take longer to complete, whereas the
        /// additional work can be prevented by writing the changes back to the index.
        pub fn has_changes(&self) -> bool {
            self.changes.as_ref().map_or(false, |changes| !changes.is_empty())
        }

        /// Write the changes if there are any back to the index file.
        /// This can only be done once as the changes are consumed in the process, if there were any.
        pub fn write_changes(&mut self) -> Option<Result<(), gix_index::file::write::Error>> {
            let _span = gix_features::trace::coarse!("gix::status::index_worktree::iter::Outcome::write_changes()");
            let changes = self.changes.take()?;
            let mut index = match &self.index {
                IndexPersistedOrInMemory::Persisted(persisted) => (***persisted).clone(),
                IndexPersistedOrInMemory::InMemory(index) => index.clone(),
            };

            let entries = index.entries_mut();
            for (entry_index, change) in changes {
                let entry = &mut entries[entry_index];
                match change {
                    ApplyChange::SetSizeToZero => {
                        entry.stat.size = 0;
                    }
                    ApplyChange::NewStat(new_stat) => {
                        entry.stat = new_stat;
                    }
                }
            }

            Some(index.write(crate::index::write::Options {
                extensions: Default::default(),
                skip_hash: self.skip_hash,
            }))
        }
    }

    /// Either an index entry for renames or another directory entry in case of copies.
    #[derive(Clone, PartialEq, Debug)]
    pub enum RewriteSource {
        /// The source originates in the index and is detected as missing in the working tree.
        /// This can also happen for copies.
        RewriteFromIndex {
            /// The entry that is the source of the rewrite, which means it was removed on disk,
            /// equivalent to [Change::Removed].
            ///
            /// Note that the [entry-id](gix_index::Entry::id) is the content-id of the source of the rewrite.
            source_entry: gix_index::Entry,
            /// The index of the `source_entry` for lookup in [`gix_index::State::entries()`] - useful to look at neighbors.
            source_entry_index: usize,
            /// The repository-relative path of the `source_entry`.
            source_rela_path: BString,
            /// The computed status of the `source_entry`.
            source_status: gix_status::index_as_worktree::EntryStatus<(), crate::submodule::Status>,
        },
        /// This source originates in the directory tree and is always the source of copies.
        CopyFromDirectoryEntry {
            /// The source of the copy operation, which is also an entry of the directory walk.
            ///
            /// Note that its [`rela_path`](gix_dir::EntryRef::rela_path) is the source of the rewrite.
            source_dirwalk_entry: gix_dir::Entry,
            /// `collapsed_directory_status` is `Some(dir_status)` if this `source_dirwalk_entry` was part of a directory with the given
            /// `dir_status` that wasn't the same as the one of `source_dirwalk_entry` and
            /// if [gix_dir::walk::Options::emit_collapsed] was [CollapsedEntriesEmissionMode::OnStatusMismatch](gix_dir::walk::CollapsedEntriesEmissionMode::OnStatusMismatch).
            /// It will also be `Some(dir_status)` if that option was [CollapsedEntriesEmissionMode::All](gix_dir::walk::CollapsedEntriesEmissionMode::All).
            source_dirwalk_entry_collapsed_directory_status: Option<gix_dir::entry::Status>,
            /// The object id as it would appear if the entry was written to the object database.
            /// It's the same as [`dirwalk_entry_id`](Item::Rewrite), or `diff` is `Some(_)` to indicate that the copy
            /// was determined by similarity, not by content equality.
            source_dirwalk_entry_id: gix_hash::ObjectId,
        },
    }

    /// Access
    impl RewriteSource {
        /// The repository-relative path of this source.
        pub fn rela_path(&self) -> &BStr {
            match self {
                RewriteSource::RewriteFromIndex { source_rela_path, .. } => source_rela_path.as_ref(),
                RewriteSource::CopyFromDirectoryEntry {
                    source_dirwalk_entry, ..
                } => source_dirwalk_entry.rela_path.as_ref(),
            }
        }
    }

    impl<'index> From<gix_status::index_as_worktree_with_renames::RewriteSource<'index, (), SubmoduleStatus>>
        for RewriteSource
    {
        fn from(value: gix_status::index_as_worktree_with_renames::RewriteSource<'index, (), SubmoduleStatus>) -> Self {
            match value {
                gix_status::index_as_worktree_with_renames::RewriteSource::RewriteFromIndex {
                    index_entries: _,
                    source_entry,
                    source_entry_index,
                    source_rela_path,
                    source_status,
                } => RewriteSource::RewriteFromIndex {
                    source_entry: source_entry.clone(),
                    source_entry_index,
                    source_rela_path: source_rela_path.to_owned(),
                    source_status,
                },
                gix_status::index_as_worktree_with_renames::RewriteSource::CopyFromDirectoryEntry {
                    source_dirwalk_entry,
                    source_dirwalk_entry_collapsed_directory_status,
                    source_dirwalk_entry_id,
                } => RewriteSource::CopyFromDirectoryEntry {
                    source_dirwalk_entry,
                    source_dirwalk_entry_collapsed_directory_status,
                    source_dirwalk_entry_id,
                },
            }
        }
    }

    /// The item produced by the iterator
    #[derive(Clone, PartialEq, Debug)]
    pub enum Item {
        /// A tracked file was modified, and index-specific information is passed.
        Modification {
            /// The entry with modifications.
            entry: gix_index::Entry,
            /// The index of the `entry` for lookup in [`gix_index::State::entries()`] - useful to look at neighbors.
            entry_index: usize,
            /// The repository-relative path of the entry.
            rela_path: BString,
            /// The computed status of the entry.
            status: gix_status::index_as_worktree::EntryStatus<(), SubmoduleStatus>,
        },
        /// An entry returned by the directory walk, without any relation to the index.
        ///
        /// This can happen if ignored files are returned as well, or if rename-tracking is disabled.
        DirectoryContents {
            /// The entry found during the disk traversal.
            entry: gix_dir::Entry,
            /// `collapsed_directory_status` is `Some(dir_status)` if this `entry` was part of a directory with the given
            /// `dir_status` that wasn't the same as the one of `entry` and if [gix_dir::walk::Options::emit_collapsed] was
            /// [CollapsedEntriesEmissionMode::OnStatusMismatch](gix_dir::walk::CollapsedEntriesEmissionMode::OnStatusMismatch).
            /// It will also be `Some(dir_status)` if that option was [CollapsedEntriesEmissionMode::All](gix_dir::walk::CollapsedEntriesEmissionMode::All).
            collapsed_directory_status: Option<gix_dir::entry::Status>,
        },
        /// The rewrite tracking discovered a match between a deleted and added file, and considers them equal enough,
        /// depending on the tracker settings.
        ///
        /// Note that the source of the rewrite is always the index as it detects the absence of entries, something that
        /// can't be done during a directory walk.
        Rewrite {
            /// The source of the rewrite operation.
            source: RewriteSource,
            /// The untracked entry found during the disk traversal, the destination of the rewrite.
            ///
            /// Note that its [`rela_path`](gix_dir::EntryRef::rela_path) is the destination of the rewrite, and the current
            /// location of the entry.
            dirwalk_entry: gix_dir::Entry,
            /// `collapsed_directory_status` is `Some(dir_status)` if this `dirwalk_entry` was part of a directory with the given
            /// `dir_status` that wasn't the same as the one of `dirwalk_entry` and if [gix_dir::walk::Options::emit_collapsed] was
            /// [CollapsedEntriesEmissionMode::OnStatusMismatch](gix_dir::walk::CollapsedEntriesEmissionMode::OnStatusMismatch).
            /// It will also be `Some(dir_status)` if that option was [CollapsedEntriesEmissionMode::All](gix_dir::walk::CollapsedEntriesEmissionMode::All).
            dirwalk_entry_collapsed_directory_status: Option<gix_dir::entry::Status>,
            /// The object id after the rename, specifically hashed in order to determine equality.
            dirwalk_entry_id: gix_hash::ObjectId,
            /// It's `None` if the 'source.id' is equal to `dirwalk_entry_id`, as identity made an actual diff computation unnecessary.
            /// Otherwise, and if enabled, it's `Some(stats)` to indicate how similar both entries were.
            diff: Option<gix_diff::blob::DiffLineStats>,
            /// If true, this rewrite is created by copy, and 'source.id' is pointing to its source.
            /// Otherwise, it's a rename, and 'source.id' points to a deleted object,
            /// as renames are tracked as deletions and additions of the same or similar content.
            copy: bool,
        },
    }

    impl Item {
        /// Return a simplified summary of the item as digest of its status, or `None` if this item is
        /// created from the directory walk and is *not untracked*, or if it is merely to communicate
        /// a needed update to the index entry.
        pub fn summary(&self) -> Option<Summary> {
            use gix_status::index_as_worktree_with_renames::Summary::*;
            Some(match self {
                Item::Modification { status, .. } => match status {
                    EntryStatus::Conflict(_) => Conflict,
                    EntryStatus::Change(change) => match change {
                        Change::Removed => Removed,
                        Change::Type => TypeChange,
                        Change::Modification { .. } | Change::SubmoduleModification(_) => Modified,
                    },
                    EntryStatus::NeedsUpdate(_) => return None,
                    EntryStatus::IntentToAdd => IntentToAdd,
                },
                Item::DirectoryContents { entry, .. } => {
                    if matches!(entry.status, gix_dir::entry::Status::Untracked) {
                        Added
                    } else {
                        return None;
                    }
                }
                Item::Rewrite { copy, .. } => {
                    if *copy {
                        Copied
                    } else {
                        Renamed
                    }
                }
            })
        }
    }

    impl<'index> From<gix_status::index_as_worktree_with_renames::Entry<'index, (), SubmoduleStatus>> for Item {
        fn from(value: gix_status::index_as_worktree_with_renames::Entry<'index, (), SubmoduleStatus>) -> Self {
            match value {
                gix_status::index_as_worktree_with_renames::Entry::Modification {
                    entries: _,
                    entry,
                    entry_index,
                    rela_path,
                    status,
                } => Item::Modification {
                    entry: entry.clone(),
                    entry_index,
                    rela_path: rela_path.to_owned(),
                    status,
                },
                gix_status::index_as_worktree_with_renames::Entry::DirectoryContents {
                    entry,
                    collapsed_directory_status,
                } => Item::DirectoryContents {
                    entry,
                    collapsed_directory_status,
                },
                gix_status::index_as_worktree_with_renames::Entry::Rewrite {
                    source,
                    dirwalk_entry,
                    dirwalk_entry_collapsed_directory_status,
                    dirwalk_entry_id,
                    diff,
                    copy,
                } => Item::Rewrite {
                    source: source.into(),
                    dirwalk_entry,
                    dirwalk_entry_collapsed_directory_status,
                    dirwalk_entry_id,
                    diff,
                    copy,
                },
            }
        }
    }

    type SubmoduleStatus = crate::submodule::Status;

    /// The error returned by [Platform::into_index_worktree_iter()](crate::status::Platform::into_index_worktree_iter()).
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Index(#[from] crate::worktree::open_index::Error),
        #[error("Failed to spawn producer thread")]
        #[cfg(feature = "parallel")]
        SpawnThread(#[source] std::io::Error),
        #[error(transparent)]
        #[cfg(not(feature = "parallel"))]
        IndexWorktreeStatus(#[from] crate::status::index_worktree::Error),
        #[error(transparent)]
        ConfigSkipHash(#[from] crate::config::boolean::Error),
        #[error(transparent)]
        PrepareSubmodules(#[from] crate::submodule::modules::Error),
    }

    /// Lifecycle
    impl<'repo, Progress> Platform<'repo, Progress>
    where
        Progress: gix_features::progress::Progress,
    {
        /// Turn the platform into an iterator for changes between the index and the working tree.
        ///
        /// * `patterns`
        ///     - Optional patterns to use to limit the paths to look at. If empty, all paths are considered.
        #[doc(alias = "diff_index_to_workdir", alias = "git2")]
        pub fn into_index_worktree_iter(
            self,
            patterns: impl IntoIterator<Item = BString>,
        ) -> Result<index_worktree::Iter, Error> {
            let index = match self.index {
                None => IndexPersistedOrInMemory::Persisted(self.repo.index_or_empty()?),
                Some(index) => index,
            };

            let skip_hash = self
                .repo
                .config
                .resolved
                .boolean("index", None, "skipHash")
                .map(|res| crate::config::tree::Index::SKIP_HASH.enrich_error(res))
                .transpose()
                .with_lenient_default(self.repo.config.lenient_config)?
                .unwrap_or_default();
            let should_interrupt = self.should_interrupt.clone().unwrap_or_default();
            let submodule = BuiltinSubmoduleStatus::new(self.repo.clone().into_sync(), self.submodules)?;
            #[cfg(feature = "parallel")]
            {
                let (tx, rx) = std::sync::mpsc::channel();
                let mut collect = Collect { tx };
                let patterns: Vec<_> = patterns.into_iter().collect();
                let join = std::thread::Builder::new()
                    .name("gix::status::index_worktree::iter::producer".into())
                    .spawn({
                        let repo = self.repo.clone().into_sync();
                        let options = self.index_worktree_options;
                        let should_interrupt = should_interrupt.clone();
                        let mut progress = self.progress;
                        move || -> Result<_, crate::status::index_worktree::Error> {
                            let repo = repo.to_thread_local();
                            let out = repo.index_worktree_status(
                                &index,
                                patterns,
                                &mut collect,
                                gix_status::index_as_worktree::traits::FastEq,
                                submodule,
                                &mut progress,
                                &should_interrupt,
                                options,
                            )?;
                            Ok(Outcome {
                                index_worktree: out,
                                index,
                                changes: None,
                                skip_hash,
                            })
                        }
                    })
                    .map_err(Error::SpawnThread)?;

                Ok(super::Iter {
                    rx_and_join: Some((rx, join)),
                    should_interrupt,
                    changes: Vec::new(),
                    out: None,
                })
            }
            #[cfg(not(feature = "parallel"))]
            {
                let mut collect = Collect { items: Vec::new() };

                let repo = self.repo.clone().into_sync();
                let options = self.index_worktree_options;
                let mut progress = self.progress;
                let repo = repo.to_thread_local();
                let out = repo.index_worktree_status(
                    &index,
                    patterns,
                    &mut collect,
                    gix_status::index_as_worktree::traits::FastEq,
                    submodule,
                    &mut progress,
                    &should_interrupt,
                    options,
                )?;
                let mut out = Outcome {
                    index_worktree: out,
                    index,
                    changes: None,
                    skip_hash,
                };
                let mut iter = super::Iter {
                    items: Vec::new().into_iter(),
                    changes: Vec::new(),
                    out: None,
                };
                let items = collect
                    .items
                    .into_iter()
                    .filter_map(|item| iter.maybe_keep_index_change(item))
                    .collect::<Vec<_>>();
                out.changes = (!iter.changes.is_empty()).then(|| std::mem::take(&mut iter.changes));
                iter.items = items.into_iter();
                iter.out = Some(out);
                Ok(iter)
            }
        }
    }

    impl Iterator for super::Iter {
        type Item = Result<Item, index_worktree::Error>;

        fn next(&mut self) -> Option<Self::Item> {
            #[cfg(feature = "parallel")]
            loop {
                let (rx, _join) = self.rx_and_join.as_ref()?;
                match rx.recv().ok() {
                    Some(item) => {
                        if let Some(item) = self.maybe_keep_index_change(item) {
                            break Some(Ok(item));
                        }
                        continue;
                    }
                    None => {
                        let (_rx, handle) = self.rx_and_join.take()?;
                        break match handle.join().expect("no panic") {
                            Ok(mut out) => {
                                out.changes = Some(std::mem::take(&mut self.changes));
                                self.out = Some(out);
                                None
                            }
                            Err(err) => Some(Err(err)),
                        };
                    }
                }
            }
            #[cfg(not(feature = "parallel"))]
            self.items.next().map(Ok)
        }
    }

    /// Access
    impl super::Iter {
        /// Return the outcome of the iteration, or `None` if the iterator isn't fully consumed.
        pub fn outcome_mut(&mut self) -> Option<&mut Outcome> {
            self.out.as_mut()
        }

        /// Turn the iterator into the iteration outcome, which is `None` on error or if the iteration
        /// isn't complete.
        pub fn into_outcome(mut self) -> Option<Outcome> {
            self.out.take()
        }
    }

    impl super::Iter {
        fn maybe_keep_index_change(&mut self, item: Item) -> Option<Item> {
            let change = match item {
                Item::Modification {
                    status: gix_status::index_as_worktree::EntryStatus::NeedsUpdate(stat),
                    entry_index,
                    ..
                } => (entry_index, ApplyChange::NewStat(stat)),
                Item::Modification {
                    status:
                        gix_status::index_as_worktree::EntryStatus::Change(
                            gix_status::index_as_worktree::Change::Modification {
                                set_entry_stat_size_zero,
                                ..
                            },
                        ),
                    entry_index,
                    ..
                } if set_entry_stat_size_zero => (entry_index, ApplyChange::SetSizeToZero),
                _ => return Some(item),
            };

            self.changes.push(change);
            None
        }
    }

    #[cfg(feature = "parallel")]
    impl Drop for super::Iter {
        fn drop(&mut self) {
            crate::util::parallel_iter_drop(self.rx_and_join.take(), &self.should_interrupt);
        }
    }

    struct Collect {
        #[cfg(feature = "parallel")]
        tx: std::sync::mpsc::Sender<Item>,
        #[cfg(not(feature = "parallel"))]
        items: Vec<Item>,
    }

    impl<'index> gix_status::index_as_worktree_with_renames::VisitEntry<'index> for Collect {
        type ContentChange = <gix_status::index_as_worktree::traits::FastEq as gix_status::index_as_worktree::traits::CompareBlobs>::Output;
        type SubmoduleStatus =
            <BuiltinSubmoduleStatus as gix_status::index_as_worktree::traits::SubmoduleStatus>::Output;

        fn visit_entry(
            &mut self,
            entry: gix_status::index_as_worktree_with_renames::Entry<
                'index,
                Self::ContentChange,
                Self::SubmoduleStatus,
            >,
        ) {
            // NOTE: we assume that the receiver triggers interruption so the operation will stop if the receiver is down.
            #[cfg(feature = "parallel")]
            self.tx.send(entry.into()).ok();
            #[cfg(not(feature = "parallel"))]
            self.items.push(entry.into());
        }
    }
}
