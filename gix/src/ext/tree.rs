use std::borrow::BorrowMut;

use gix_hash::oid;
use gix_object::TreeRefIter;
use gix_traverse::tree::breadthfirst;

pub trait Sealed {}

/// An extension trait for tree iterators
pub trait TreeIterExt: Sealed {
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
        V: gix_traverse::tree::Visit;
}

impl<'d> Sealed for TreeRefIter<'d> {}

impl<'d> TreeIterExt for TreeRefIter<'d> {
    fn traverse<StateMut, Find, V>(
        &self,
        state: StateMut,
        find: Find,
        delegate: &mut V,
    ) -> Result<(), breadthfirst::Error>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<TreeRefIter<'a>>,
        StateMut: BorrowMut<breadthfirst::State>,
        V: gix_traverse::tree::Visit,
    {
        breadthfirst(self.clone(), state, find, delegate)
    }
}
