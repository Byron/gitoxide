#![allow(missing_docs)]
#[cfg(feature = "git-diff")]
use std::borrow::BorrowMut;

#[cfg(feature = "git-diff")]
use git_hash::oid;
use git_object::{immutable, tree};
#[cfg(feature = "git-traverse")]
use git_traverse::tree::breadthfirst;

pub trait Sealed {}

pub trait TreeIterExt: Sealed {
    #[cfg(feature = "git-diff")]
    fn changes_needed<FindFn, R, StateMut>(
        &self,
        other: tree::RefIter<'_>,
        state: StateMut,
        find: FindFn,
        delegate: &mut R,
    ) -> Result<(), git_diff::tree::changes::Error>
    where
        FindFn: for<'b> FnMut(&oid, &'b mut Vec<u8>) -> Option<immutable::tree::RefIter<'b>>,
        R: git_diff::tree::Visit,
        StateMut: BorrowMut<git_diff::tree::State>;

    /// Use this for squeezing out the last bits of performance.
    #[cfg(feature = "git-traverse")]
    fn traverse<StateMut, Find, V>(
        &self,
        state: StateMut,
        find: Find,
        delegate: &mut V,
    ) -> Result<(), breadthfirst::Error>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<tree::RefIter<'a>>,
        StateMut: BorrowMut<breadthfirst::State>,
        V: git_traverse::tree::Visit;
}

impl<'d> Sealed for tree::RefIter<'d> {}

impl<'d> TreeIterExt for tree::RefIter<'d> {
    #[cfg(feature = "git-diff")]
    fn changes_needed<FindFn, R, StateMut>(
        &self,
        other: tree::RefIter<'_>,
        state: StateMut,
        find: FindFn,
        delegate: &mut R,
    ) -> Result<(), git_diff::tree::changes::Error>
    where
        FindFn: for<'b> FnMut(&oid, &'b mut Vec<u8>) -> Option<immutable::tree::RefIter<'b>>,
        R: git_diff::tree::Visit,
        StateMut: BorrowMut<git_diff::tree::State>,
    {
        git_diff::tree::Changes::from(Some(self.clone())).needed_to_obtain(other, state, find, delegate)
    }

    #[cfg(feature = "git-traverse")]
    fn traverse<StateMut, Find, V>(
        &self,
        state: StateMut,
        find: Find,
        delegate: &mut V,
    ) -> Result<(), breadthfirst::Error>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<tree::RefIter<'a>>,
        StateMut: BorrowMut<breadthfirst::State>,
        V: git_traverse::tree::Visit,
    {
        breadthfirst(self.clone(), state, find, delegate)
    }
}
