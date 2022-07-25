use std::cell::RefCell;

impl crate::Repository {
    pub(crate) fn from_refs_and_objects(
        refs: crate::RefStore,
        objects: crate::OdbHandle,
        work_tree: Option<std::path::PathBuf>,
        common_dir: Option<std::path::PathBuf>,
        config: crate::config::Cache,
        linked_worktree_options: crate::open::Options,
    ) -> Self {
        crate::Repository {
            bufs: RefCell::new(Vec::with_capacity(4)),
            work_tree,
            common_dir,
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
            options: linked_worktree_options,
        }
    }

    /// Convert this instance into a [`ThreadSafeRepository`][crate::ThreadSafeRepository] by dropping all thread-local data.
    pub fn into_sync(self) -> crate::ThreadSafeRepository {
        self.into()
    }
}
