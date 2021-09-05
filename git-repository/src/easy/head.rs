#![allow(missing_docs)]

use git_hash::ObjectId;
use git_ref::FullNameRef;

use crate::{easy, easy::Head, ext::ReferenceExt};

pub enum Kind {
    /// The existing reference the symbolic HEAD points to.
    Symbolic(git_ref::Reference),
    /// The not-yet-existing reference the symbolic HEAD refers to.
    Unborn(git_ref::FullName),
    Detached {
        target: ObjectId,
        peeled: Option<ObjectId>,
    },
}

impl Kind {
    pub fn attach<A>(self, access: &A) -> Head<'_, A> {
        Head { kind: self, access }
    }
}

impl<'repo, A> Head<'repo, A> {
    pub fn name(&self) -> Option<FullNameRef<'_>> {
        Some(match &self.kind {
            Kind::Symbolic(r) => r.name.to_ref(),
            Kind::Unborn(name) => name.to_ref(),
            Kind::Detached { .. } => return None,
        })
    }
    pub fn is_detached(&self) -> bool {
        matches!(self.kind, Kind::Detached { .. })
    }
}

impl<'repo, A> Head<'repo, A>
where
    A: easy::Access + Sized,
{
    pub fn into_referent(self) -> easy::Reference<'repo, A> {
        match self.kind {
            Kind::Symbolic(r) => r.attach(self.access),
            _ => panic!("BUG: Expected head to be a born symbolic reference"),
        }
    }
}

pub mod log {
    use std::marker::PhantomData;

    use crate::{
        easy,
        easy::{ext::ReferenceAccessExt, Head},
    };

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        BorrowState(#[from] easy::borrow::state::Error),
        #[error(transparent)]
        FindExistingReference(#[from] easy::reference::find::existing::Error),
    }

    impl<'repo, A> Head<'repo, A>
    where
        A: easy::Access + Sized,
    {
        pub fn log(&self) -> Result<easy::reference::log::State<'repo, A, easy::Reference<'repo, A>>, Error> {
            Ok(easy::reference::log::State {
                reference: self.access.find_reference("HEAD")?,
                buf: self.access.state().try_borrow_mut_buf()?,
                _phantom: PhantomData::default(),
            })
        }
    }
}

pub mod peel {
    use git_hash::ObjectId;

    use crate::{
        easy::{head::Kind, Access, Head},
        ext::{ObjectIdExt, ReferenceExt},
    };

    mod error {
        use crate::easy::{object, reference};
        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error(transparent)]
            FindExistingObject(#[from] object::find::existing::Error),
            #[error(transparent)]
            PeelReference(#[from] reference::peel_to_id_in_place::Error),
        }
    }
    pub use error::Error;

    impl<'repo, A> Head<'repo, A>
    where
        A: Access + Sized,
    {
        // TODO: tests
        pub fn peel_to_id_in_place(&mut self) -> Option<Result<ObjectId, Error>> {
            Some(match &mut self.kind {
                Kind::Unborn(_name) => return None,
                Kind::Detached {
                    peeled: Some(peeled), ..
                } => Ok(*peeled),
                Kind::Detached { peeled: None, target } => {
                    match target
                        .attach(self.access)
                        .object()
                        .map_err(Into::into)
                        .and_then(|obj| obj.peel_to_end().map_err(Into::into))
                        .map(|peeled| peeled.id)
                    {
                        Ok(peeled) => {
                            self.kind = Kind::Detached {
                                peeled: Some(peeled),
                                target: *target,
                            };
                            Ok(peeled)
                        }
                        Err(err) => Err(err),
                    }
                }
                Kind::Symbolic(r) => {
                    let mut nr = r.clone().attach(self.access);
                    let peeled = nr.peel_to_id_in_place().map_err(Into::into).map(|id| id.detach());
                    *r = nr.detach();
                    peeled
                }
            })
        }

        pub fn into_fully_peeled_id(self) -> Option<Result<ObjectId, Error>> {
            Some(match self.kind {
                Kind::Unborn(_name) => return None,
                Kind::Detached {
                    peeled: Some(peeled), ..
                } => Ok(peeled),
                Kind::Detached { peeled: None, target } => target
                    .attach(self.access)
                    .object()
                    .map_err(Into::into)
                    .and_then(|obj| obj.peel_to_end().map_err(Into::into))
                    .map(|peeled| peeled.id),
                Kind::Symbolic(r) => r
                    .attach(self.access)
                    .peel_to_id_in_place()
                    .map_err(Into::into)
                    .map(|id| id.detach()),
            })
        }
    }
}
