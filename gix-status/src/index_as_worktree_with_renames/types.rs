use crate::index_as_worktree::EntryStatus;
use bstr::{BStr, ByteSlice};
use std::sync::atomic::AtomicBool;

/// The error returned by [index_as_worktree_with_renames()`](crate::index_as_worktree_with_renames()).
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    TrackedFileModifications(#[from] crate::index_as_worktree::Error),
    #[error(transparent)]
    DirWalk(gix_dir::walk::Error),
    #[error(transparent)]
    SpawnThread(std::io::Error),
    #[error("Failed to change the context for querying gitattributes to the respective path")]
    SetAttributeContext(std::io::Error),
    #[error("Could not open worktree file for reading")]
    OpenWorktreeFile(std::io::Error),
    #[error(transparent)]
    HashFile(std::io::Error),
    #[error("Could not read worktree link content")]
    ReadLink(std::io::Error),
    #[error(transparent)]
    ConvertToGit(#[from] gix_filter::pipeline::convert::to_git::Error),
    #[error(transparent)]
    RewriteTracker(#[from] gix_diff::rewrites::tracker::emit::Error),
}

/// The way all output should be sorted.
#[derive(Clone, Copy, Default, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Sorting {
    /// The entries are sorted by their path in a case-sensitive fashion.
    #[default]
    ByPathCaseSensitive,
}

/// Provide additional information collected during the runtime of [`index_as_worktree_with_renames()`](crate::index_as_worktree_with_renames()).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Outcome {
    /// The outcome of the modification check of tracked files.
    pub tracked_file_modification: crate::index_as_worktree::Outcome,
    /// The outcome of the directory walk, or `None` if its [options](Options::dirwalk) also weren't present which means
    /// the dirwalk never ran.
    pub dirwalk: Option<gix_dir::walk::Outcome>,
    /// The result of the rewrite operation, if [rewrites were configured](Options::rewrites).
    pub rewrites: Option<gix_diff::rewrites::Outcome>,
}

/// Either an index entry for renames or another directory entry in case of copies.
#[derive(Clone, PartialEq, Debug)]
pub enum RewriteSource<'index, ContentChange, SubmoduleStatus> {
    /// The source originates in the index and is detected as missing in the working tree.
    /// This can also happen for copies.
    RewriteFromIndex {
        /// All entries in the index.
        index_entries: &'index [gix_index::Entry],
        /// The entry that is the source of the rewrite, which means it was removed on disk,
        /// equivalent to [Change::Removed](crate::index_as_worktree::Change::Removed).
        ///
        /// Note that the [entry-id](gix_index::Entry::id) is the content-id of the source of the rewrite.
        source_entry: &'index gix_index::Entry,
        /// The index of the `source_entry` for lookup in `index_entries` - useful to look at neighbors.
        source_entry_index: usize,
        /// The repository-relative path of the `source_entry`.
        source_rela_path: &'index BStr,
        /// The computed status of the `source_entry`.
        source_status: EntryStatus<ContentChange, SubmoduleStatus>,
    },
    /// This source originates in the directory tree and is always the source of copies.
    CopyFromDirectoryEntry {
        /// The source of the copy operation, which is also an entry of the directory walk.
        ///
        /// Note that its [`rela_path`](gix_dir::EntryRef::rela_path) is the source of the rewrite.
        source_dirwalk_entry: gix_dir::Entry,
        /// `collapsed_directory_status` is `Some(dir_status)` if this `source_dirwalk_entry` was part of a directory with the given
        /// `dir_status` that wasn't the same as the one of `source_dirwalk_entry` and if [gix_dir::walk::Options::emit_collapsed] was
        /// [CollapsedEntriesEmissionMode::OnStatusMismatch](gix_dir::walk::CollapsedEntriesEmissionMode::OnStatusMismatch).
        /// It will also be `Some(dir_status)` if that option was [CollapsedEntriesEmissionMode::All](gix_dir::walk::CollapsedEntriesEmissionMode::All).
        source_dirwalk_entry_collapsed_directory_status: Option<gix_dir::entry::Status>,
        /// The object id as it would appear if the entry was written to the object database.
        /// It's the same as `dirwalk_entry_id`, or `diff` is `Some(_)` to indicate that the copy was determined by similarity.
        source_dirwalk_entry_id: gix_hash::ObjectId,
    },
}

/// An 'entry' in the sense of a merge of modified tracked files and results from a directory walk.
#[derive(Clone, PartialEq, Debug)]
pub enum Entry<'index, ContentChange, SubmoduleStatus> {
    /// A tracked file was modified, and index-specific information is passed.
    Modification {
        /// All entries in the index.
        entries: &'index [gix_index::Entry],
        /// The entry with modifications.
        entry: &'index gix_index::Entry,
        /// The index of the `entry` for lookup in `entries` - useful to look at neighbors.
        entry_index: usize,
        /// The repository-relative path of the entry.
        rela_path: &'index BStr,
        /// The computed status of the entry.
        status: EntryStatus<ContentChange, SubmoduleStatus>,
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
        source: RewriteSource<'index, ContentChange, SubmoduleStatus>,
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

/// Access
impl<ContentChange, SubmoduleStatus> RewriteSource<'_, ContentChange, SubmoduleStatus> {
    /// The repository-relative path of this source.
    pub fn rela_path(&self) -> &BStr {
        match self {
            RewriteSource::RewriteFromIndex { source_rela_path, .. } => source_rela_path,
            RewriteSource::CopyFromDirectoryEntry {
                source_dirwalk_entry, ..
            } => source_dirwalk_entry.rela_path.as_bstr(),
        }
    }
}

/// Access
impl<ContentChange, SubmoduleStatus> Entry<'_, ContentChange, SubmoduleStatus> {
    /// The repository-relative path at which the source of a rewrite is located.
    ///
    /// If this isn't a rewrite, the path is the location of the entry itself.
    pub fn source_rela_path(&self) -> &BStr {
        match self {
            Entry::Modification { rela_path, .. } => rela_path,
            Entry::DirectoryContents { entry, .. } => entry.rela_path.as_bstr(),
            Entry::Rewrite { source, .. } => source.rela_path(),
        }
    }

    /// The repository-relative path at which the destination of a rewrite is located.
    ///
    /// If this isn't a rewrite, the path is the location of the entry itself.
    pub fn destination_rela_path(&self) -> &BStr {
        match self {
            Entry::Modification { rela_path, .. } => rela_path,
            Entry::DirectoryContents { entry, .. } => entry.rela_path.as_bstr(),
            Entry::Rewrite { dirwalk_entry, .. } => dirwalk_entry.rela_path.as_bstr(),
        }
    }
}

/// Options for use in [index_as_worktree_with_renames()](crate::index_as_worktree_with_renames()).
#[derive(Clone, Default)]
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
    pub sorting: Option<Sorting>,
    /// The kind of hash to create when hashing worktree entries.
    pub object_hash: gix_hash::Kind,
    /// Options to configure how modifications to tracked files should be obtained.
    pub tracked_file_modifications: crate::index_as_worktree::Options,
    /// Options to control the directory walk that informs about untracked files.
    ///
    /// Note that we forcefully disable emission of tracked files to avoid any overlap
    /// between emissions to indicate modifications, and those that are obtained by
    /// the directory walk.
    ///
    /// If `None`, the directory walk portion will not run at all, yielding data similar
    /// to a bare [index_as_worktree()](crate::index_as_worktree()) call.
    pub dirwalk: Option<gix_dir::walk::Options>,
    /// The configuration for the rewrite tracking. Note that if set, the [`dirwalk`](Self::dirwalk) should be configured
    /// to *not* collapse untracked and ignored entries, as rewrite tracking is on a file-by-file basis.
    /// Also note that when `Some(_)`, it will collect certain changes depending on the exact configuration, which typically increases
    /// the latency until the first entries are received. Note that some entries are never candidates for renames, which means
    /// they are forwarded to the caller right away.
    ///
    /// If `None`, no tracking will occour, which means that all output becomes visible to the delegate immediately.
    pub rewrites: Option<gix_diff::Rewrites>,
}

/// The context for [index_as_worktree_with_renames()`](crate::index_as_worktree_with_renames()).
pub struct Context<'a> {
    /// The pathspec to limit the amount of paths that are checked. Can be empty to allow all paths.
    ///
    /// Note that these are expected to have a [commont_prefix()](gix_pathspec::Search::common_prefix()) according
    /// to the prefix of the repository to efficiently limit the scope of the paths we process, both for the
    /// index modifications as well as for the directory walk.
    pub pathspec: gix_pathspec::Search,
    /// A fully-configured platform capable of producing diffable buffers similar to what Git would do, for use
    /// with rewrite tracking.
    ///
    /// Note that it contains resources that are additionally used here:
    ///
    /// * `attr_stack`
    ///     - A stack pre-configured to allow accessing attributes for each entry, as required for `filter`
    ///      and possibly pathspecs.
    ///      It *may* also allow accessing `.gitignore` information for use in the directory walk.
    ///      If no excludes information is present, the directory walk will identify ignored files as untracked, which
    ///      might be desirable under certain circumstances.
    /// * `filter`
    ///     - A filter to be able to perform conversions from and to the worktree format.
    ///       It is needed to potentially refresh the index with data read from the worktree, which needs to be converted back
    ///       to the form stored in Git.
    pub resource_cache: gix_diff::blob::Platform,
    /// A flag to query to learn if cancellation is requested.
    pub should_interrupt: &'a AtomicBool,
    /// The context for the directory walk.
    pub dirwalk: DirwalkContext<'a>,
}

/// All information that is required to perform a [dirwalk](gix_dir::walk()).
pub struct DirwalkContext<'a> {
    /// The `git_dir` of the parent repository, after a call to [`gix_path::realpath()`].
    ///
    /// It's used to help us differentiate our own `.git` directory from nested unrelated repositories,
    /// which is needed if `core.worktree` is used to nest the `.git` directory deeper within.
    pub git_dir_realpath: &'a std::path::Path,
    /// The current working directory as returned by `gix_fs::current_dir()` to assure it respects `core.precomposeUnicode`.
    /// It's used to produce the realpath of the git-dir of a repository candidate to assure it's not our own repository.
    pub current_dir: &'a std::path::Path,
    /// A utility to lookup index entries faster, and deal with ignore-case handling.
    ///
    /// Must be set if [`ignore_case`](gix_dir::walk::Options::ignore_case) is `true`, or else some entries won't be found if their case is different.
    ///
    /// [Read more in `gix-dir`](gix_dir::walk::Context::ignore_case_index_lookup).
    pub ignore_case_index_lookup: Option<&'a gix_index::AccelerateLookup<'a>>,
}

/// Observe the status of an entry by comparing an index entry to the worktree, along
/// with potential directory walk results.
pub trait VisitEntry<'a> {
    /// Data generated by comparing an entry with a file.
    type ContentChange;
    /// Data obtained when checking the submodule status.
    type SubmoduleStatus;
    /// Observe the `status` of `entry` at the repository-relative `rela_path` at `entry_index`
    /// (for accessing `entry` and surrounding in the complete list of `entries`).
    fn visit_entry(&mut self, entry: Entry<'a, Self::ContentChange, Self::SubmoduleStatus>);
}
