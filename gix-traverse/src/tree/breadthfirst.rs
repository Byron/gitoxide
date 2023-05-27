use std::collections::VecDeque;

use gix_hash::ObjectId;

/// The error is part of the item returned by the [`traverse()`][impl_::traverse()] function.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("The tree {oid} could not be found")]
    NotFound { oid: ObjectId },
    #[error("The delegate cancelled the operation")]
    Cancelled,
    #[error(transparent)]
    ObjectDecode(#[from] gix_object::decode::Error),
}

/// The state used and potentially shared by multiple tree traversals.
#[derive(Default, Clone)]
pub struct State {
    next: VecDeque<ObjectId>,
    buf: Vec<u8>,
}

impl State {
    fn clear(&mut self) {
        self.next.clear();
        self.buf.clear();
    }
}

pub(crate) mod impl_ {
    use std::borrow::BorrowMut;

    use gix_hash::oid;
    use gix_object::{tree::EntryMode, TreeRefIter};

    use super::{Error, State};
    use crate::tree::Visit;

    /// Start a breadth-first iteration over the `root` trees entries.
    ///
    /// * `root`
    ///   * the tree to iterate in a nested fashion.
    /// * `state` - all state used for the iteration. If multiple iterations are performed, allocations can be minimized by reusing
    ///   this state.
    /// * `find` - a way to lookup new object data during traversal by their `ObjectId`, writing their data into buffer and returning
    ///    an iterator over entries if the object is present and is a tree. Caching should be implemented within this function
    ///    as needed. The return value is `Option<TreeIter>` which degenerates all error information. Not finding a commit should also
    ///    be considered an errors as all objects in the tree DAG should be present in the database. Hence [`Error::NotFound`] should
    ///    be escalated into a more specific error if its encountered by the caller.
    /// * `delegate` - A way to observe entries and control the iteration while allowing the optimizer to let you pay only for what you use.
    pub fn traverse<StateMut, Find, V>(
        root: TreeRefIter<'_>,
        mut state: StateMut,
        mut find: Find,
        delegate: &mut V,
    ) -> Result<(), Error>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<TreeRefIter<'a>>,
        StateMut: BorrowMut<State>,
        V: Visit,
    {
        let state = state.borrow_mut();
        state.clear();
        let mut tree = root;
        loop {
            for entry in tree {
                let entry = entry?;
                match entry.mode {
                    EntryMode::Tree => {
                        use crate::tree::visit::Action::*;
                        delegate.push_path_component(entry.filename);
                        let action = delegate.visit_tree(&entry);
                        match action {
                            Skip => {}
                            Continue => {
                                delegate.pop_path_component();
                                delegate.push_back_tracked_path_component(entry.filename);
                                state.next.push_back(entry.oid.to_owned())
                            }
                            Cancel => {
                                return Err(Error::Cancelled);
                            }
                        }
                    }
                    _non_tree => {
                        delegate.push_path_component(entry.filename);
                        if delegate.visit_nontree(&entry).cancelled() {
                            return Err(Error::Cancelled);
                        }
                    }
                }
                delegate.pop_path_component();
            }
            match state.next.pop_front() {
                Some(oid) => {
                    delegate.pop_front_tracked_path_and_set_current();
                    match find(&oid, &mut state.buf) {
                        Some(tree_iter) => tree = tree_iter,
                        None => return Err(Error::NotFound { oid: oid.to_owned() }),
                    }
                }
                None => break Ok(()),
            }
        }
    }
}
