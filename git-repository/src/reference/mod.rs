//!

use git_odb::pack::Find;
use git_ref::file::ReferenceExt;

use crate::{Id, Reference};

pub mod iter;
mod remote;

mod errors;
pub use errors::{edit, find, head_commit, head_id, peel};

use crate::ext::ObjectIdExt;

pub mod log;

pub use git_ref::{Category, Kind};

/// Access
impl<'repo> Reference<'repo> {
    /// Returns the attached id we point to, or `None` if this is a symbolic ref.
    pub fn try_id(&self) -> Option<Id<'repo>> {
        match self.inner.target {
            git_ref::Target::Symbolic(_) => None,
            git_ref::Target::Peeled(oid) => oid.to_owned().attach(self.repo).into(),
        }
    }

    /// Returns the attached id we point to, or panic if this is a symbolic ref.
    pub fn id(&self) -> Id<'repo> {
        self.try_id()
            .expect("BUG: tries to obtain object id from symbolic target")
    }

    /// Return the target to which this reference points to.
    pub fn target(&self) -> git_ref::TargetRef<'_> {
        self.inner.target.to_ref()
    }

    /// Return the reference's full name.
    pub fn name(&self) -> &git_ref::FullNameRef {
        self.inner.name.as_ref()
    }

    /// Turn this instances into a stand-alone reference.
    pub fn detach(self) -> git_ref::Reference {
        self.inner
    }
}

impl<'repo> std::fmt::Debug for Reference<'repo> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.inner, f)
    }
}

impl<'repo> Reference<'repo> {
    pub(crate) fn from_ref(reference: git_ref::Reference, repo: &'repo crate::Repository) -> Self {
        Reference { inner: reference, repo }
    }
}

impl<'repo> Reference<'repo> {
    /// Follow all symbolic targets this reference might point to and peel the underlying object
    /// to the end of the chain, and return it.
    ///
    /// This is useful to learn where this reference is ulitmately pointing to.
    pub fn peel_to_id_in_place(&mut self) -> Result<Id<'repo>, peel::Error> {
        let repo = &self.repo;
        let oid = self.inner.peel_to_id_in_place(&repo.refs, |oid, buf| {
            repo.objects
                .try_find(oid, buf)
                .map(|po| po.map(|(o, _l)| (o.kind, o.data)))
        })?;
        Ok(Id::from_id(oid, repo))
    }

    /// Similar to [`peel_to_id_in_place()`][Reference::peel_to_id_in_place()], but consumes this instance.
    pub fn into_fully_peeled_id(mut self) -> Result<Id<'repo>, peel::Error> {
        self.peel_to_id_in_place()
    }
}

mod set_target_id {
    use crate::bstr::BString;
    use crate::Reference;
    use git_ref::transaction::PreviousValue;
    use git_ref::Target;

    mod error {
        use git_ref::FullName;

        /// The error returned by [`Reference::set_target_id()`].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error("Cannot change symbolic reference {name:?} into a direct one by setting it to an id")]
            SymbolicReference { name: FullName },
            #[error(transparent)]
            ReferenceEdit(#[from] crate::reference::edit::Error),
        }
    }
    pub use error::Error;

    impl<'repo> Reference<'repo> {
        /// Set the id of this direct reference to `id` and use `reflog_message` for the reflog (if enabled in the repository).
        ///
        /// Note that the operation will fail on symbolic references, to change their type use the lower level reference database.
        /// Furthermore, refrain from using this method for more than a one-off change as it creates a transaction for each invocation.
        /// If multiple reference should be changed, use a transaction of the lower level reference database instead.
        pub fn set_target_id(
            &mut self,
            id: impl Into<git_hash::ObjectId>,
            reflog_message: impl Into<BString>,
        ) -> Result<(), Error> {
            match &self.inner.target {
                Target::Symbolic(name) => return Err(Error::SymbolicReference { name: name.clone() }),
                Target::Peeled(current_id) => {
                    let changed = self.repo.reference(
                        self.name(),
                        id,
                        PreviousValue::ExistingMustMatch(Target::Peeled(current_id.to_owned())),
                        reflog_message,
                    )?;
                    *self = changed;
                }
            }
            Ok(())
        }
    }
}
