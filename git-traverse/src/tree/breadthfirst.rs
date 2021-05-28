use crate::tree::visit::Visit;
use git_hash::{oid, ObjectId};
use git_object::{immutable, tree};
use quick_error::quick_error;
use std::{borrow::BorrowMut, collections::VecDeque};

quick_error! {
    /// The error is part of the item returned by the [`traverse()`] function.
    #[derive(Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        NotFound{oid: ObjectId} {
            display("The tree {} could not be found", oid)
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
pub struct State {
    next: VecDeque<(bool, ObjectId)>,
    buf: Vec<u8>,
}

impl State {
    fn clear(&mut self) {
        self.next.clear();
        self.buf.clear();
    }
}

/// Start a breadth-first iteration over the `root` trees entries.
///
/// * `root`
///   * the tree to iterate in a nested fashion.
/// * `state` - all state used for the iteration. If multiple iterations are performed, allocations can be minimized by reusing
///   this state.
/// * `find` - a way to lookup new object data during traversal by their ObjectId, writing their data into buffer and returning
///    an iterator over entries if the object is present and is a tree. Caching should be implemented within this function
///    as needed. The return value is `Option<TreeIter>` which degenerates all error information. Not finding a commit should also
///    be considered an errors as all objects in the tree DAG should be present in the database. Hence [`Error::NotFound`] should
///    be escalated into a more specific error if its encountered by the caller.
/// * `delegate` - A way to observe entries and control the iteration while allowing the optimizer to let you pay only for what you use.
pub fn traverse<StateMut, Find, V>(
    root: immutable::TreeIter<'_>,
    mut state: StateMut,
    mut find: Find,
    delegate: &mut V,
) -> Result<(), Error>
where
    Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<immutable::TreeIter<'a>>,
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
                tree::EntryMode::Tree => {
                    use super::visit::Action::*;
                    delegate.push_path_component(entry.filename);
                    let action = delegate.visit_tree(&entry);
                    match action {
                        Skip => {}
                        Continue => {
                            delegate.pop_path_component();
                            delegate.push_back_tracked_path_component(entry.filename);
                            state.next.push_back((true, entry.oid.to_owned()))
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
            Some((should_pop_path, oid)) => {
                if should_pop_path {
                    delegate.pop_front_tracked_path_and_set_current();
                }
                match find(&oid, &mut state.buf) {
                    Some(tree_iter) => tree = tree_iter,
                    None => return Err(Error::NotFound { oid: oid.to_owned() }),
                }
            }
            None => break Ok(()),
        }
    }
}
