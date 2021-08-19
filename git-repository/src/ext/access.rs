pub(crate) mod object {
    use std::ops::DerefMut;

    use git_hash::ObjectId;

    use crate::{
        object,
        odb::{Find, FindExt},
        Access, ObjectRef,
    };

    pub fn find_object<A: Access + Sized>(
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

    pub fn try_find_object<A: Access + Sized>(
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
        fn find_object(&self, id: impl Into<ObjectId>) -> Result<ObjectRef<'_, Self>, object::find::existing::Error> {
            find_object(self, id)
        }

        fn try_find_object(&self, id: impl Into<ObjectId>) -> Result<Option<ObjectRef<'_, Self>>, object::find::Error> {
            try_find_object(self, id)
        }
    }
}

pub(crate) mod reference {
    use std::convert::TryInto;

    use crate::{
        actor, lock, reference,
        refs::{file::find::Error, transaction::RefEdit, PartialName},
        Access, Reference,
    };

    /// Obtain and alter references comfortably
    pub trait ReferenceAccessExt: Access + Sized {
        fn edit_reference(
            &self,
            edit: RefEdit,
            lock_mode: lock::acquire::Fail,
            log_committer: Option<&actor::Signature>,
        ) -> Result<Vec<RefEdit>, reference::edit::Error> {
            self.edit_references(Some(edit), lock_mode, log_committer)
        }

        fn edit_references(
            &self,
            edits: impl IntoIterator<Item = RefEdit>,
            lock_mode: lock::acquire::Fail,
            log_committer: Option<&actor::Signature>,
        ) -> Result<Vec<RefEdit>, reference::edit::Error> {
            let committer_storage;
            let commiter = match log_committer {
                Some(c) => c,
                None => {
                    // TODO: actually read the committer information from git-config, probably it should be provided here
                    committer_storage = actor::Signature::empty();
                    &committer_storage
                }
            };
            self.repo()
                .refs
                .transaction()
                .prepare(edits, lock_mode)?
                .commit(commiter)
                .map_err(Into::into)
        }

        fn find_reference<'a, Name, E>(
            &self,
            name: Name,
        ) -> Result<Reference<'_, Self>, reference::find::existing::Error>
        where
            Name: TryInto<PartialName<'a>, Error = E>,
            Error: From<E>,
        {
            self.try_find_reference(name)?
                .ok_or(reference::find::existing::Error::NotFound)
        }

        fn try_find_reference<'a, Name, E>(
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
                    Some(r) => Ok(Some(Reference::from_file_ref(r, self))),
                    None => Ok(None),
                },
                Err(err) => Err(err.into()),
            }
        }
    }

    impl<A> ReferenceAccessExt for A where A: Access + Sized {}
}
