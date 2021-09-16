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

    /// Read well-known environment variables related to caches and apply them to this instance, but not to clones of it - each
    /// needs their own configuration.
    ///
    /// Note that environment configuration never fails due to invalid environment values, but it should be used with caution as it
    /// could be used to cause high memory consumption.
    ///
    /// Use the `GITOXIDE_DISABLE_PACK_CACHE` environment variable to turn off any pack cache, which can be beneficial when it's known that
    /// the cache efficiency is low. Use `GITOXIDE_PACK_CACHE_MEMORY_IN_BYTES=512000` to use up to 512MB of RAM for the pack delta base
    /// cache. If none of these are set, the default cache is fast enough to nearly never cause a (marginal) slow-down while providing
    /// some gains most of the time. Note that the value given is _per-thread_.
    fn apply_environment(self) -> easy::borrow::state::Result<Self> {
        #[cfg(not(feature = "max-performance"))]
        let pack_cache = git_pack::cache::Never;
        #[cfg(feature = "max-performance")]
        let pack_cache: crate::easy::PackCache = {
            if std::env::var_os("GITOXIDE_DISABLE_PACK_CACHE").is_some() {
                Box::new(git_pack::cache::Never)
            } else if let Some(num_bytes) = std::env::var("GITOXIDE_PACK_CACHE_MEMORY_IN_BYTES")
                .ok()
                .and_then(|v| <usize as std::str::FromStr>::from_str(&v).ok())
            {
                Box::new(git_pack::cache::lru::MemoryCappedHashmap::new(num_bytes))
            } else {
                Box::new(git_pack::cache::lru::StaticLinkedList::<64>::default())
            }
        };
        *self.state().try_borrow_mut_pack_cache()? = pack_cache;
        Ok(self)
    }
}

impl<A> CacheAccessExt for A where A: easy::Access + Sized {}
