use std::ops::DerefMut;

use crate::easy;

/// Configure how caches are used to speed up various git repository operations
pub trait CacheAccessExt: easy::Access + Sized {
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
            bytes.map(easy::object::cache::MemoryCappedHashmap::new),
        )
        .map(|c| c.capacity()))
    }

    /// Set the cache for speeding up pack access to `cache`, and return the previous set cache.
    ///
    /// It can be unset by using `git_repository::odb::pack::cache::Never`.
    #[cfg(all(feature = "unstable", feature = "max-performance"))]
    fn pack_cache(
        &self,
        cache: impl git_pack::cache::DecodeEntry + Send + 'static,
    ) -> easy::borrow::state::Result<Box<dyn git_pack::cache::DecodeEntry + Send + 'static>> {
        Ok(std::mem::replace(
            self.state().try_borrow_mut_pack_cache()?.deref_mut(),
            Box::new(cache),
        ))
    }
}

impl<A> CacheAccessExt for A where A: easy::Access + Sized {}
