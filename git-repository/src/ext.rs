mod tree {
    use git_hash::oid;
    use git_object::immutable;
    use git_traverse::tree::breadthfirst;
    use std::borrow::BorrowMut;

    pub trait Sealed {}

    pub trait TreeExt: Sealed {
        fn changes_needed_to_obtain_with_state<FindFn, R, StateMut>(
            &self,
            other: immutable::TreeIter<'_>,
            state: StateMut,
            find: FindFn,
            delegate: &mut R,
        ) -> Result<(), git_diff::tree::changes::Error>
        where
            FindFn: for<'b> FnMut(&oid, &'b mut Vec<u8>) -> Option<immutable::tree::TreeIter<'b>>,
            R: git_diff::tree::Visit,
            StateMut: BorrowMut<git_diff::tree::State>;

        /// Use this for squeezing out the last bits of performance.
        fn traverse_with_state<StateMut, Find, V>(
            &self,
            state: StateMut,
            find: Find,
            delegate: &mut V,
        ) -> Result<(), breadthfirst::Error>
        where
            Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<immutable::TreeIter<'a>>,
            StateMut: BorrowMut<breadthfirst::State>,
            V: git_traverse::tree::Visit;
    }

    impl<'d> Sealed for immutable::TreeIter<'d> {}

    impl<'d> TreeExt for immutable::TreeIter<'d> {
        fn changes_needed_to_obtain_with_state<FindFn, R, StateMut>(
            &self,
            other: immutable::TreeIter<'_>,
            state: StateMut,
            find: FindFn,
            delegate: &mut R,
        ) -> Result<(), git_diff::tree::changes::Error>
        where
            FindFn: for<'b> FnMut(&oid, &'b mut Vec<u8>) -> Option<immutable::tree::TreeIter<'b>>,
            R: git_diff::tree::Visit,
            StateMut: BorrowMut<git_diff::tree::State>,
        {
            git_diff::tree::Changes::from(Some(self.clone())).needed_to_obtain(other, state, find, delegate)
        }

        fn traverse_with_state<StateMut, Find, V>(
            &self,
            state: StateMut,
            find: Find,
            delegate: &mut V,
        ) -> Result<(), breadthfirst::Error>
        where
            Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<immutable::TreeIter<'a>>,
            StateMut: BorrowMut<breadthfirst::State>,
            V: git_traverse::tree::Visit,
        {
            breadthfirst(self.clone(), state, find, delegate)
        }
    }
}
pub use tree::TreeExt;

mod object_id {
    use git_hash::{oid, ObjectId};
    use git_object::immutable;
    use git_traverse::commit::ancestors::{Ancestors, State};

    pub trait Sealed {}

    pub trait ObjectIdExt: Sealed {
        fn ancestors_iter<Find>(self, find: Find) -> Ancestors<Find, fn(&oid) -> bool, State>
        where
            Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<immutable::CommitIter<'a>>;
    }

    impl Sealed for ObjectId {}
    impl ObjectIdExt for ObjectId {
        fn ancestors_iter<Find>(self, find: Find) -> Ancestors<Find, fn(&oid) -> bool, State>
        where
            Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<immutable::CommitIter<'a>>,
        {
            Ancestors::new(Some(self), State::default(), find)
        }
    }
}
pub use object_id::ObjectIdExt;
