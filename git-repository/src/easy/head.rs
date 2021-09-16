//!
use git_hash::ObjectId;
use git_ref::FullNameRef;

use crate::{
    easy,
    easy::Head,
    ext::{ObjectIdExt, ReferenceExt},
};

/// Represents the kind of `HEAD` reference.
pub enum Kind {
    /// The existing reference the symbolic HEAD points to.
    ///
    /// This is the common case.
    Symbolic(git_ref::Reference),
    /// The yet-to-be-created reference the symbolic HEAD refers to.
    ///
    /// This is the case in a newly initialized repository.
    Unborn(git_ref::FullName),
    /// The head points to an object directly, not to a symbolic reference.
    ///
    /// This state is less common and can occur when checking out commits directly.
    Detached {
        /// The object to which the head points to
        target: ObjectId,
        /// Possibly the final destination of `target` after following the object chain from tag objects to commits.
        peeled: Option<ObjectId>,
    },
}

impl Kind {
    /// Attach this instance with an [access][easy::Access] reference to produce a [`Head`].
    pub fn attach<A>(self, access: &A) -> Head<'_, A> {
        Head { kind: self, access }
    }
}

impl<'repo, A> Head<'repo, A> {
    /// Returns the full reference name of this head if it is not detached, or `None` otherwise.
    pub fn referent_name(&self) -> Option<FullNameRef<'_>> {
        Some(match &self.kind {
            Kind::Symbolic(r) => r.name.to_ref(),
            Kind::Unborn(name) => name.to_ref(),
            Kind::Detached { .. } => return None,
        })
    }
    /// Returns true if this instance is detached, and points to an object directly.
    pub fn is_detached(&self) -> bool {
        matches!(self.kind, Kind::Detached { .. })
    }
}

impl<'repo, A> Head<'repo, A>
where
    A: easy::Access + Sized,
{
    // TODO: tests
    /// Returns the id the head points to, which isn't possible on unborn heads.
    pub fn id(&self) -> Option<easy::Oid<'repo, A>> {
        match &self.kind {
            Kind::Symbolic(r) => r.target.as_id().map(|oid| oid.to_owned().attach(self.access)),
            Kind::Detached { peeled, target } => (*peeled)
                .unwrap_or_else(|| target.to_owned())
                .attach(self.access)
                .into(),
            Kind::Unborn(_) => None,
        }
    }

    /// Force transforming this instance into the symbolic reference that it points to, or panic if it is unborn or detached.
    pub fn into_referent(self) -> easy::Reference<'repo, A> {
        match self.kind {
            Kind::Symbolic(r) => r.attach(self.access),
            _ => panic!("BUG: Expected head to be a born symbolic reference"),
        }
    }
}

///
pub mod log {
    use std::marker::PhantomData;

    use crate::{
        easy,
        easy::{ext::ReferenceAccessExt, Head},
    };

    /// The error returned by [Head::logs()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
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
        /// Return a platform for obtaining iterators on the reference log associated with the `HEAD` reference.
        pub fn logs(&self) -> Result<easy::reference::Logs<'repo, A, easy::Reference<'repo, A>>, Error> {
            Ok(easy::reference::Logs {
                reference: self.access.find_reference("HEAD")?,
                buf: self.access.state().try_borrow_mut_buf()?,
                _phantom: PhantomData::default(),
            })
        }
    }
}

///
pub mod peel {
    use crate::{
        easy,
        easy::{head::Kind, Access, Head},
        ext::{ObjectIdExt, ReferenceExt},
    };

    mod error {
        use crate::easy::{object, reference};

        /// The error returned by [Head::peel_to_id_in_place()][super::Head::peel_to_id_in_place()] and [Head::into_fully_peeled_id()][super::Head::into_fully_peeled_id()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error(transparent)]
            FindExistingObject(#[from] object::find::existing::Error),
            #[error(transparent)]
            PeelReference(#[from] reference::peel::Error),
        }
    }
    pub use error::Error;

    impl<'repo, A> Head<'repo, A>
    where
        A: Access + Sized,
    {
        // TODO: tests
        /// Peel this instance to make obtaining its final target id possible, while returning an error on unborn heads.
        pub fn peeled(mut self) -> Result<Self, Error> {
            self.peel_to_id_in_place().transpose()?;
            Ok(self)
        }

        // TODO: tests
        /// Follow the symbolic reference of this head until its target object and peel it by following tag objects there is no
        /// more object to follow, and return that object id.
        ///
        /// Returns `None` if the head is unborn.
        pub fn peel_to_id_in_place(&mut self) -> Option<Result<easy::Oid<'repo, A>, Error>> {
            Some(match &mut self.kind {
                Kind::Unborn(_name) => return None,
                Kind::Detached {
                    peeled: Some(peeled), ..
                } => Ok((*peeled).attach(self.access)),
                Kind::Detached { peeled: None, target } => {
                    match target
                        .attach(self.access)
                        .object()
                        .map_err(Into::into)
                        .and_then(|obj| obj.peel_tags_to_end().map_err(Into::into))
                        .map(|peeled| peeled.id)
                    {
                        Ok(peeled) => {
                            self.kind = Kind::Detached {
                                peeled: Some(peeled),
                                target: *target,
                            };
                            Ok(peeled.attach(self.access))
                        }
                        Err(err) => Err(err),
                    }
                }
                Kind::Symbolic(r) => {
                    let mut nr = r.clone().attach(self.access);
                    let peeled = nr.peel_to_id_in_place().map_err(Into::into);
                    *r = nr.detach();
                    peeled
                }
            })
        }

        /// Consume this instance and transform it into the final object that it points to, or `None` if the `HEAD`
        /// reference is yet to be born.
        pub fn into_fully_peeled_id(self) -> Option<Result<easy::Oid<'repo, A>, Error>> {
            Some(match self.kind {
                Kind::Unborn(_name) => return None,
                Kind::Detached {
                    peeled: Some(peeled), ..
                } => Ok(peeled.attach(self.access)),
                Kind::Detached { peeled: None, target } => target
                    .attach(self.access)
                    .object()
                    .map_err(Into::into)
                    .and_then(|obj| obj.peel_tags_to_end().map_err(Into::into))
                    .map(|obj| obj.id.attach(self.access)),
                Kind::Symbolic(r) => r.attach(self.access).peel_to_id_in_place().map_err(Into::into),
            })
        }
    }
}
