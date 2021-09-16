use std::ops::DerefMut;

use crate::easy;

/// The catch-all of extension traits.
pub trait RepositoryAccessExt: easy::Access + Sized {
    // TODO: actual implementation
    /// Return the committer as configured by this repository, which is determined by…
    ///
    /// * …the git configuration…
    /// * …the GIT_(AUTHOR|COMMITTER)_(NAME|EMAIL|DATE) environment variables…
    ///
    /// …and in that order.
    fn committer(&self) -> easy::borrow::repo::Result<git_actor::Signature> {
        // TODO: actually do the work, probably that should be cached and be refreshable
        Ok(git_actor::Signature::empty())
    }

    /// The kind of hash the repository is configured to use.
    fn hash_kind(&self) -> easy::borrow::repo::Result<git_hash::Kind> {
        self.repo().map(|r| r.hash_kind)
    }

    /// Refresh persistent object database structures to reflect the state on disk.
    fn refresh_object_database(&self) -> Result<(), easy::odb::refresh::Error> {
        self.repo_mut()?.deref_mut().odb.refresh()?;
        Ok(())
    }
}

impl<A> RepositoryAccessExt for A where A: easy::Access + Sized {}
