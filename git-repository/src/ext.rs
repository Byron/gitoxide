mod tree {
    use git_hash::oid;
    use git_object::immutable;
    use git_traverse::tree::{
        breadthfirst,
        breadthfirst::{Error, State},
        Visit,
    };
    use std::borrow::BorrowMut;

    pub trait TreeExt {
        fn traverse<StateMut, Find, V>(&self, state: StateMut, find: Find, delegate: &mut V) -> Result<(), Error>
        where
            Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<immutable::TreeIter<'a>>,
            StateMut: BorrowMut<State>,
            V: Visit;
    }

    impl<'d> TreeExt for immutable::TreeIter<'d> {
        fn traverse<StateMut, Find, V>(&self, state: StateMut, find: Find, delegate: &mut V) -> Result<(), Error>
        where
            Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<immutable::TreeIter<'a>>,
            StateMut: BorrowMut<State>,
            V: Visit,
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

    pub trait ObjectIdExt {
        fn ancestors_iter<Find>(self, find: Find) -> Ancestors<Find, fn(&oid) -> bool, State>
        where
            Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<immutable::CommitIter<'a>>;
    }

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
