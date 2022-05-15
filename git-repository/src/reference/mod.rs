//!

use git_odb::pack::Find;
use git_ref::file::ReferenceExt;

use crate::{Id, Reference};

pub mod iter;

mod errors;
pub use errors::{edit, find, head_commit, head_id, peel};

use crate::ext::ObjectIdExt;

pub mod log;

pub use git_ref::{Category, Kind};

/// Access
impl<'repo> Reference<'repo> {
    /// Return the target to which this reference points to.
    pub fn target(&self) -> git_ref::TargetRef<'_> {
        self.inner.target.to_ref()
    }

    /// Return the reference's full name.
    pub fn name(&self) -> &git_ref::FullNameRef {
        self.inner.name.to_ref()
    }

    /// Turn this instances into a stand-alone reference.
    pub fn detach(self) -> git_ref::Reference {
        self.inner
    }
}

impl<'repo> Reference<'repo> {
    pub(crate) fn from_ref(reference: git_ref::Reference, repo: &'repo crate::Repository) -> Self {
        Reference { inner: reference, repo }
    }

    /// Returns the attached id we point to, or `None` if this is a symbolic ref.
    pub fn try_id(&self) -> Option<crate::Id<'repo>> {
        match self.inner.target {
            git_ref::Target::Symbolic(_) => None,
            git_ref::Target::Peeled(oid) => oid.to_owned().attach(self.repo).into(),
        }
    }

    /// Returns the attached id we point to, or panic if this is a symbolic ref.
    pub fn id(&self) -> crate::Id<'repo> {
        self.try_id()
            .expect("BUG: tries to obtain object id from symbolic target")
    }

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
