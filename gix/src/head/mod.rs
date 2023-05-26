//!
use std::convert::TryInto;

use gix_hash::ObjectId;
use gix_ref::FullNameRef;

use crate::{
    ext::{ObjectIdExt, ReferenceExt},
    Head,
};

/// Represents the kind of `HEAD` reference.
#[derive(Clone)]
pub enum Kind {
    /// The existing reference the symbolic HEAD points to.
    ///
    /// This is the common case.
    Symbolic(gix_ref::Reference),
    /// The yet-to-be-created reference the symbolic HEAD refers to.
    ///
    /// This is the case in a newly initialized repository.
    Unborn(gix_ref::FullName),
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
    /// Attach this instance to a `repo` to produce a [`Head`].
    pub fn attach(self, repo: &crate::Repository) -> Head<'_> {
        Head { kind: self, repo }
    }
}

/// Access
impl<'repo> Head<'repo> {
    /// Returns the name of this references, always `HEAD`.
    pub fn name(&self) -> &'static FullNameRef {
        // TODO: use a statically checked version of this when available.
        "HEAD".try_into().expect("HEAD is valid")
    }

    /// Returns the full reference name of this head if it is not detached, or `None` otherwise.
    pub fn referent_name(&self) -> Option<&FullNameRef> {
        Some(match &self.kind {
            Kind::Symbolic(r) => r.name.as_ref(),
            Kind::Unborn(name) => name.as_ref(),
            Kind::Detached { .. } => return None,
        })
    }

    /// Returns true if this instance is detached, and points to an object directly.
    pub fn is_detached(&self) -> bool {
        matches!(self.kind, Kind::Detached { .. })
    }

    /// Returns true if this instance is not yet born, hence it points to a ref that doesn't exist yet.
    ///
    /// This is the case in a newly initialized repository.
    pub fn is_unborn(&self) -> bool {
        matches!(self.kind, Kind::Unborn(_))
    }

    // TODO: tests
    /// Returns the id the head points to, which isn't possible on unborn heads.
    pub fn id(&self) -> Option<crate::Id<'repo>> {
        match &self.kind {
            Kind::Symbolic(r) => r.target.try_id().map(|oid| oid.to_owned().attach(self.repo)),
            Kind::Detached { peeled, target } => (*peeled).unwrap_or(*target).attach(self.repo).into(),
            Kind::Unborn(_) => None,
        }
    }

    /// Try to transform this instance into the symbolic reference that it points to, or return `None` if head is detached or unborn.
    pub fn try_into_referent(self) -> Option<crate::Reference<'repo>> {
        match self.kind {
            Kind::Symbolic(r) => r.attach(self.repo).into(),
            _ => None,
        }
    }
}

mod remote {
    use super::Head;
    use crate::{remote, Remote};

    /// Remote
    impl<'repo> Head<'repo> {
        /// Return the remote with which the currently checked our reference can be handled as configured by `branch.<name>.remote|pushRemote`
        /// or fall back to the non-branch specific remote configuration. `None` is returned if the head is detached or unborn, so there is
        /// no branch specific remote.
        ///
        /// This is equivalent to calling [`Reference::remote(…)`][crate::Reference::remote()] and
        /// [`Repository::remote_default_name()`][crate::Repository::remote_default_name()] in order.
        ///
        /// Combine it with [`find_default_remote()`][crate::Repository::find_default_remote()] as fallback to handle detached heads,
        /// i.e. obtain a remote even in case of detached heads.
        pub fn into_remote(
            self,
            direction: remote::Direction,
        ) -> Option<Result<Remote<'repo>, remote::find::existing::Error>> {
            let repo = self.repo;
            self.try_into_referent()?
                .remote(direction)
                .or_else(|| repo.find_default_remote(direction))
        }
    }
}

///
pub mod log;

///
pub mod peel;
