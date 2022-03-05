/// Configure how caches are used to speed up various git repository operations
impl crate::Repository {
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
                .set_object_cache(move || Box::new(crate::object::cache::MemoryCappedHashmap::new(bytes))),
            None => self.objects.unset_object_cache(),
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
    ///
    /// Use the `GITOXIDE_OBJECT_CACHE_MEMORY=16mb` to set the given amount of memory to store full objects, on a per-thread basis.
    pub fn apply_environment(self) -> Self {
        // We have no cache types available without this flag currently. Maybe this should change at some point.
        #[cfg(not(feature = "max-performance"))]
        return self;
        #[cfg(feature = "max-performance")]
        {
            let pack_cache_disabled = std::env::var_os("GITOXIDE_DISABLE_PACK_CACHE").is_some();
            let mut this = self;
            if !pack_cache_disabled {
                let bytes = parse_bytes_from_var("GITOXIDE_PACK_CACHE_MEMORY");
                let new_pack_cache = move || -> Box<git_odb::cache::PackCache> {
                    match bytes {
                        Some(bytes) => Box::new(git_pack::cache::lru::MemoryCappedHashmap::new(bytes)),
                        None => Box::new(git_pack::cache::lru::StaticLinkedList::<64>::default()),
                    }
                };
                this.objects.set_pack_cache(new_pack_cache);
            } else {
                this.objects.unset_pack_cache();
            }

            if let Some(bytes) = parse_bytes_from_var("GITOXIDE_OBJECT_CACHE_MEMORY") {
                this.objects
                    .set_object_cache(move || Box::new(git_pack::cache::object::MemoryCappedHashmap::new(bytes)));
            }
            this
        }
    }
}

#[cfg(feature = "max-performance")]
fn parse_bytes_from_var(name: &str) -> Option<usize> {
    std::env::var(name)
        .ok()
        .and_then(|v| {
            byte_unit::Byte::from_str(&v)
                .map_err(|err| log::warn!("Failed to parse {:?} into byte unit for pack cache: {}", v, err))
                .ok()
        })
        .and_then(|unit| {
            use std::convert::TryInto;
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
}
