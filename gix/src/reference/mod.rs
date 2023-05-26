//!

use gix_odb::pack::Find;
use gix_ref::file::ReferenceExt;

use crate::{Id, Reference};

pub mod iter;
///
pub mod remote;

mod errors;
pub use errors::{edit, find, head_commit, head_id, peel};

use crate::ext::ObjectIdExt;

pub mod log;

pub use gix_ref::{Category, Kind};

/// Access
impl<'repo> Reference<'repo> {
    /// Returns the attached id we point to, or `None` if this is a symbolic ref.
    pub fn try_id(&self) -> Option<Id<'repo>> {
        match self.inner.target {
            gix_ref::Target::Symbolic(_) => None,
            gix_ref::Target::Peeled(oid) => oid.attach(self.repo).into(),
        }
    }

    /// Returns the attached id we point to, or panic if this is a symbolic ref.
    pub fn id(&self) -> Id<'repo> {
        self.try_id()
            .expect("BUG: tries to obtain object id from symbolic target")
    }

    /// Return the target to which this reference points to.
    pub fn target(&self) -> gix_ref::TargetRef<'_> {
        self.inner.target.to_ref()
    }

    /// Return the reference's full name.
    pub fn name(&self) -> &gix_ref::FullNameRef {
        self.inner.name.as_ref()
    }

    /// Turn this instances into a stand-alone reference.
    pub fn detach(self) -> gix_ref::Reference {
        self.inner
    }
}

impl<'repo> std::fmt::Debug for Reference<'repo> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.inner, f)
    }
}

impl<'repo> Reference<'repo> {
    pub(crate) fn from_ref(reference: gix_ref::Reference, repo: &'repo crate::Repository) -> Self {
        Reference { inner: reference, repo }
    }
}

impl<'repo> Reference<'repo> {
    /// Follow all symbolic targets this reference might point to and peel the underlying object
    /// to the end of the chain, and return it.
    ///
    /// This is useful to learn where this reference is ultimately pointing to.
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

mod edits;
pub use edits::{delete, set_target_id};
