mod tree {
    use std::borrow::BorrowMut;

    use git_hash::oid;
    use git_object::immutable;
    #[cfg(feature = "git-traverse")]
    use git_traverse::tree::breadthfirst;

    pub trait Sealed {}

    pub trait TreeIterExt: Sealed {
        #[cfg(feature = "git-diff")]
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
        #[cfg(feature = "git-traverse")]
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
        #[cfg(feature = "git-diff")]
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

        #[cfg(feature = "git-traverse")]
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
    #[cfg(feature = "git-traverse")]
    use git_traverse::commit::ancestors::{Ancestors, State};

    use crate::{Access, Oid};

    pub trait Sealed {}

    pub trait ObjectIdExt: Sealed {
        #[cfg(feature = "git-traverse")]
        fn ancestors_iter<Find>(self, find: Find) -> Ancestors<Find, fn(&oid) -> bool, State>
        where
            Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<immutable::CommitIter<'a>>;

        fn attach<A: Access + Sized>(self, access: &A) -> Oid<'_, A>;
    }

    impl Sealed for ObjectId {}
    impl ObjectIdExt for ObjectId {
        #[cfg(feature = "git-traverse")]
        fn ancestors_iter<Find>(self, find: Find) -> Ancestors<Find, fn(&oid) -> bool, State>
        where
            Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<immutable::CommitIter<'a>>,
        {
            Ancestors::new(Some(self), State::default(), find)
        }

        fn attach<A: Access + Sized>(self, access: &A) -> Oid<'_, A> {
            Oid::from_id(self, access)
        }
    }
}
pub use object_id::ObjectIdExt;

pub(crate) mod access {
    pub(crate) mod object {
        use std::ops::DerefMut;

        use git_hash::ObjectId;

        use crate::{
            object,
            odb::{Find, FindExt},
            Access, ObjectRef,
        };

        pub fn find_existing_object<A: Access + Sized>(
            access: &A,
            id: impl Into<ObjectId>,
        ) -> Result<ObjectRef<'_, A>, object::find::existing::Error> {
            let cache = access.cache();
            let id = id.into();
            let kind = {
                let mut buf = access.cache().buf.borrow_mut();
                let obj = access
                    .repo()
                    .odb
                    .find_existing(&id, &mut buf, cache.pack.borrow_mut().deref_mut())?;
                obj.kind
            };

            Ok(ObjectRef::from_current_buf(id, kind, access))
        }

        pub fn find_object<A: Access + Sized>(
            access: &A,
            id: impl Into<ObjectId>,
        ) -> Result<Option<ObjectRef<'_, A>>, object::find::Error> {
            let cache = access.cache();
            let id = id.into();
            Ok(access
                .repo()
                .odb
                .find(&id, &mut cache.buf.borrow_mut(), cache.pack.borrow_mut().deref_mut())?
                .map(|obj| {
                    let kind = obj.kind;
                    drop(obj);
                    ObjectRef::from_current_buf(id, kind, access)
                }))
        }

        pub trait ObjectAccessExt: Access + Sized {
            // NOTE: in order to get the actual kind of object, is must be fully decoded from storage in case of packs
            // even though partial decoding is possible for loose objects, it won't matter much here.
            fn find_existing_object(
                &self,
                id: impl Into<ObjectId>,
            ) -> Result<ObjectRef<'_, Self>, object::find::existing::Error> {
                find_existing_object(self, id)
            }

            fn find_object(&self, id: impl Into<ObjectId>) -> Result<Option<ObjectRef<'_, Self>>, object::find::Error> {
                find_object(self, id)
            }
        }
    }

    pub(crate) mod reference {
        use std::convert::TryInto;

        use crate::{
            reference,
            refs::{file::find::Error, PartialName},
            Access, Reference,
        };

        /// Obtain and alter references comfortably
        pub trait ReferenceAccessExt: Access + Sized {
            fn find_existing_reference<'a, Name, E>(
                &self,
                name: Name,
            ) -> Result<Reference<'_, Self>, reference::find::existing::Error>
            where
                Name: TryInto<PartialName<'a>, Error = E>,
                Error: From<E>,
            {
                self.find_reference(name)?
                    .ok_or(reference::find::existing::Error::NotFound)
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

        impl<A> ReferenceAccessExt for A where A: Access + Sized {}
    }
}
pub use access::{object::ObjectAccessExt, reference::ReferenceAccessExt};
