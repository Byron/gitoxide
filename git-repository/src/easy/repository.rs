//!

use crate::easy;

mod init {
    use crate::easy;
    use std::cell::RefCell;

    impl easy::Repository {
        pub(crate) fn from_refs_and_objects(
            refs: crate::RefStore,
            objects: crate::OdbHandle,
            object_hash: git_hash::Kind,
            work_tree: Option<std::path::PathBuf>,
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
            }
        }

        /// Convert this instance into a [`SyncRepository`] by dropping all thread-local data.
        pub fn into_sync(self) -> crate::SyncRepository {
            self.into()
        }
    }
}

mod location {
    use crate::easy;

    impl easy::Repository {
        /// Return the work tree containing all checked out files, if there is one.
        pub fn work_tree(&self) -> Option<&std::path::Path> {
            self.work_tree.as_deref()
        }

        /// Return the kind of repository, either bare or one with a work tree.
        pub fn kind(&self) -> crate::Kind {
            match self.work_tree {
                Some(_) => crate::Kind::WorkTree,
                None => crate::Kind::Bare,
            }
        }

        /// Return the path to the repository itself, containing objects, references, configuration, and more.
        ///
        /// Synonymous to [`path()`][Repository::path()].
        pub fn git_dir(&self) -> &std::path::Path {
            self.refs.base()
        }

        // TODO: tests
        /// Load the index file of this repository's workspace, if present.
        ///
        /// Note that it is loaded into memory each time this method is called, but also is independent of the workspace.
        #[cfg(feature = "git-index")]
        pub fn load_index(&self) -> Option<Result<git_index::File, git_index::file::init::Error>> {
            // TODO: choose better/correct options
            let opts = git_index::decode::Options {
                object_hash: self.object_hash,
                thread_limit: None,
                min_extension_block_in_bytes_for_threading: 1024 * 256,
            };
            match git_index::File::at(self.git_dir().join("index"), opts) {
                Ok(index) => Some(Ok(index)),
                Err(git_index::file::init::Error::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => None,
                Err(err) => Some(Err(err)),
            }
        }
    }
}

mod impls {
    use crate::easy;

    impl Clone for easy::Repository {
        fn clone(&self) -> Self {
            easy::Repository::from_refs_and_objects(
                self.refs.clone(),
                self.objects.clone(),
                self.object_hash,
                self.work_tree.clone(),
            )
        }
    }

    impl std::fmt::Debug for easy::Repository {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Repository(git = '{}', working_tree: {:?}",
                self.git_dir().display(),
                self.work_tree
            )
        }
    }

    impl PartialEq<easy::Repository> for easy::Repository {
        fn eq(&self, other: &easy::Repository) -> bool {
            self.git_dir() == other.git_dir() && self.work_tree == other.work_tree
        }
    }

    impl From<&crate::SyncRepository> for easy::Repository {
        fn from(repo: &crate::SyncRepository) -> Self {
            easy::Repository::from_refs_and_objects(
                repo.refs.clone(),
                repo.objects.to_handle().into(),
                repo.object_hash,
                repo.work_tree.clone(),
            )
        }
    }

    impl From<crate::SyncRepository> for easy::Repository {
        fn from(repo: crate::SyncRepository) -> Self {
            easy::Repository::from_refs_and_objects(
                repo.refs,
                repo.objects.to_handle().into(),
                repo.object_hash,
                repo.work_tree,
            )
        }
    }

    impl From<easy::Repository> for crate::SyncRepository {
        fn from(r: easy::Repository) -> Self {
            crate::SyncRepository {
                refs: r.refs,
                objects: r.objects.into_inner().store(),
                work_tree: r.work_tree,
                object_hash: r.object_hash,
            }
        }
    }
}

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
