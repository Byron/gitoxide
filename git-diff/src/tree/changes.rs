use crate::{
    tree,
    tree::{visit::Change, TreeInfo, TreeInfoPair},
};
use git_hash::{oid, ObjectId};
use git_object::immutable;
use quick_error::quick_error;
use std::{borrow::BorrowMut, collections::VecDeque};

quick_error! {
    /// The error returned by [tree::Changes::needed_to_obtain()].
    #[derive(Debug)]
    #[allow(missing_docs)]
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

impl<'a> tree::Changes<'a> {
    /// Calculate the changes that would need to be applied to `self` to get `other`.
    ///
    /// * The `state` maybe owned or mutably borrowed to allow reuses allocated data structures through multiple runs.
    /// * `locate` is a function `f(object_id, &mut buffer) -> Option<TreeIter>` to return a `TreeIter` for the given object id backing
    ///   its data in the given buffer. Returning `None` is unexpected as these trees are obtained during iteration, and in a typical
    ///   database errors are not expected either which is why the error case is omitted. To allow proper error reporting, [`Error::NotFound`]
    ///   should be converted into a more telling error.
    /// * `delegate` will receive the computed changes, see [`tree::Visit`] for more information on what to expect.
    ///
    /// # Notes
    ///
    /// * To obtain progress, implement it within the `delegate`.
    /// * Tree entries are expected to be ordered using [`tree-entry-comparison`][git_cmp_c] (the same [in Rust][git_cmp_rs])
    /// * it does a breadth first iteration as buffer space only fits two trees, the current one on the one we compare with.
    /// * does not do rename tracking but attempts to reduce allocations to zero (so performance is mostly determined
    ///   by the delegate implementation which should be as specific as possible. Rename tracking can be computed on top of the changes
    ///   received by the `delegate`.
    /// * cycle checking is not performed, but can be performed in the delegate which can return [`tree::visit::Action::Cancel`] to stop the traversal.
    /// * [std::mem::ManuallyDrop] is used because `Peekable` is needed. When using it as wrapper around our no-drop iterators, all of the sudden
    ///   borrowcheck complains as Drop is present (even though it's not)
    ///
    /// [git_cmp_c]: https://github.com/git/git/blob/311531c9de557d25ac087c1637818bd2aad6eb3a/tree-diff.c#L49:L65
    /// [git_cmp_rs]: https://github.com/Byron/gitoxide/blob/a4d5f99c8dc99bf814790928a3bf9649cd99486b/git-object/src/mutable/tree.rs#L52-L55
    pub fn needed_to_obtain<LocateFn, R, StateMut>(
        mut self,
        other: immutable::TreeIter<'a>,
        mut state: StateMut,
        mut locate: LocateFn,
        delegate: &mut R,
    ) -> Result<(), Error>
    where
        LocateFn: for<'b> FnMut(&oid, &'b mut Vec<u8>) -> Option<immutable::tree::TreeIter<'b>>,
        R: tree::Visit,
        StateMut: BorrowMut<tree::State<R::PathId>>,
    {
        let state = state.borrow_mut();
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
                        Less => catchup_lhs_with_rhs(&mut lhs_entries, lhs, rhs, &mut state.trees, delegate)?,
                        Greater => catchup_rhs_with_lhs(&mut rhs_entries, lhs, rhs, &mut state.trees, delegate)?,
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

fn delete_entry_schedule_recursion<R: tree::Visit>(
    entry: immutable::tree::Entry<'_>,
    queue: &mut VecDeque<TreeInfoPair<R::PathId>>,
    delegate: &mut R,
) -> Result<(), Error> {
    delegate.push_path_component(entry.filename);
    if delegate
        .visit(Change::Deletion {
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

fn add_entry_schedule_recursion<R: tree::Visit>(
    entry: immutable::tree::Entry<'_>,
    queue: &mut VecDeque<TreeInfoPair<R::PathId>>,
    delegate: &mut R,
) -> Result<(), Error> {
    delegate.push_path_component(entry.filename);
    if delegate
        .visit(Change::Addition {
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
fn catchup_rhs_with_lhs<R: tree::Visit>(
    rhs_entries: &mut IteratorType<immutable::TreeIter<'_>>,
    lhs: immutable::tree::Entry<'_>,
    rhs: immutable::tree::Entry<'_>,
    queue: &mut VecDeque<TreeInfoPair<R::PathId>>,
    delegate: &mut R,
) -> Result<(), Error> {
    use std::cmp::Ordering::*;
    add_entry_schedule_recursion(rhs, queue, delegate)?;
    loop {
        match rhs_entries.peek() {
            Some(Ok(rhs)) => match lhs.filename.cmp(rhs.filename) {
                Equal => {
                    let rhs = rhs_entries.next().transpose()?.expect("the peeked item tobe present");
                    delegate.pop_path_component();
                    handle_lhs_and_rhs_with_equal_filenames(lhs, rhs, queue, delegate)?;
                    break;
                }
                Greater => {
                    let rhs = rhs_entries.next().transpose()?.expect("the peeked item tobe present");
                    delegate.pop_path_component();
                    add_entry_schedule_recursion(rhs, queue, delegate)?;
                }
                Less => {
                    delegate.pop_path_component();
                    delete_entry_schedule_recursion(lhs, queue, delegate)?;
                    break;
                }
            },
            Some(Err(err)) => return Err(Error::EntriesDecode(err.to_owned())),
            None => {
                delegate.pop_path_component();
                delete_entry_schedule_recursion(lhs, queue, delegate)?;
                break;
            }
        }
    }
    Ok(())
}

fn catchup_lhs_with_rhs<R: tree::Visit>(
    lhs_entries: &mut IteratorType<immutable::TreeIter<'_>>,
    lhs: immutable::tree::Entry<'_>,
    rhs: immutable::tree::Entry<'_>,
    queue: &mut VecDeque<TreeInfoPair<R::PathId>>,
    delegate: &mut R,
) -> Result<(), Error> {
    use std::cmp::Ordering::*;
    delete_entry_schedule_recursion(lhs, queue, delegate)?;
    loop {
        match lhs_entries.peek() {
            Some(Ok(lhs)) => match lhs.filename.cmp(rhs.filename) {
                Equal => {
                    let lhs = lhs_entries.next().expect("the peeked item to be present")?;
                    delegate.pop_path_component();
                    handle_lhs_and_rhs_with_equal_filenames(lhs, rhs, queue, delegate)?;
                    break;
                }
                Less => {
                    let lhs = lhs_entries.next().expect("the peeked item to be present")?;
                    delegate.pop_path_component();
                    delete_entry_schedule_recursion(lhs, queue, delegate)?;
                }
                Greater => {
                    delegate.pop_path_component();
                    add_entry_schedule_recursion(rhs, queue, delegate)?;
                    break;
                }
            },
            Some(Err(err)) => return Err(Error::EntriesDecode(err.to_owned())),
            None => {
                delegate.pop_path_component();
                add_entry_schedule_recursion(rhs, queue, delegate)?;
                break;
            }
        }
    }
    Ok(())
}

fn handle_lhs_and_rhs_with_equal_filenames<R: tree::Visit>(
    lhs: immutable::tree::Entry<'_>,
    rhs: immutable::tree::Entry<'_>,
    queue: &mut VecDeque<TreeInfoPair<R::PathId>>,
    delegate: &mut R,
) -> Result<(), Error> {
    use git_object::tree::EntryMode::*;
    match (lhs.mode, rhs.mode) {
        (Tree, Tree) => {
            let path_id = delegate.push_tracked_path_component(lhs.filename);
            if lhs.oid != rhs.oid
                && delegate
                    .visit(Change::Modification {
                        previous_entry_mode: lhs.mode,
                        previous_oid: lhs.oid.to_owned(),
                        entry_mode: rhs.mode,
                        oid: rhs.oid.to_owned(),
                    })
                    .cancelled()
            {
                return Err(Error::Cancelled);
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
                .visit(Change::Deletion {
                    entry_mode: lhs.mode,
                    oid: lhs.oid.to_owned(),
                })
                .cancelled()
            {
                return Err(Error::Cancelled);
            };
            if delegate
                .visit(Change::Addition {
                    entry_mode: rhs.mode,
                    oid: rhs.oid.to_owned(),
                })
                .cancelled()
            {
                return Err(Error::Cancelled);
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
                .visit(Change::Deletion {
                    entry_mode: lhs.mode,
                    oid: lhs.oid.to_owned(),
                })
                .cancelled()
            {
                return Err(Error::Cancelled);
            }
            if delegate
                .visit(Change::Addition {
                    entry_mode: rhs.mode,
                    oid: rhs.oid.to_owned(),
                })
                .cancelled()
            {
                return Err(Error::Cancelled);
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
                    .visit(Change::Modification {
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

type IteratorType<I> = std::mem::ManuallyDrop<std::iter::Peekable<I>>;

fn peekable<I: Iterator>(iter: I) -> IteratorType<I> {
    std::mem::ManuallyDrop::new(iter.peekable())
}
