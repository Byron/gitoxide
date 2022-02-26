//!

use crate::easy;

/// Internal
impl easy::Repository {
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
impl easy::Repository {
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
        self.object_hash
    }
}

mod init {
    use crate::{easy, sync};
    use std::cell::RefCell;

    impl easy::Repository {
        pub(crate) fn from_refs_and_objects(
            refs: crate::RefStore,
            objects: crate::OdbHandle,
            object_hash: git_hash::Kind,
            work_tree: Option<std::path::PathBuf>,
            config: crate::Config,
        ) -> Self {
            easy::Repository {
                bufs: RefCell::new(Vec::with_capacity(4)),
                object_hash,
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

        /// Convert this instance into a [`SyncRepository`] by dropping all thread-local data.
        pub fn into_sync(self) -> sync::Handle {
            self.into()
        }
    }
}

mod location;

mod trait_impls;

mod cache;

mod reference;

mod object;
