use crate::{instruction, RefSpecRef};

/// The way to interpret a refspec.
#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub(crate) enum Mode {
    /// Apply standard rules for refspecs which are including refs with specific rules related to allowing fast forwards of destinations.
    Normal,
    /// Even though according to normal rules a non-fastforward would be denied, override this and reset a ref forcefully in the destination.
    Force,
    /// Instead of considering matching refs included, we consider them excluded. This applies only to the source side of a refspec.
    Negative,
}

/// Tells what to do and is derived from a [`RefSpec`][crate::RefSpecRef].
#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum Instruction<'a> {
    /// An instruction for pushing.
    Push(instruction::Push<'a>),
    /// An instruction for fetching.
    Fetch(instruction::Fetch<'a>),
}

/// A match group is able to match a list of ref specs in order while handling negation, conflicts and one to many mappings.
pub struct MatchGroup<'a> {
    pub(crate) specs: Vec<RefSpecRef<'a>>,
}
