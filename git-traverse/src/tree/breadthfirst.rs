use crate::tree::visit::Visit;
use git_hash::{oid, ObjectId};
use git_object::{immutable, tree};
use quick_error::quick_error;
use std::{borrow::BorrowMut, collections::VecDeque};

quick_error! {
    /// The error is part of the item returned by the [breadthfirst] function.
    #[derive(Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        NotFound{oid: ObjectId} {
            display("The object {} could not be found", oid)
        }
        Cancelled {
            display("The delegate cancelled the operation")
        }
        ObjectDecode(err: immutable::object::decode::Error) {
            display("An object could not be decoded")
            source(err)
            from()
        }
    }
}

/// The state used and potentially shared by multiple tree traversals.
#[derive(Default, Clone)]
pub struct State<PathId: Clone> {
    next: VecDeque<(Option<PathId>, ObjectId)>,
    buf: Vec<u8>,
}

impl<PathId: Clone> State<PathId> {
    fn clear(&mut self) {
        self.next.clear();
        self.buf.clear();
    }
}

/// Create a new instance.
///
/// * `find` - a way to lookup new object data during traversal by their ObjectId, writing their data into buffer and returning
///    an iterator over entries if the object is present and is a tree. Caching should be implemented within this function
///    as needed. The return value is `Option<TreeIter>` which degenerates all error information. Not finding a commit should also
///    be considered an errors as all objects in the tree DAG should be present in the database. Hence [`Error::NotFound`] should
///    be escalated into a more specific error if its encountered by the caller.
/// * `state` - all state used for the iteration. If multiple iterations are performed, allocations can be minimized by reusing
///   this state.
/// * `root`
///   * the starting points of the iteration
///   * each commit they lead to will only be returned once, including the tip that started it
pub fn traverse<StateMut, Find, V>(
    root: impl Into<ObjectId>,
    mut state: StateMut,
    mut find: Find,
    delegate: &mut V,
) -> Result<(), Error>
where
    Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<immutable::TreeIter<'a>>,
    StateMut: BorrowMut<State<V::PathId>>,
    V: Visit,
{
    let state = state.borrow_mut();
    state.clear();
    state.next.push_back((None, root.into()));
    while let Some((path_id, oid)) = state.next.pop_front() {
        if let Some(path_id) = path_id {
            delegate.set_current_path(path_id);
        }
        match find(&oid, &mut state.buf) {
            Some(tree_iter) => {
                for entry in tree_iter {
                    let entry = entry?;
                    match entry.mode {
                        tree::EntryMode::Tree => {
                            use super::visit::Action::*;
                            let path_id = delegate.push_tracked_path_component(entry.filename);
                            let action = delegate.visit_tree(&entry);
                            match action {
                                Skip => {}
                                Continue => state.next.push_back((Some(path_id), entry.oid.to_owned())),
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
            }
            None => return Err(Error::NotFound { oid: oid.to_owned() }),
        }
    }
    Ok(())
}
