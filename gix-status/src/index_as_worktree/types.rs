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
    /// The amount of entries whose stats have been updated as its modification couldn't be determined without an expensive calculation.
    ///
    /// With these updates, this calculation will be avoided next time the status runs.
    pub entries_updated: usize,
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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
    /// An index entry that correspond to an untracked worktree file marked with `git add --intent-to-add`.
    ///
    /// This means it's not available in the object database yet
    /// even though now an entry exists that represents the worktree file.
    IntentToAdd,
}

/// Observe changes by comparing an index entry to the worktree or another index.
pub trait VisitEntry<'index> {
    /// Data generated by comparing an entry with a file.
    type ContentChange;
    /// Data obtained when checking the submodule status.
    type SubmoduleStatus;
    /// Observe the `change` of `entry` at the repository-relative `rela_path`, indicating whether
    /// or not it has a `conflict`.
    /// If `change` is `None`, there is no change.
    fn visit_entry(
        &mut self,
        entry: &'index gix_index::Entry,
        rela_path: &'index BStr,
        change: Option<Change<Self::ContentChange, Self::SubmoduleStatus>>,
        conflict: bool,
    );
}
