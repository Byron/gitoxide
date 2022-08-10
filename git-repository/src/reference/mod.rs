//!

use git_odb::pack::Find;
use git_ref::file::ReferenceExt;
use std::borrow::Cow;

use crate::{bstr, remote, Id, Reference};

pub mod iter;

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

    /// Find the name of our remote for `direction` as configured in `branch.<name>.remote|pushRemote` respectively.
    /// If `Some(<name>)` it can be used in [`Repository::find_remote(…)`][crate::Repository::find_remote()], or if `None` then
    /// [Repository::remote_default_name()][crate::Repository::remote_default_name()] could be used in its place.
    ///
    /// Return `None` if no remote is configured.
    ///
    /// # Note
    ///
    /// - it's recommended to use the [`remote(…)`][Self::remote()] method as it will configure the remote with additional
    ///   information.
    /// - `branch.<name>.pushRemote` falls back to `branch.<name>.remote`.
    pub fn remote_name(&self, direction: remote::Direction) -> Option<Cow<'repo, str>> {
        use bstr::{ByteSlice, ByteVec};
        let name = self.name().shorten().to_str().ok()?;
        (direction == remote::Direction::Push)
            .then(|| self.repo.config.resolved.string("branch", Some(name), "pushRemote"))
            .flatten()
            .or_else(|| self.repo.config.resolved.string("branch", Some(name), "remote"))
            .and_then(|name| match name {
                Cow::Borrowed(n) => n.to_str().ok().map(Cow::Borrowed),
                Cow::Owned(n) => Vec::from(n).into_string().ok().map(Cow::Owned),
            })
    }

    /// Like [`remote_name(…)`][Self::remote_name()], but configures the returned `Remote` with additional information like
    ///
    /// - `branch.<name>.merge` to know which branch on the remote side corresponds to this one for merging when pulling.
    pub fn remote(
        &self,
        direction: remote::Direction,
    ) -> Option<Result<crate::Remote<'repo>, remote::find::existing::Error>> {
        let name = self.remote_name(direction)?;
        // TODO: use `branch.<name>.merge`
        self.repo.find_remote(name.as_ref()).into()
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
