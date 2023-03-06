/// Tell what operation is currently in progress.
#[derive(Debug, PartialEq, Eq)]
pub enum InProgress {
    /// A mailbox is being applied.
    ApplyMailbox,
    /// A rebase is happening while a mailbox is being applied.
    // TODO: test
    ApplyMailboxRebase,
    /// A git bisect operation has not yet been concluded.
    Bisect,
    /// A cherry pick operation.
    CherryPick,
    /// A cherry pick with multiple commits pending.
    CherryPickSequence,
    /// A merge operation.
    Merge,
    /// A rebase operation.
    Rebase,
    /// An interactive rebase operation.
    RebaseInteractive,
    /// A revert operation.
    Revert,
    /// A revert operation with multiple commits pending.
    RevertSequence,
}
