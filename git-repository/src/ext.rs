#[cfg(feature = "one-stop-shop")]
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

#[cfg(feature = "one-stop-shop")]
pub use tree::TreeExt;
