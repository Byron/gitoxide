use crate::instruction;

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

/// A type keeping enough information about a ref-spec to be able to efficiently match it against multiple matcher items.
#[allow(dead_code)]
pub struct Matcher<'a> {
    /// How to interpret our lefthand-side and right-hand side ref-specs
    op: crate::parse::Operation,
    lhs: &'a bstr::BStr,
}

/// The result of a match operation.
#[derive(Default)]
#[allow(dead_code)]
pub struct Match<'a> {
    pub(crate) lhs: Option<&'a bstr::BStr>,
    pub(crate) rhs: Option<&'a bstr::BStr>,
}
