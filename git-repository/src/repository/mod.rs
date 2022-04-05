//!

/// Internal
impl crate::Repository {
    #[inline]
    pub(crate) fn free_buf(&self) -> Vec<u8> {
        self.bufs.borrow_mut().pop().unwrap_or_default()
    }

    /// This method is commonly called from the destructor of objects that previously claimed an entry
    /// in the free-list with `free_buf()`.
    /// They are welcome to take out the data themselves, for instance when the object is detached, to avoid
    /// it to be reclaimed.
    #[inline]
    pub(crate) fn reuse_buffer(&self, data: &mut Vec<u8>) {
        if data.capacity() > 0 {
            self.bufs.borrow_mut().push(std::mem::take(data));
        }
    }
}

/// Everything else
impl crate::Repository {
    // TODO: actual implementation
    /// Return the committer as configured by this repository, which is determined by…
    ///
    /// * …the git configuration…
    /// * …the GIT_(AUTHOR|COMMITTER)_(NAME|EMAIL|DATE) environment variables…
    ///
    /// …and in that order.
    pub fn committer(&self) -> git_actor::Signature {
        // TODO: actually do the work, probably that should be cached and be refreshable
        git_actor::Signature::empty()
    }

    /// The kind of object hash the repository is configured to use.
    pub fn object_hash(&self) -> git_hash::Kind {
        self.config.object_hash
    }
}

mod init {
    use std::cell::RefCell;

    impl crate::Repository {
        pub(crate) fn from_refs_and_objects(
            refs: crate::RefStore,
            objects: crate::OdbHandle,
            work_tree: Option<std::path::PathBuf>,
            config: crate::config::Cache,
        ) -> Self {
            crate::Repository {
                bufs: RefCell::new(Vec::with_capacity(4)),
                work_tree,
                objects: {
                    #[cfg(feature = "max-performance")]
                    {
                        objects.with_pack_cache(|| Box::new(git_pack::cache::lru::StaticLinkedList::<64>::default()))
                    }
                    #[cfg(not(feature = "max-performance"))]
                    {
                        objects
                    }
                },
                refs,
                config,
            }
        }

        /// Convert this instance into a [`ThreadSafeRepository`][crate::ThreadSafeRepository] by dropping all thread-local data.
        pub fn into_sync(self) -> crate::ThreadSafeRepository {
            self.into()
        }
    }
}

mod location;

mod snapshots;

mod impls;

mod cache;

mod reference;

mod object;

mod thread_safe;
