use crate::easy;

/// Configure how caches are used to speed up various git repository operations
impl easy::Handle {
    /// Sets the amount of space used at most for caching most recently accessed fully decoded objects, to `Some(bytes)`,
    /// or `None` to deactivate it entirely.
    ///
    /// Note that it is unset by default but can be enabled once there is time for performance optimization.
    /// Well-chosen cache sizes can improve performance particularly if objects are accessed multiple times in a row.
    /// The cache is configured to grow gradually.
    ///
    /// Note that a cache on application level should be considered as well as the best object access is not doing one.
    pub fn object_cache_size(&mut self, bytes: impl Into<Option<usize>>) {
        let bytes = bytes.into();
        match bytes {
            Some(bytes) => self
                .objects
                .set_object_cache(move || Box::new(easy::object::cache::MemoryCappedHashmap::new(bytes))),
            None => self.objects.without_object_cache(),
        }
    }

    /// Read well-known environment variables related to caches and apply them to this instance, but not to clones of it - each
    /// needs their own configuration.
    ///
    /// Note that environment configuration never fails due to invalid environment values, but it should be used with caution as it
    /// could be used to cause high memory consumption.
    ///
    /// Use the `GITOXIDE_DISABLE_PACK_CACHE` environment variable to turn off any pack cache, which can be beneficial when it's known that
    /// the cache efficiency is low. Use `GITOXIDE_PACK_CACHE_MEMORY=512MB` to use up to 512MB of RAM for the pack delta base
    /// cache. If none of these are set, the default cache is fast enough to nearly never cause a (marginal) slow-down while providing
    /// some gains most of the time. Note that the value given is _per-thread_.
    pub fn apply_environment(mut self) -> Self {
        self.objects.set_pack_cache(|| {
            #[cfg(not(feature = "max-performance"))]
            {
                Box::new(git_pack::cache::Never)
            }
            #[cfg(feature = "max-performance")]
            {
                use std::convert::TryInto;
                if std::env::var_os("GITOXIDE_DISABLE_PACK_CACHE").is_some() {
                    Box::new(git_pack::cache::Never)
                } else if let Some(bytes) = std::env::var("GITOXIDE_PACK_CACHE_MEMORY")
                    .ok()
                    .and_then(|v| {
                        byte_unit::Byte::from_str(&v)
                            .map_err(|err| log::warn!("Failed to parse {:?} into byte unit for pack cache: {}", v, err))
                            .ok()
                    })
                    .and_then(|unit| {
                        unit.get_bytes()
                            .try_into()
                            .map_err(|err| {
                                log::warn!(
                            "Parsed bytes value is not representable as usize. Defaulting to standard pack cache: {}",
                            err
                        )
                            })
                            .ok()
                    })
                {
                    Box::new(git_pack::cache::lru::MemoryCappedHashmap::new(bytes))
                } else {
                    Box::new(git_pack::cache::lru::StaticLinkedList::<64>::default())
                }
            }
        });
        self
    }
}
