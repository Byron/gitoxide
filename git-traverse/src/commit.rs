///
pub mod ancestors {
    use std::{
        borrow::BorrowMut,
        collections::{BTreeSet, VecDeque},
    };

    use git_hash::{oid, ObjectId};
    use git_object::commit;
    use quick_error::quick_error;

    quick_error! {
        /// The error is part of the item returned by the [Ancestors] iterator.
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            NotFound{oid: ObjectId} {
                display("The commit {} could not be found", oid)
            }
            ObjectDecode(err: git_object::decode::Error) {
                display("An object could not be decoded")
                source(err)
                from()
            }
        }
    }

    /// The state used and potentially shared by multiple graph traversals.
    #[derive(Default, Clone)]
    pub struct State {
        next: VecDeque<ObjectId>,
        buf: Vec<u8>,
        seen: BTreeSet<ObjectId>,
    }

    impl State {
        fn clear(&mut self) {
            self.next.clear();
            self.buf.clear();
            self.seen.clear();
        }
    }

    /// An iterator over the ancestors one or more starting commits
    pub struct Ancestors<Find, Predicate, StateMut> {
        find: Find,
        predicate: Predicate,
        state: StateMut,
    }

    impl<Find, StateMut> Ancestors<Find, fn(&oid) -> bool, StateMut>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<commit::CommitRefIter<'a>>,
        StateMut: BorrowMut<State>,
    {
        /// Create a new instance.
        ///
        /// * `find` - a way to lookup new object data during traversal by their ObjectId, writing their data into buffer and returning
        ///    an iterator over commit tokens if the object is present and is a commit. Caching should be implemented within this function
        ///    as needed. The return value is `Option<CommitIter>` which degenerates all error information. Not finding a commit should also
        ///    be considered an errors as all objects in the commit graph should be present in the database. Hence [`Error::NotFound`] should
        ///    be escalated into a more specific error if its encountered by the caller.
        /// * `state` - all state used for the traversal. If multiple traversals are performed, allocations can be minimized by reusing
        ///   this state.
        /// * `tips`
        ///   * the starting points of the iteration, usually commits
        ///   * each commit they lead to will only be returned once, including the tip that started it
        pub fn new(tips: impl IntoIterator<Item = impl Into<ObjectId>>, state: StateMut, find: Find) -> Self {
            Self::filtered(tips, state, find, |_| true)
        }
    }

    impl<Find, Predicate, StateMut> Ancestors<Find, Predicate, StateMut>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<commit::CommitRefIter<'a>>,
        Predicate: FnMut(&oid) -> bool,
        StateMut: BorrowMut<State>,
    {
        /// Create a new instance with commit filtering enabled.
        ///
        /// * `find` - a way to lookup new object data during traversal by their ObjectId, writing their data into buffer and returning
        ///    an iterator over commit tokens if the object is present and is a commit. Caching should be implemented within this function
        ///    as needed. The return value is `Option<CommitIter>` which degenerates all error information. Not finding a commit should also
        ///    be considered an errors as all objects in the commit graph should be present in the database. Hence [`Error::NotFound`] should
        ///    be escalated into a more specific error if its encountered by the caller.
        /// * `state` - all state used for the traversal. If multiple traversals are performed, allocations can be minimized by reusing
        ///   this state.
        /// * `tips`
        ///   * the starting points of the iteration, usually commits
        ///   * each commit they lead to will only be returned once, including the tip that started it
        /// * `predicate` - indicate whether a given commit should be included in the result as well
        ///   as whether its parent commits should be traversed.
        pub fn filtered(
            tips: impl IntoIterator<Item = impl Into<ObjectId>>,
            mut state: StateMut,
            find: Find,
            mut predicate: Predicate,
        ) -> Self {
            let tips = tips.into_iter();
            {
                let state = state.borrow_mut();
                state.clear();
                state.next.reserve(tips.size_hint().0);
                for tip in tips.map(Into::into) {
                    let was_inserted = state.seen.insert(tip);
                    if was_inserted && predicate(&tip) {
                        state.next.push_back(tip);
                    }
                }
            }
            Self { find, predicate, state }
        }
    }

    impl<Find, Predicate, StateMut> Iterator for Ancestors<Find, Predicate, StateMut>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<commit::CommitRefIter<'a>>,
        Predicate: FnMut(&oid) -> bool,
        StateMut: BorrowMut<State>,
    {
        type Item = Result<ObjectId, Error>;

        fn next(&mut self) -> Option<Self::Item> {
            let state = self.state.borrow_mut();
            let res = state.next.pop_front();
            if let Some(oid) = res {
                match (self.find)(&oid, &mut state.buf) {
                    Some(mut commit_iter) => {
                        if let Some(Err(decode_tree_err)) = commit_iter.next() {
                            return Some(Err(decode_tree_err.into()));
                        }
                        for token in commit_iter {
                            match token {
                                Ok(git_object::commit::ref_iter::Token::Parent { id }) => {
                                    let was_inserted = state.seen.insert(id);
                                    if was_inserted && (self.predicate)(&id) {
                                        state.next.push_back(id);
                                    }
                                }
                                Ok(_a_token_past_the_parents) => break,
                                Err(err) => return Some(Err(err.into())),
                            }
                        }
                    }
                    None => return Some(Err(Error::NotFound { oid })),
                }
            }
            res.map(Ok)
        }
    }
}
#[doc(inline)]
pub use ancestors::Ancestors;
