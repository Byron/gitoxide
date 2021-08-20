pub(crate) mod object {
    use std::ops::DerefMut;

    use git_hash::ObjectId;

    use crate::{
        easy, object,
        odb::{Find, FindExt},
        ObjectRef,
    };

    pub fn find_object<A: easy::Access + Sized>(
        access: &A,
        id: impl Into<ObjectId>,
    ) -> Result<ObjectRef<'_, A>, object::find::existing::Error> {
        let state = access.state();
        let id = id.into();
        let kind = {
            let mut buf = access.state().try_borrow_mut_buf()?;
            let obj =
                access
                    .repo()?
                    .odb
                    .find_existing(&id, &mut buf, state.try_borrow_mut_pack_cache()?.deref_mut())?;
            obj.kind
        };

        ObjectRef::from_current_buf(id, kind, access).map_err(Into::into)
    }

    pub fn try_find_object<A: easy::Access + Sized>(
        access: &A,
        id: impl Into<ObjectId>,
    ) -> Result<Option<ObjectRef<'_, A>>, object::find::Error> {
        let state = access.state();
        let id = id.into();
        access
            .repo()?
            .odb
            .find(
                &id,
                state.try_borrow_mut_buf()?.deref_mut(),
                state.try_borrow_mut_pack_cache()?.deref_mut(),
            )?
            .map(|obj| {
                let kind = obj.kind;
                drop(obj);
                ObjectRef::from_current_buf(id, kind, access).map_err(Into::into)
            })
            .transpose()
    }

    pub trait ObjectAccessExt: easy::Access + Sized {
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

    use git_hash::ObjectId;

    use crate::{
        actor, easy, lock, reference,
        refs::{
            file::find::Error,
            mutable::Target,
            transaction::{Change, Create, RefEdit},
            PartialName,
        },
        Reference,
    };

    /// Obtain and alter references comfortably
    pub trait ReferenceAccessExt: easy::Access + Sized {
        fn tag(
            &self,
            name: impl AsRef<str>,
            target: impl Into<ObjectId>,
            lock_mode: lock::acquire::Fail,
            force: bool,
        ) -> Result<Vec<RefEdit>, reference::edit::Error> {
            self.edit_references(
                Some(RefEdit {
                    change: Change::Update {
                        log: Default::default(),
                        mode: if force {
                            Create::OrUpdate { previous: None }
                        } else {
                            Create::Only
                        },
                        new: Target::Peeled(target.into()),
                    },
                    name: format!("tags/refs/{}", name.as_ref()).try_into()?,
                    deref: false,
                }),
                lock_mode,
                None,
            )
        }

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
            self.repo()?
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
            let state = self.state();
            state.assure_packed_refs_present(&self.repo()?.refs)?;
            match self.repo()?.refs.find(name, state.try_borrow_packed_refs()?.as_ref()) {
                Ok(r) => match r {
                    Some(r) => Ok(Some(Reference::from_file_ref(r, self))),
                    None => Ok(None),
                },
                Err(err) => Err(err.into()),
            }
        }
    }

    impl<A> ReferenceAccessExt for A where A: easy::Access + Sized {}
}
