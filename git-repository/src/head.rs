//!
use git_hash::ObjectId;
use git_ref::FullNameRef;

use crate::{
    ext::{ObjectIdExt, ReferenceExt},
    Head,
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
    /// Attach this instance with an [access][sync::Handle] reference to produce a [`Head`].
    pub fn attach(self, handle: &crate::Repository) -> Head<'_> {
        Head { kind: self, handle }
    }
}

impl<'repo> Head<'repo> {
    /// Returns the name of this references, always `HEAD`.
    pub fn name(&self) -> FullNameRef<'static> {
        // TODO: use a statically checked version of this when available.
        use std::convert::TryFrom;
        FullNameRef::try_from("HEAD").expect("HEAD is valid")
    }

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

    // TODO: tests
    /// Returns the id the head points to, which isn't possible on unborn heads.
    pub fn id(&self) -> Option<crate::Id<'repo>> {
        match &self.kind {
            Kind::Symbolic(r) => r.target.as_id().map(|oid| oid.to_owned().attach(self.handle)),
            Kind::Detached { peeled, target } => (*peeled)
                .unwrap_or_else(|| target.to_owned())
                .attach(self.handle)
                .into(),
            Kind::Unborn(_) => None,
        }
    }

    /// Force transforming this instance into the symbolic reference that it points to, or panic if it is unborn or detached.
    ///
    /// # Panics
    ///
    /// If this isn't actually a head pointing to a symbolic reference.
    pub fn into_referent(self) -> crate::Reference<'repo> {
        match self.kind {
            Kind::Symbolic(r) => r.attach(self.handle),
            _ => panic!("BUG: Expected head to be a born symbolic reference"),
        }
    }
}
///
pub mod log {
    use std::convert::TryFrom;

    use git_ref::FullNameRef;

    use crate::Head;

    impl<'repo> Head<'repo> {
        /// Return a platform for obtaining iterators on the reference log associated with the `HEAD` reference.
        pub fn log_iter(&self) -> git_ref::file::log::iter::Platform<'static, '_> {
            git_ref::file::log::iter::Platform {
                store: &self.handle.refs,
                name: FullNameRef::try_from("HEAD").expect("HEAD is always valid"),
                buf: Vec::new(),
            }
        }
    }
}

///
pub mod peel {
    use crate::{
        ext::{ObjectIdExt, ReferenceExt},
        Head,
    };

    mod error {
        use crate::{object, reference};

        /// The error returned by [Head::peel_to_id_in_place()][super::Head::peel_to_id_in_place()] and [Head::into_fully_peeled_id()][super::Head::into_fully_peeled_id()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error(transparent)]
            FindExistingObject(#[from] object::find::existing::OdbError),
            #[error(transparent)]
            PeelReference(#[from] reference::peel::Error),
        }
    }
    use crate::head::Kind;
    pub use error::Error;

    mod peel_to_commit {
        use crate::object;

        /// The error returned by [Head::peel_to_commit_in_place()][super::Head::peel_to_commit_in_place()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error(transparent)]
            Peel(#[from] super::Error),
            #[error("Branch '{name}' does not have any commits")]
            Unborn { name: git_ref::FullName },
            #[error(transparent)]
            ObjectKind(#[from] object::try_into::Error),
        }
    }

    impl<'repo> Head<'repo> {
        // TODO: tests
        /// Peel this instance to make obtaining its final target id possible, while returning an error on unborn heads.
        pub fn peeled(mut self) -> Result<Self, Error> {
            self.peel_to_id_in_place().transpose()?;
            Ok(self)
        }

        // TODO: tests
        /// Follow the symbolic reference of this head until its target object and peel it by following tag objects until there is no
        /// more object to follow, and return that object id.
        ///
        /// Returns `None` if the head is unborn.
        pub fn peel_to_id_in_place(&mut self) -> Option<Result<crate::Id<'repo>, Error>> {
            Some(match &mut self.kind {
                Kind::Unborn(_name) => return None,
                Kind::Detached {
                    peeled: Some(peeled), ..
                } => Ok((*peeled).attach(self.handle)),
                Kind::Detached { peeled: None, target } => {
                    match target
                        .attach(self.handle)
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
                            Ok(peeled.attach(self.handle))
                        }
                        Err(err) => Err(err),
                    }
                }
                Kind::Symbolic(r) => {
                    let mut nr = r.clone().attach(self.handle);
                    let peeled = nr.peel_to_id_in_place().map_err(Into::into);
                    *r = nr.detach();
                    peeled
                }
            })
        }

        // TODO: tests
        // TODO: something similar in `crate::Reference`
        /// Follow the symbolic reference of this head until its target object and peel it by following tag objects until there is no
        /// more object to follow, transform the id into a commit if possible and return that.
        ///
        /// Returns an error if the head is unborn or if it doesn't point to a commit.
        pub fn peel_to_commit_in_place(&mut self) -> Result<crate::Commit<'repo>, peel_to_commit::Error> {
            let id = self
                .peel_to_id_in_place()
                .ok_or_else(|| peel_to_commit::Error::Unborn {
                    name: self.referent_name().expect("unborn").to_owned(),
                })??;
            id.object()
                .map_err(|err| peel_to_commit::Error::Peel(Error::FindExistingObject(err)))
                .and_then(|object| object.try_into_commit().map_err(Into::into))
        }

        /// Consume this instance and transform it into the final object that it points to, or `None` if the `HEAD`
        /// reference is yet to be born.
        pub fn into_fully_peeled_id(self) -> Option<Result<crate::Id<'repo>, Error>> {
            Some(match self.kind {
                Kind::Unborn(_name) => return None,
                Kind::Detached {
                    peeled: Some(peeled), ..
                } => Ok(peeled.attach(self.handle)),
                Kind::Detached { peeled: None, target } => target
                    .attach(self.handle)
                    .object()
                    .map_err(Into::into)
                    .and_then(|obj| obj.peel_tags_to_end().map_err(Into::into))
                    .map(|obj| obj.id.attach(self.handle)),
                Kind::Symbolic(r) => r.attach(self.handle).peel_to_id_in_place().map_err(Into::into),
            })
        }
    }
}
