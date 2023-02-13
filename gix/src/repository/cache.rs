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

    /// Set an object cache of size `bytes` if none is set.
    ///
    /// Use this method to avoid overwriting any existing value while assuring better performance in case no value is set.
    pub fn object_cache_size_if_unset(&mut self, bytes: usize) {
        if !self.objects.has_object_cache() {
            self.object_cache_size(bytes)
        }
    }
}
