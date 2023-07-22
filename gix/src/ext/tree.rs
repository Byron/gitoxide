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

/// Extensions for [EntryRef][gix_object::tree::EntryRef].
pub trait TreeEntryRefExt<'a>: 'a {
    /// Attach [`Repository`][crate::Repository] to the given tree entry. It can be detached later with `detach()`.
    fn attach<'repo>(self, repo: &'repo crate::Repository) -> crate::object::tree::EntryRef<'repo, 'a>;
}

impl<'a> TreeEntryRefExt<'a> for gix_object::tree::EntryRef<'a> {
    fn attach<'repo>(self, repo: &'repo crate::Repository) -> crate::object::tree::EntryRef<'repo, 'a> {
        crate::object::tree::EntryRef { inner: self, repo }
    }
}

/// Extensions for [Entry][gix_object::tree::Entry].
pub trait TreeEntryExt {
    /// Attach [`Repository`][crate::Repository] to the given tree entry. It can be detached later with `detach()`.
    fn attach(self, repo: &crate::Repository) -> crate::object::tree::Entry<'_>;
}

impl TreeEntryExt for gix_object::tree::Entry {
    fn attach(self, repo: &crate::Repository) -> crate::object::tree::Entry<'_> {
        crate::object::tree::Entry { inner: self, repo }
    }
}
