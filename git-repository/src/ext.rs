mod tree {
    use std::borrow::BorrowMut;

    use git_hash::oid;
    use git_object::immutable;
    use git_traverse::tree::breadthfirst;

    pub trait Sealed {}

    pub trait TreeIterExt: Sealed {
        fn changes_needed<FindFn, R, StateMut>(
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
        fn traverse<StateMut, Find, V>(
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

    impl<'d> TreeIterExt for immutable::TreeIter<'d> {
        fn changes_needed<FindFn, R, StateMut>(
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

        fn traverse<StateMut, Find, V>(
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
pub use tree::TreeIterExt;

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

pub(crate) mod access {
    pub(crate) mod reference {
        use std::{cell::RefCell, convert::TryInto};

        use crate::{
            hash::ObjectId,
            reference,
            reference::Backing,
            refs,
            refs::{file::find::Error, PartialName},
            Access, Reference, Repository,
        };

        /// Obtain and alter references comfortably
        pub trait ReferenceExt: Access + Sized {
            fn find_existing_reference<'a, Name, E>(
                &self,
                name: Name,
            ) -> Result<Reference<'_, Self>, reference::find::existing::Error>
            where
                Name: TryInto<PartialName<'a>, Error = E>,
                Error: From<E>,
            {
                Ok(self
                    .find_reference(name)?
                    .ok_or_else(|| reference::find::existing::Error::NotFound)?)
            }

            fn find_reference<'a, Name, E>(
                &self,
                name: Name,
            ) -> Result<Option<Reference<'_, Self>>, reference::find::Error>
            where
                Name: TryInto<PartialName<'a>, Error = E>,
                Error: From<E>,
            {
                let cache = self.cache();
                cache.assure_packed_refs_present(&self.repo().refs)?;
                match self.repo().refs.find(name, cache.packed_refs.borrow().as_ref()) {
                    Ok(r) => match r {
                        Some(r) => Ok(Some(Reference::from_ref(r, self))),
                        None => Ok(None),
                    },
                    Err(err) => Err(err.into()),
                }
            }
        }

        impl<A> ReferenceExt for A where A: Access + Sized {}
    }
}
