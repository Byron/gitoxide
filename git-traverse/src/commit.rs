///
pub mod ancestors {
    use crate::commit::ancestors::predicates::AlwaysTrue;
    use git_hash::{oid, ObjectId};
    use git_object::immutable;
    use quick_error::quick_error;
    use std::{
        borrow::BorrowMut,
        collections::{BTreeSet, VecDeque},
    };

    quick_error! {
        /// The error is part of the item returned by the [Ancestors] iterator.
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            NotFound{oid: ObjectId} {
                display("The object {} could not be found", oid)
            }
            ObjectDecode(err: immutable::object::decode::Error) {
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

    impl<Find, StateMut> Ancestors<Find, predicates::AlwaysTrue, StateMut>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<immutable::CommitIter<'a>>,
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
            Self::filtered_inner(tips, state, find, AlwaysTrue {})
        }
    }

    impl<Find, Predicate, StateMut> Ancestors<Find, predicates::FnMutWrapper<Predicate>, StateMut>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<immutable::CommitIter<'a>>,
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
            state: StateMut,
            find: Find,
            predicate: Predicate,
        ) -> Self
        where
            Predicate: FnMut(&oid) -> bool,
        {
            Self::filtered_inner(tips, state, find, predicates::FnMutWrapper { predicate })
        }
    }

    impl<Find, Predicate, StateMut> Ancestors<Find, Predicate, StateMut>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<immutable::CommitIter<'a>>,
        Predicate: predicates::OidPredicate,
        StateMut: BorrowMut<State>,
    {
        fn filtered_inner(
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
                    if was_inserted && predicate.call_mut(&tip) {
                        state.next.push_back(tip);
                    }
                }
            }
            Self { find, predicate, state }
        }
    }

    impl<Find, Predicate, StateMut> Iterator for Ancestors<Find, Predicate, StateMut>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<immutable::CommitIter<'a>>,
        Predicate: predicates::OidPredicate,
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
                                Ok(immutable::commit::iter::Token::Parent { id }) => {
                                    let was_inserted = state.seen.insert(id);
                                    if was_inserted && self.predicate.call_mut(&id) {
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

    mod predicates {
        use super::*;

        /// `FnMut`-like trait used for pruning commit subgraphs from the ancestors iterator.
        ///
        /// This trait only exists because:
        /// 1. Predicate usage should be optional.
        ///    The user should opt-in to using predicates via an alternate constructor or via the
        ///    builder pattern.
        /// 2. The default case (include all ancestors) should not increase the size of the
        ///    [`Ancestors`] struct, nor should it require extra branches at runtime.
        ///    So don't use boxed closures, `Option<impl FnMut...>`, or `fn(&oid) -> bool`.
        ///    Instead, use a zero-sized type ([`AlwaysTrue`]) to handle the default case.
        /// 3. Directly implementing `FnMut` is not yet supported in stable Rust.
        /// 4. I could not get the compiler to accept [`Ancestors::new`] using a closure or static
        ///    function as the default predicate. I don't think this is possible without using
        ///    something like `typeof!(always_true)` (where `always_true` is a static function) as a
        ///    type parameter.
        pub trait OidPredicate {
            /// Indicate whether the given commit and its parents should be visited by the
            /// [`Ancestors`] iterator.
            ///
            /// Even if this method returns `false`, the commit's ancestors may still be visited if
            /// they are reachable from another commit for which this method returns `true`.
            /// If you still want to exclude those ancestors, then this method should also return
            /// `false` for those ancestor commits.
            fn call_mut(&mut self, oid: &oid) -> bool;
        }

        /// Default behavior to include all ancestor commits in the [`Ancestors`] iterator.
        pub struct AlwaysTrue;

        impl OidPredicate for AlwaysTrue {
            #[inline(always)]
            fn call_mut(&mut self, _: &oid) -> bool {
                true
            }
        }

        pub struct FnMutWrapper<P>
        where
            P: FnMut(&oid) -> bool,
        {
            pub predicate: P,
        }

        impl<P> OidPredicate for FnMutWrapper<P>
        where
            P: FnMut(&oid) -> bool,
        {
            #[inline(always)]
            fn call_mut(&mut self, oid: &oid) -> bool {
                (self.predicate)(oid)
            }
        }
    }
}
#[doc(inline)]
pub use ancestors::Ancestors;
