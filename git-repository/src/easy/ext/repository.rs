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

    /// Sets the amount of space used at most for caching most recently accessed fully decoded objects, to `Some(bytes)`,
    /// or `None` to deactivate it entirely.
    ///
    /// Note that it is unset by default well but can be enabled once there is time for performance optimization.
    /// Well-chosen cache sizes can improve performance particularly if objects are accessed multiple times in a row.
    /// The cache is configured to grow gradually.
    ///
    /// Note that a cache on application level should be considered as well as the best object access is not doing one.
    ///
    /// Returns the previous cache size.
    fn object_cache_size(&self, bytes: impl Into<Option<usize>>) -> easy::borrow::state::Result<Option<usize>> {
        let bytes = bytes.into();
        Ok(std::mem::replace(
            self.state().try_borrow_mut_object_cache()?.deref_mut(),
            bytes.map(|bytes| easy::object::cache::MemoryCappedHashmap::new(bytes)),
        )
        .map(|c| c.capacity()))
    }

    /// Set the cache for speeding up pack access to `cache`, and return the previous set cache.
    ///
    /// It can be unset by using `git_repository::odb::pack::cache::Never`.
    #[cfg(all(feature = "unstable", feature = "max-performance"))]
    fn package_cache(
        &self,
        cache: impl git_pack::cache::DecodeEntry + Send + 'static,
    ) -> easy::borrow::state::Result<Box<dyn git_pack::cache::DecodeEntry + Send + 'static>> {
        Ok(std::mem::replace(
            self.state().try_borrow_mut_pack_cache()?.deref_mut(),
            Box::new(cache),
        ))
    }
}

impl<A> RepositoryAccessExt for A where A: easy::Access + Sized {}
