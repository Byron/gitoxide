use crate::{
    visit,
    visit::{changes::Error::Cancelled, record::Change, TreeInfo, TreeInfoPair},
};
use git_hash::{oid, ObjectId};
use git_object::{immutable, tree};
use quick_error::quick_error;
use std::collections::VecDeque;

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
    /// * cycle checking is not performed, but can be performed in the delegate
    /// * [ManuallyDrop] is used because `Peekable` is needed. When using it as wrapper around our no-drop iterators, all of the sudden
    ///   borrowcheck complains as Drop is present (even though it's not)
    pub fn needed_to_obtain<LocateFn, R>(
        mut self,
        other: immutable::TreeIter<'a>,
        state: &mut visit::State<R::PathId>,
        mut locate: LocateFn,
        delegate: &mut R,
    ) -> Result<(), Error>
    where
        LocateFn: for<'b> FnMut(&oid, &'b mut Vec<u8>) -> Option<immutable::tree::TreeIter<'b>>,
        R: visit::Record,
    {
        state.clear();
        let mut lhs_entries = peekable(self.0.take().unwrap_or_default());
        let mut rhs_entries = peekable(other);
        let mut avoid_popping_path: Option<()> = None;

        loop {
            if avoid_popping_path.take().is_none() {
                delegate.pop_path_component();
            }
            match (lhs_entries.next(), rhs_entries.next()) {
                (None, None) => {
                    match state.trees.pop_front() {
                        Some((None, Some(rhs))) => {
                            delegate.set_current_path(rhs.parent_path_id.clone());
                            rhs_entries =
                                peekable(locate(&rhs.tree_id, &mut state.buf2).ok_or(Error::NotFound(rhs.tree_id))?);
                        }
                        Some((Some(lhs), Some(rhs))) => {
                            delegate.set_current_path(lhs.parent_path_id.clone());
                            lhs_entries =
                                peekable(locate(&lhs.tree_id, &mut state.buf1).ok_or(Error::NotFound(lhs.tree_id))?);
                            rhs_entries =
                                peekable(locate(&rhs.tree_id, &mut state.buf2).ok_or(Error::NotFound(rhs.tree_id))?);
                        }
                        Some((Some(lhs), None)) => {
                            delegate.set_current_path(lhs.parent_path_id.clone());
                            lhs_entries =
                                peekable(locate(&lhs.tree_id, &mut state.buf1).ok_or(Error::NotFound(lhs.tree_id))?);
                        }
                        Some((None, None)) => unreachable!("BUG: it makes no sense to fill the stack with empties"),
                        None => return Ok(()),
                    };
                    avoid_popping_path = Some(());
                }
                (Some(lhs), Some(rhs)) => {
                    use std::cmp::Ordering::*;
                    let (lhs, rhs) = (lhs?, rhs?);
                    match lhs.filename.cmp(rhs.filename) {
                        Equal => handle_lhs_and_rhs_with_equal_filenames(lhs, rhs, &mut state.trees, delegate)?,
                        Less => {
                            delete_entry_schedule_recursion(lhs, &mut state.trees, delegate)?;
                            'inner_less: loop {
                                match lhs_entries.peek() {
                                    Some(Ok(lhs)) => match lhs.filename.cmp(rhs.filename) {
                                        Equal => {
                                            let lhs =
                                                lhs_entries.next().transpose()?.expect("the peeked item tobe present");
                                            handle_lhs_and_rhs_with_equal_filenames(
                                                lhs,
                                                rhs,
                                                &mut state.trees,
                                                delegate,
                                            )?;
                                            break 'inner_less;
                                        }
                                        Less => {
                                            let lhs =
                                                lhs_entries.next().transpose()?.expect("the peeked item tobe present");
                                            delegate.pop_path_component();
                                            delete_entry_schedule_recursion(lhs, &mut state.trees, delegate)?;
                                        }
                                        Greater => {
                                            delegate.pop_path_component();
                                            add_entry_schedule_recursion(rhs, &mut state.trees, delegate)?;
                                            break 'inner_less;
                                        }
                                    },
                                    Some(Err(err)) => return Err(Error::EntriesDecode(err.to_owned())),
                                    None => {
                                        delegate.pop_path_component();
                                        add_entry_schedule_recursion(rhs, &mut state.trees, delegate)?;
                                        break 'inner_less;
                                    }
                                }
                            }
                        }
                        Greater => {
                            add_entry_schedule_recursion(rhs, &mut state.trees, delegate)?;
                            'inner_greater: loop {
                                match rhs_entries.peek() {
                                    Some(Ok(rhs)) => match lhs.filename.cmp(rhs.filename) {
                                        Equal => {
                                            let rhs =
                                                rhs_entries.next().transpose()?.expect("the peeked item tobe present");
                                            handle_lhs_and_rhs_with_equal_filenames(
                                                lhs,
                                                rhs,
                                                &mut state.trees,
                                                delegate,
                                            )?;
                                            break 'inner_greater;
                                        }
                                        Greater => {
                                            let rhs =
                                                rhs_entries.next().transpose()?.expect("the peeked item tobe present");
                                            delegate.pop_path_component();
                                            add_entry_schedule_recursion(rhs, &mut state.trees, delegate)?;
                                        }
                                        Less => {
                                            delegate.pop_path_component();
                                            delete_entry_schedule_recursion(lhs, &mut state.trees, delegate)?;
                                            break 'inner_greater;
                                        }
                                    },
                                    Some(Err(err)) => return Err(Error::EntriesDecode(err.to_owned())),
                                    None => {
                                        todo!("GREATER: catchup less: break inner depleted - it never caught up");
                                        // break 'inner;
                                    }
                                }
                            }
                        }
                    }
                }
                (Some(lhs), None) => {
                    let lhs = lhs?;
                    delete_entry_schedule_recursion(lhs, &mut state.trees, delegate)?;
                }
                (None, Some(rhs)) => {
                    let rhs = rhs?;
                    add_entry_schedule_recursion(rhs, &mut state.trees, delegate)?;
                }
            }
        }
    }
}

fn delete_entry_schedule_recursion<R: visit::Record>(
    entry: immutable::tree::Entry<'_>,
    queue: &mut VecDeque<TreeInfoPair<R::PathId>>,
    delegate: &mut R,
) -> Result<(), Error> {
    delegate.push_path_component(entry.filename);
    if delegate
        .record(Change::Deletion {
            entry_mode: entry.mode,
            oid: entry.oid.to_owned(),
        })
        .cancelled()
    {
        return Err(Error::Cancelled);
    }
    if entry.mode.is_tree() {
        delegate.pop_path_component();
        let path_id = delegate.push_tracked_path_component(entry.filename);
        queue.push_back((
            Some(TreeInfo {
                tree_id: entry.oid.to_owned(),
                parent_path_id: path_id,
            }),
            None,
        ));
    }
    Ok(())
}

fn add_entry_schedule_recursion<R: visit::Record>(
    entry: immutable::tree::Entry<'_>,
    queue: &mut VecDeque<TreeInfoPair<R::PathId>>,
    delegate: &mut R,
) -> Result<(), Error> {
    delegate.push_path_component(entry.filename);
    if delegate
        .record(Change::Addition {
            entry_mode: entry.mode,
            oid: entry.oid.to_owned(),
        })
        .cancelled()
    {
        return Err(Error::Cancelled);
    }
    if entry.mode.is_tree() {
        delegate.pop_path_component();
        let path_id = delegate.push_tracked_path_component(entry.filename);
        queue.push_back((
            None,
            Some(TreeInfo {
                tree_id: entry.oid.to_owned(),
                parent_path_id: path_id,
            }),
        ))
    }
    Ok(())
}

fn handle_lhs_and_rhs_with_equal_filenames<R: visit::Record>(
    lhs: immutable::tree::Entry<'_>,
    rhs: immutable::tree::Entry<'_>,
    queue: &mut VecDeque<TreeInfoPair<R::PathId>>,
    delegate: &mut R,
) -> Result<(), Error> {
    use tree::EntryMode::*;
    match (lhs.mode, rhs.mode) {
        (Tree, Tree) => {
            let path_id = delegate.push_tracked_path_component(lhs.filename);
            if lhs.oid != rhs.oid
                && delegate
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
            queue.push_back((
                Some(TreeInfo {
                    tree_id: lhs.oid.to_owned(),
                    parent_path_id: path_id.clone(),
                }),
                Some(TreeInfo {
                    tree_id: rhs.oid.to_owned(),
                    parent_path_id: path_id,
                }),
            ));
        }
        (lhs_mode, Tree) if lhs_mode.is_no_tree() => {
            let path_id = delegate.push_tracked_path_component(lhs.filename);
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
            queue.push_back((
                None,
                Some(TreeInfo {
                    tree_id: rhs.oid.to_owned(),
                    parent_path_id: path_id,
                }),
            ));
        }
        (Tree, rhs_mode) if rhs_mode.is_no_tree() => {
            let path_id = delegate.push_tracked_path_component(lhs.filename);
            if delegate
                .record(Change::Deletion {
                    entry_mode: lhs.mode,
                    oid: lhs.oid.to_owned(),
                })
                .cancelled()
            {
                return Err(Error::Cancelled);
            }
            if delegate
                .record(Change::Addition {
                    entry_mode: rhs.mode,
                    oid: rhs.oid.to_owned(),
                })
                .cancelled()
            {
                return Err(Cancelled);
            };
            queue.push_back((
                Some(TreeInfo {
                    tree_id: lhs.oid.to_owned(),
                    parent_path_id: path_id,
                }),
                None,
            ));
        }
        (lhs_non_tree, rhs_non_tree) => {
            delegate.push_path_component(lhs.filename);
            debug_assert!(lhs_non_tree.is_no_tree() && rhs_non_tree.is_no_tree());
            if lhs.oid != rhs.oid
                && delegate
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
    };
    Ok(())
}

fn peekable<I: Iterator>(iter: I) -> std::mem::ManuallyDrop<std::iter::Peekable<I>> {
    std::mem::ManuallyDrop::new(iter.peekable())
}
