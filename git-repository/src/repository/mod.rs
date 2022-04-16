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

/// Various permissions for parts of git repositories.
pub mod permissions {
    use git_sec::permission::Resource;
    use git_sec::{Access, Trust};

    /// Permissions for the `.git` directory containing the repository.
    pub struct GitDir {
        /// If set, we can both read and write a git repository's git dir and by relation, its work tree(s).
        /// Otherwise, the git-dir will be rejected and any open operation fails.
        pub read_write: bool,
    }

    /// Permissions associated with various resources of a git repository
    pub struct Permissions {
        /// Control how a git-dir can be used.
        pub git_dir: Access<Resource, GitDir>,
    }

    impl Permissions {
        /// Return permissions similar to what git does when the repository isn't owned by the current user,
        /// thus refusing all operations in it.
        pub fn strict() -> Self {
            Permissions {
                git_dir: Access::resource(GitDir { read_write: false }),
            }
        }

        /// Return permissions that will not include configuration files not owned by the current user,
        /// but trust system and global configuration files along with those which are owned by the current user.
        ///
        /// This allows to read and write repositories even if they aren't owned by the current user, but avoid using
        /// anything else that could cause us to write into unknown locations or use programs beyond our `PATH`.
        pub fn secure() -> Self {
            Permissions {
                git_dir: Access::resource(GitDir { read_write: true }),
            }
        }

        /// Everything is allowed with this set of permissions, thus we read all configuration and do what git typically
        /// does with owned repositories.
        pub fn all() -> Self {
            Permissions {
                git_dir: Access::resource(GitDir { read_write: true }),
            }
        }
    }

    impl git_sec::trust::DefaultForLevel for Permissions {
        fn default_for_level(level: Trust) -> Self {
            match level {
                Trust::Full => Permissions::all(),
                Trust::Reduced => Permissions::secure(),
            }
        }
    }

    impl Default for Permissions {
        fn default() -> Self {
            Permissions::secure()
        }
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
