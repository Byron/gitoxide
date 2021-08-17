use std::{cell::RefCell, ops::DerefMut};

use crate::{hash::ObjectId, odb::Find, refs, refs::mutable, Access, Object, Reference, Repository};

pub(crate) enum Backing {
    OwnedPacked {
        /// The validated full name of the reference.
        name: mutable::FullName,
        /// The target object id of the reference, hex encoded.
        target: ObjectId,
        /// The fully peeled object id, hex encoded, that the ref is ultimately pointing to
        /// i.e. when all indirections are removed.
        object: Option<ObjectId>,
    },
    LooseFile(refs::file::loose::Reference),
}

pub mod peel_to_id_in_place {
    use quick_error::quick_error;

    use crate::refs;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            LoosePeelToId(err: refs::file::loose::reference::peel::to_id::Error) {
                display("Could not peel loose reference")
                from()
                source(err)
            }
            PackedRefsOpen(err: refs::packed::buffer::open::Error) {
                display("The packed-refs file could not be opened")
                from()
                source(err)
            }
        }
    }
}

impl<'repo, A> Reference<'repo, A>
where
    A: Access + Sized,
{
    pub(crate) fn from_ref(reference: refs::file::Reference<'_>, access: &'repo A) -> Self {
        Reference {
            backing: match reference {
                refs::file::Reference::Packed(p) => Backing::OwnedPacked {
                    name: p.name.into(),
                    target: p.target(),
                    object: p
                        .object
                        .map(|hex| ObjectId::from_hex(hex).expect("a hash kind we know")),
                },
                refs::file::Reference::Loose(l) => Backing::LooseFile(l),
            }
            .into(),
            access,
        }
    }
    pub fn target(&self) -> refs::mutable::Target {
        match self.backing.as_ref().expect("always set") {
            Backing::OwnedPacked { target, .. } => mutable::Target::Peeled(target.to_owned()),
            Backing::LooseFile(r) => r.target.clone(),
        }
    }

    pub fn name(&self) -> refs::FullName<'_> {
        match self.backing.as_ref().expect("always set") {
            Backing::OwnedPacked { name, .. } => &name,
            Backing::LooseFile(r) => &r.name,
        }
        .borrow()
    }

    pub fn peel_to_object_in_place(&mut self) -> Result<Object<'repo, A>, peel_to_id_in_place::Error> {
        let repo = self.access.repo();
        match self.backing.take().expect("a ref must be set") {
            Backing::LooseFile(mut r) => {
                let cache = self.access.cache();
                cache.assure_packed_refs_present(&repo.refs)?;
                let oid = r
                    .peel_to_id_in_place(&repo.refs, cache.packed_refs.borrow().as_ref(), |oid, buf| {
                        repo.odb
                            .find(oid, buf, cache.pack.borrow_mut().deref_mut())
                            .map(|po| po.map(|o| (o.kind, o.data)))
                    })?
                    .to_owned();
                self.backing = Backing::LooseFile(r).into();
                Ok(Object::try_from_oid(oid, self.access).unwrap())
            }
            Backing::OwnedPacked {
                mut target,
                mut object,
                name,
            } => {
                if let Some(peeled_id) = object.take() {
                    target = peeled_id;
                }
                self.backing = Backing::OwnedPacked {
                    name,
                    target,
                    object: None,
                }
                .into();
                Ok(Object::try_from_oid(target, self.access).unwrap())
            }
        }
    }
}

mod access {
    use std::{cell::RefCell, convert::TryInto};

    use crate::{
        hash::ObjectId,
        reference::Backing,
        refs,
        refs::{file::find::Error, PartialName},
        Access, Reference, Repository,
    };

    /// Obtain and alter references comfortably
    pub trait ReferencesExt: Access + Sized {
        fn find_reference<'a, Name, E>(
            &self,
            name: Name,
        ) -> Result<Option<Reference<'_, Self>>, crate::reference::find::Error>
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

    impl<A> ReferencesExt for A where A: Access + Sized {}
}

pub use access::ReferencesExt;

pub mod find {
    use quick_error::quick_error;

    use crate::refs;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Find(err: refs::file::find::Error) {
                display("An error occurred when trying to find a reference")
                from()
                source(err)
            }
            PackedRefsOpen(err: refs::packed::buffer::open::Error) {
                display("The packed-refs file could not be opened")
                from()
                source(err)
            }
        }
    }
}
