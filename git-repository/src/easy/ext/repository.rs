#![allow(missing_docs)]
use std::ops::DerefMut;

use crate::easy;

/// The catch-all of extension traits.
pub trait RepositoryAccessExt: easy::Access + Sized {
    // TODO: actual implementation
    fn committer(&self) -> git_actor::Signature {
        // TODO: actually read the committer information from git-config, probably it should be provided here
        git_actor::Signature::empty()
    }

    /// The kind of hash the repository is configured to use
    fn hash_kind(&self) -> Result<git_hash::Kind, easy::borrow::repo::Error> {
        self.repo().map(|r| r.hash_kind)
    }

    /// Refresh persistent object database structures to reflect the state on disk.
    fn refresh_object_database(&self) -> Result<(), easy::odb::refresh::Error> {
        self.repo_mut()?.deref_mut().odb.refresh()?;
        Ok(())
    }
}

impl<A> RepositoryAccessExt for A where A: easy::Access + Sized {}
