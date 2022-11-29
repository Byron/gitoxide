use std::cell::RefCell;

impl crate::Repository {
    pub(crate) fn from_refs_and_objects(
        refs: crate::RefStore,
        objects: crate::OdbHandle,
        work_tree: Option<std::path::PathBuf>,
        common_dir: Option<std::path::PathBuf>,
        config: crate::config::Cache,
        linked_worktree_options: crate::open::Options,
        index: crate::worktree::IndexStorage,
    ) -> Self {
        let objects = setup_objects(objects, &config);
        crate::Repository {
            bufs: RefCell::new(Vec::with_capacity(4)),
            work_tree,
            common_dir,
            objects,
            refs,
            config,
            options: linked_worktree_options,
            index,
        }
    }

    /// Convert this instance into a [`ThreadSafeRepository`][crate::ThreadSafeRepository] by dropping all thread-local data.
    pub fn into_sync(self) -> crate::ThreadSafeRepository {
        self.into()
    }
}

#[cfg_attr(not(feature = "max-performance-safe"), allow(unused_variables, unused_mut))]
fn setup_objects(mut objects: crate::OdbHandle, config: &crate::config::Cache) -> crate::OdbHandle {
    #[cfg(feature = "max-performance-safe")]
    {
        match config.pack_cache_bytes {
            None => objects.set_pack_cache(|| Box::new(git_pack::cache::lru::StaticLinkedList::<64>::default())),
            Some(0) => objects.unset_pack_cache(),
            Some(bytes) => objects.set_pack_cache(move || -> Box<git_odb::cache::PackCache> {
                Box::new(git_pack::cache::lru::MemoryCappedHashmap::new(bytes))
            }),
        };
        if config.object_cache_bytes == 0 {
            objects.unset_object_cache();
        } else {
            let bytes = config.object_cache_bytes;
            objects.set_object_cache(move || Box::new(git_pack::cache::object::MemoryCappedHashmap::new(bytes)));
        }
        objects
    }
    #[cfg(not(feature = "max-performance-safe"))]
    {
        objects
    }
}
