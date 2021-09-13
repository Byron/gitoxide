use std::borrow::BorrowMut;

use git_hash::oid;
use git_object::TreeRefIter;
use git_traverse::tree::breadthfirst;

pub trait Sealed {}

/// An extension trait for tree iterators
pub trait TreeIterExt: Sealed {
    /// Traverse both `self` and the `other` tree in lock-step to allow computing what's needed to turn `self` into `other`,
    /// with `state` being provided to allow reusing allocations and `find` being a function to lookup trees and turn them
    /// into an iterator.
    ///
    /// The `delegate` implements a way to store the desired information about the traversal, allowing to pay only for what is needed.
    /// It is also expected to store the result of the comparison, hence _unit_ is returned.
    #[cfg(feature = "git-diff")]
    fn changes_needed<FindFn, R, StateMut>(
        &self,
        other: TreeRefIter<'_>,
        state: StateMut,
        find: FindFn,
        delegate: &mut R,
    ) -> Result<(), git_diff::tree::changes::Error>
    where
        FindFn: for<'b> FnMut(&oid, &'b mut Vec<u8>) -> Option<TreeRefIter<'b>>,
        R: git_diff::tree::Visit,
        StateMut: BorrowMut<git_diff::tree::State>;

    /// Traverse this tree with `state` being provided to potentially reuse allocations, and `find` being a function to lookup trees
    /// and turn them into iterators.
    ///
    /// The `delegate` implements a way to store details about the traversal to allow paying only for what's actually used.
    /// Since it is expected to store the operation result, _unit_ is returned.
    fn traverse<StateMut, Find, V>(
        &self,
        state: StateMut,
        find: Find,
        delegate: &mut V,
    ) -> Result<(), breadthfirst::Error>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<TreeRefIter<'a>>,
        StateMut: BorrowMut<breadthfirst::State>,
        V: git_traverse::tree::Visit;
}

impl<'d> Sealed for TreeRefIter<'d> {}

impl<'d> TreeIterExt for TreeRefIter<'d> {
    #[cfg(feature = "git-diff")]
    fn changes_needed<FindFn, R, StateMut>(
        &self,
        other: TreeRefIter<'_>,
        state: StateMut,
        find: FindFn,
        delegate: &mut R,
    ) -> Result<(), git_diff::tree::changes::Error>
    where
        FindFn: for<'b> FnMut(&oid, &'b mut Vec<u8>) -> Option<TreeRefIter<'b>>,
        R: git_diff::tree::Visit,
        StateMut: BorrowMut<git_diff::tree::State>,
    {
        git_diff::tree::Changes::from(Some(self.clone())).needed_to_obtain(other, state, find, delegate)
    }

    fn traverse<StateMut, Find, V>(
        &self,
        state: StateMut,
        find: Find,
        delegate: &mut V,
    ) -> Result<(), breadthfirst::Error>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<TreeRefIter<'a>>,
        StateMut: BorrowMut<breadthfirst::State>,
        V: git_traverse::tree::Visit,
    {
        breadthfirst(self.clone(), state, find, delegate)
    }
}
