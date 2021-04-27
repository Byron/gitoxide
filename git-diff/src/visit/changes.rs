use crate::{
    visit,
    visit::{changes::Error::Cancelled, record::Change},
};
use git_hash::{oid, ObjectId};
use git_object::{immutable, tree};
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        NotFound(oid: ObjectId) {
            display("The object {} referenced by the tree was not found in the database", oid)
        }
        Cancelled {
            display("The delegate cancelled the operation")
        }
        EntriesDecode(err: immutable::object::decode::Error) {
            display("tree entries could not be decoded.")
            from()
            source(err)
        }
    }
}

impl<'a> visit::Changes<'a> {
    /// Returns the changes that need to be applied to `self` to get `other`.
    ///
    /// # Notes
    ///
    /// * Tree entries are expected to be ordered using [`tree-entry-comparison`][git_cmp_c] (the same [in Rust][git_cmp_rs])
    ///
    /// [git_cmp_c]: https://github.com/git/git/blob/311531c9de557d25ac087c1637818bd2aad6eb3a/tree-diff.c#L49:L65
    /// [git_cmp_rs]: https://github.com/Byron/gitoxide/blob/a4d5f99c8dc99bf814790928a3bf9649cd99486b/git-object/src/mutable/tree.rs#L52-L55
    ///
    /// * it does a breadth first iteration as buffer space only fits two trees, the current one on the one we compare with.
    /// * does not do rename tracking but attempts to reduce allocations to zero (so performance is mostly determined
    ///   by the delegate implementation which should be as specific as possible.
    pub fn needed_to_obtain<LocateFn>(
        mut self,
        other: immutable::TreeIter<'a>,
        _state: &mut visit::State,
        _locate: LocateFn,
        delegate: &mut impl visit::Record,
    ) -> Result<(), Error>
    where
        LocateFn: for<'b> FnMut(&oid, &'b mut Vec<u8>) -> Option<immutable::Object<'b>>,
    {
        let mut lhs_entries = self.0.take().unwrap_or_default();
        let mut rhs_entries = other;

        loop {
            delegate.pop_path_name();
            match (lhs_entries.next(), rhs_entries.next()) {
                (None, None) => return Ok(()),
                (Some(lhs), Some(rhs)) => {
                    use std::cmp::Ordering::*;
                    let (lhs, rhs) = (lhs?, rhs?);
                    match lhs.filename.cmp(rhs.filename) {
                        Equal => {
                            use tree::EntryMode::*;
                            match (lhs.mode, rhs.mode) {
                                (Tree, Tree) => {
                                    let _path_id = delegate.push_tree_name(lhs.filename);
                                    if lhs.oid != rhs.oid {
                                        if delegate
                                            .record(Change::Modification {
                                                previous_entry_mode: lhs.mode,
                                                previous_oid: lhs.oid.to_owned(),
                                                entry_mode: rhs.mode,
                                                oid: rhs.oid.to_owned(),
                                            })
                                            .cancelled()
                                        {
                                            return Err(Cancelled);
                                        }
                                    }
                                    todo!("schedule tree|tree iteration schedule the trees with stack")
                                }
                                (lhs_mode, Tree) if lhs_mode.is_no_tree() => {
                                    let _path_id = delegate.push_tree_name(lhs.filename);
                                    if delegate
                                        .record(Change::Deletion {
                                            entry_mode: lhs.mode,
                                            oid: lhs.oid.to_owned(),
                                        })
                                        .cancelled()
                                    {
                                        return Err(Cancelled);
                                    };
                                    if delegate
                                        .record(Change::Addition {
                                            entry_mode: rhs.mode,
                                            oid: rhs.oid.to_owned(),
                                        })
                                        .cancelled()
                                    {
                                        return Err(Cancelled);
                                    };
                                    todo!("delete non-tree ✓|add tree✓ - add rhs children recursively")
                                }
                                (Tree, rhs_mode) if rhs_mode.is_no_tree() => {
                                    let _path_id = delegate.push_tree_name(lhs.filename);
                                    if delegate
                                        .record(Change::Deletion {
                                            entry_mode: lhs.mode,
                                            oid: lhs.oid.to_owned(),
                                        })
                                        .cancelled()
                                    {
                                        return Err(Error::Cancelled);
                                    }
                                    todo!("delete lhs recursively|add non-tree")
                                }
                                (lhs_non_tree, rhs_non_tree) => {
                                    delegate.push_non_tree_name(lhs.filename);
                                    debug_assert!(lhs_non_tree.is_no_tree() && rhs_non_tree.is_no_tree());
                                    if lhs.oid != rhs.oid {
                                        if delegate
                                            .record(Change::Modification {
                                                previous_entry_mode: lhs.mode,
                                                previous_oid: lhs.oid.to_owned(),
                                                entry_mode: rhs.mode,
                                                oid: rhs.oid.to_owned(),
                                            })
                                            .cancelled()
                                        {
                                            return Err(Error::Cancelled);
                                        }
                                    }
                                }
                            };
                        }
                        Less => todo!("entry compares less - catch up"),
                        Greater => todo!("entry compares more - let the other side catch up"),
                    }
                }
                (Some(lhs), None) => {
                    let lhs = lhs?;
                    delegate.push_non_tree_name(lhs.filename);
                    if delegate
                        .record(Change::Deletion {
                            entry_mode: lhs.mode,
                            oid: lhs.oid.to_owned(),
                        })
                        .cancelled()
                    {
                        break Err(Error::Cancelled);
                    }
                    if lhs.mode.is_tree() {
                        delegate.pop_path_name();
                        let _path_id = delegate.push_tree_name(lhs.filename);
                        todo!("delete tree recursively")
                    }
                }
                (None, Some(rhs)) => {
                    let rhs = rhs?;
                    delegate.push_non_tree_name(rhs.filename);
                    if delegate
                        .record(Change::Addition {
                            entry_mode: rhs.mode,
                            oid: rhs.oid.to_owned(),
                        })
                        .cancelled()
                    {
                        break Err(Error::Cancelled);
                    }
                    if rhs.mode.is_tree() {
                        delegate.pop_path_name();
                        let _path_id = delegate.push_tree_name(rhs.filename);
                        todo!("add tree recursively")
                    }
                }
            }
        }
    }
}
