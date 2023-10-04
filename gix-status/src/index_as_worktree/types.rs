use bstr::{BStr, BString};

/// The error returned by [index_as_worktree()`](crate::index_as_worktree()).
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Could not convert path to UTF8")]
    IllformedUtf8,
    #[error("The clock was off when reading file related metadata after updating a file on disk")]
    Time(#[from] std::time::SystemTimeError),
    #[error("IO error while writing blob or reading file metadata or changing filetype")]
    Io(#[from] std::io::Error),
    #[error("Failed to obtain blob from object database")]
    Find(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("Could not determine status for submodule at '{rela_path}'")]
    SubmoduleStatus {
        rela_path: BString,
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
}

/// Options that control how the index status with a worktree is computed.
#[derive(Clone, Default)]
pub struct Options {
    /// Capabilities of the file system which affect the status computation.
    pub fs: gix_fs::Capabilities,
    /// If set, don't use more than this amount of threads.
    /// Otherwise, usually use as many threads as there are logical cores.
    /// A value of 0 is interpreted as no-limit
    pub thread_limit: Option<usize>,
    /// Options that control how stat comparisons are made when checking if a file is fresh.
    pub stat: gix_index::entry::stat::Options,
    /// Pre-configured state to allow processing attributes.
    ///
    /// These are needed to potentially refresh the index with data read from the worktree, which needs to be converted back
    /// to the form stored in git.
    pub attributes: gix_worktree::stack::state::Attributes,
}

/// Provide additional information collected during the runtime of [`index_as_worktree()`](crate::index_as_worktree()).
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Outcome {
    /// The total amount of entries that is to be processed.
    pub entries_to_process: usize,
    /// The amount of entries we actually processed. If this isn't the entire set, the operation was interrupted.
    pub entries_processed: usize,
    /// The amount of entries we didn't even traverse (and thus update with stat) due to a common prefix in pathspecs.
    /// This is similar to the current working directory.
    pub entries_skipped_by_common_prefix: usize,
    /// The amount of entries that were skipped due to exclusion by *pathspecs*.
    pub entries_skipped_by_pathspec: usize,
    /// The amount of entries that were skipped as the entry flag indicated this.
    pub entries_skipped_by_entry_flags: usize,
    /// The amount of times we queried symlink-metadata for a file on disk.
    pub symlink_metadata_calls: usize,
    /// The amount of entries whose stats would need to be updated as its modification couldn't be determined without
    /// an expensive calculation.
    ///
    /// With these updates, this calculation will be avoided next time the status runs.
    /// Note that the stat updates are delegated to the caller.
    pub entries_to_update: usize,
    /// The amount of entries that were considered racy-clean - they will need thorough checking to see if they are truly clean,
    /// i.e. didn't change.
    pub racy_clean: usize,

    /// The amount of bytes read from the worktree in order to determine if an entry changed, across all files.
    pub worktree_bytes: u64,
    /// The amount of files read in full from the worktree (and into memory).
    pub worktree_files_read: usize,
    /// The amount of bytes read from the object database in order to determine if an entry changed, across all objects.
    pub odb_bytes: u64,
    /// The amount of objects read from the object database.
    pub odb_objects_read: usize,
}

impl Outcome {
    /// The total amount of skipped entries, i.e. those that weren't processed at all.
    pub fn skipped(&self) -> usize {
        self.entries_skipped_by_common_prefix + self.entries_skipped_by_pathspec + self.entries_skipped_by_entry_flags
    }
}

/// How an index entry needs to be changed to obtain the destination worktree state, i.e. `entry.apply(this_change) == worktree-entry`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Change<T = (), U = ()> {
    /// This corresponding file does not exist in the worktree anymore.
    Removed,
    /// The type of file changed compared to the worktree, i.e. a symlink s now a file.
    Type,
    /// This worktree file was modified in some form, like a permission change or content change or both,
    /// as compared to this entry.
    Modification {
        /// Indicates that one of the stat changes was an executable bit change
        /// which is a significant change itself.
        executable_bit_changed: bool,
        /// The output of the [`CompareBlobs`](crate::index_as_worktree::traits::CompareBlobs) run on this entry.
        /// If there is no content change and only the executable bit
        /// changed then this is `None`.
        content_change: Option<T>,
        /// If true, the caller is expected to set [entry.stat.size = 0](gix_index::entry::Stat::size) to assure this
        /// otherwise racily clean entry can still be detected as dirty next time this is called, but this time without
        /// reading it from disk to hash it. It's a performance optimization and not doing so won't change the correctness
        /// of the operation.
        set_entry_stat_size_zero: bool,
    },
    /// A submodule is initialized and checked out, and there was modification to either:
    ///
    /// * the `HEAD` as compared to the superproject's desired commit for `HEAD`
    /// * the worktree has at least one modified file
    /// * there is at least one untracked file
    ///
    /// The exact nature of the modification is handled by the caller which may retain information per submodule or
    /// re-compute details as needed when seeing this variant.
    SubmoduleModification(U),
}

/// Information about an entry.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum EntryStatus<T = (), U = ()> {
    /// The entry is in a conflicting state, and we didn't collect any more information about it.
    Conflict(Conflict),
    /// There is no conflict and a change was discovered.
    Change(Change<T, U>),
    /// The entry didn't change, but its state caused extra work that can be avoided next time if its stats would be updated to the
    /// given stat.
    NeedsUpdate(
        /// The new stats which represent what's currently in the working tree. If these replace the current stats in the entry,
        /// next time this operation runs we can determine the actual state much faster.
        gix_index::entry::Stat,
    ),
    /// An index entry that corresponds to an untracked worktree file marked with `git add --intent-to-add`.
    ///
    /// This means it's not available in the object database yet even though now an entry exists that represents the worktree file.
    /// The entry represents the promise of adding a new file, no matter the actual stat or content.
    /// Effectively this means nothing changed.
    /// This also means the file is still present, and that no detailed change checks were performed.
    IntentToAdd,
}

impl<T, U> From<Change<T, U>> for EntryStatus<T, U> {
    fn from(value: Change<T, U>) -> Self {
        EntryStatus::Change(value)
    }
}

/// Describes a conflicting entry as comparison between 'our' version and 'their' version of it.
///
/// If one side isn't specified, it is assumed to have modified the entry. In general, there would be no conflict
/// if both parties ended up in the same state.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Conflict {
    /// Both deleted a different version of the entry.
    BothDeleted,
    /// We added, they modified, ending up in different states.
    AddedByUs,
    /// They deleted the entry, we modified it.
    DeletedByThem,
    /// They added the entry, we modified it, ending up in different states.
    AddedByThem,
    /// We deleted the entry, they modified it, ending up in different states.
    DeletedByUs,
    /// Both added the entry in different states.
    BothAdded,
    /// Both modified the entry, ending up in different states.
    BothModified,
}

/// Observe the status of an entry by comparing an index entry to the worktree.
pub trait VisitEntry<'index> {
    /// Data generated by comparing an entry with a file.
    type ContentChange;
    /// Data obtained when checking the submodule status.
    type SubmoduleStatus;
    /// Observe the `status` of `entry` at the repository-relative `rela_path` at `entry_index`
    /// (for accessing `entry` and surrounding in the complete list of `entries`).
    fn visit_entry(
        &mut self,
        entries: &'index [gix_index::Entry],
        entry: &'index gix_index::Entry,
        entry_index: usize,
        rela_path: &'index BStr,
        status: EntryStatus<Self::ContentChange, Self::SubmoduleStatus>,
    );
}
