///
pub mod worktree;

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
pub struct Modification {
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
