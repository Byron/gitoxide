#![allow(dead_code, unused_variables, unreachable_code)]

mod odb {
    use arc_swap::ArcSwap;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU8, AtomicUsize};
    use std::sync::Arc;

    use git_hash::oid;
    use git_odb::pack::{bundle::Location, find::Entry};

    use crate::odb::policy::{load_indices, PackIndexMarker};

    pub mod policy {
        use std::sync::Arc;
        use std::{
            io,
            path::{Path, PathBuf},
        };

        use git_hash::oid;

        mod index_file {
            use crate::odb::policy;
            use std::sync::Arc;

            pub enum SingleOrMulti {
                Single {
                    index: Arc<git_pack::index::File>,
                    data: Option<Arc<git_pack::data::File>>,
                },
                Multi {
                    index: Arc<policy::MultiIndex>,
                    data: Vec<Option<Arc<git_pack::data::File>>>,
                },
            }
        }

        pub(crate) struct IndexForObjectInPack {
            /// The internal identifier of the pack itself, which either is referred to by an index or a multi-pack index.
            pub(crate) pack_id: PackId,
            /// The index of the object within the pack
            object_index_in_pack: u32,
        }

        /// An id to refer to an index file or a multipack index file
        pub type IndexId = usize;

        /// A way to load and refer to a pack uniquely, namespaced by their indexing mechanism, aka multi-pack or not.
        pub struct PackId {
            /// Note that if `multipack_index = None`, this index is corresponding to the index id.
            /// So a pack is always identified by its corresponding index.
            /// If it is a multipack index, this is the id / offset of the pack in the `multipack_index`.
            pub(crate) index: IndexId,
            pub(crate) multipack_index: Option<IndexId>,
        }

        pub(crate) struct IndexLookup {
            file: index_file::SingleOrMulti,
            id: IndexId,
        }

        impl IndexLookup {
            /// See if the oid is contained in this index, and return its full id for lookup possibly alongside its data file if already
            /// loaded.
            /// If it is not loaded, ask it to be loaded and put it into the returned mutable option for safe-keeping.
            fn lookup(
                &mut self,
                object_id: &oid,
            ) -> Option<(IndexForObjectInPack, &mut Option<Arc<git_pack::data::File>>)> {
                match &mut self.file {
                    index_file::SingleOrMulti::Single { index, data } => {
                        index.lookup(object_id).map(|object_index_in_pack| {
                            (
                                IndexForObjectInPack {
                                    pack_id: PackId {
                                        index: self.id,
                                        multipack_index: None,
                                    },
                                    object_index_in_pack,
                                },
                                data,
                            )
                        })
                    }
                    index_file::SingleOrMulti::Multi { index, data } => {
                        todo!("find respective pack and return it as &mut Option<>")
                    }
                }
            }
        }

        #[derive(Clone)]
        pub(crate) struct OnDiskFile<T: Clone> {
            /// The last known path of the file
            path: Arc<PathBuf>,
            state: OnDiskFileState<T>,
        }

        #[derive(Clone)]
        pub(crate) enum OnDiskFileState<T: Clone> {
            /// The file is on disk and can be loaded from there.
            Unloaded,
            Loaded(T),
            /// The file was loaded, but appeared to be missing on disk after reconciling our state with what's on disk.
            /// As there were handles that required pack-id stability we had to keep the item to allow finding it on later
            /// lookups.
            Garbage(T),
            /// File is missing on disk and could not be loaded when we tried or turned missing after reconciling our state.
            Missing,
        }

        impl<T: Clone> OnDiskFile<T> {
            /// Return true if we hold a memory map of the file already.
            pub fn is_loaded(&self) -> bool {
                matches!(self.state, OnDiskFileState::Loaded(_) | OnDiskFileState::Garbage(_))
            }

            pub fn loaded(&self) -> Option<&T> {
                use OnDiskFileState::*;
                match &self.state {
                    Loaded(v) | Garbage(v) => Some(v),
                    Unloaded | Missing => None,
                }
            }

            /// We do it like this as we first have to check for a loaded interior in read-only mode, and then upgrade
            /// when we know that loading is necessary. This also works around borrow check, which is a nice coincidence.
            pub fn do_load(&mut self, load: impl FnOnce(&Path) -> io::Result<T>) -> io::Result<Option<&T>> {
                use OnDiskFileState::*;
                match &mut self.state {
                    Loaded(_) | Garbage(_) => unreachable!("BUG: check before calling this"),
                    Missing => Ok(None),
                    Unloaded => match load(&self.path) {
                        Ok(v) => {
                            self.state = OnDiskFileState::Loaded(v);
                            match &self.state {
                                Loaded(v) => Ok(Some(v)),
                                _ => unreachable!(),
                            }
                        }
                        Err(err) if err.kind() == io::ErrorKind::NotFound => {
                            self.state = OnDiskFileState::Missing;
                            Ok(None)
                        }
                        Err(err) => Err(err),
                    },
                }
            }
        }

        pub(crate) type MultiIndex = ();
        pub(crate) type HandleId = u32;
        /// These are not to be created by anyone except for the State
        pub(crate) enum HandleModeToken {
            DeletedPacksAreInaccessible,
            /// This mode signals that we should not unload packs even after they went missing.
            KeepDeletedPacksAvailable,
        }

        #[derive(Clone)]
        pub(crate) struct IndexFileBundle {
            pub index: OnDiskFile<Arc<git_pack::index::File>>,
            pub data: OnDiskFile<Arc<git_pack::data::File>>,
        }

        #[derive(Clone)]
        pub(crate) struct MultiIndexFileBundle {
            pub multi_index: OnDiskFile<Arc<MultiIndex>>,
            pub data: Vec<OnDiskFile<Arc<git_pack::data::File>>>,
        }

        #[derive(Clone)]
        pub(crate) enum IndexAndPacks {
            Index(IndexFileBundle),
            /// Note that there can only be one multi-pack file per repository, but thanks to git alternates, there can be multiple overall.
            MultiIndex(MultiIndexFileBundle),
        }

        #[derive(Default)]
        pub(crate) struct State {
            /// The root level directory from which we resolve alternates files.
            pub(crate) objects_directory: PathBuf,
            /// All of our database paths, including `objects_directory` as entrypoint
            pub(crate) db_paths: Vec<PathBuf>,
            pub(crate) files: Vec<IndexAndPacks>,
            /// The next index to load if there is the need, also useful as marker value.
            pub(crate) next_to_load: usize,

            /// Generations are incremented whenever we decide to clear out our vectors if they are too big and contains too many empty slots.
            /// If we are not allowed to unload files, the generation will never be changed.
            pub(crate) generation: u8,

            pub(crate) num_handles_stable: usize,
            pub(crate) num_handles_unstable: usize,
        }

        impl State {
            pub(crate) fn marker(&self) -> PackIndexMarker {
                PackIndexMarker {
                    generation: self.generation,
                    pack_index_sequence: self.files.len(),
                }
            }
            pub(crate) fn metrics(&self) -> Metrics {
                let mut open_packs = 0;
                let mut open_indices = 0;

                for f in &self.files {
                    match f {
                        IndexAndPacks::Index(bundle) => {
                            if bundle.index.is_loaded() {
                                open_indices += 1;
                            }
                            if bundle.index.is_loaded() {
                                open_packs += 1;
                            }
                        }
                        IndexAndPacks::MultiIndex(multi) => {
                            if multi.multi_index.is_loaded() {
                                open_indices += 1;
                            }
                            open_packs += multi.data.iter().filter(|f| f.is_loaded()).count();
                        }
                    }
                }

                Metrics {
                    num_handles: self.num_handles_unstable + self.num_handles_stable,
                    open_packs,
                    open_indices,
                }
            }

            /// refresh and possibly clear out our existing data structures, causing all pack ids to be invalidated.
            ///
            /// Note that updates to multi-pack indices always cause the old index to be invalidated (Missing) and transferred
            /// into the new MultiPack index (reusing previous maps as good as possible), effectively treating them as new index entirely.
            /// That way, extension still work as before such that old indices may be pruned, and changes/new ones are always appended.
            pub(crate) fn refresh(&mut self) -> io::Result<load_indices::Outcome> {
                let mut db_paths = git_odb::alternate::resolve(&self.objects_directory)
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
                // These are in addition to our objects directory
                db_paths.insert(0, self.objects_directory.clone());
                todo!()
            }

            /// If there is no handle with stable pack ids requirements, unload them.
            /// This property also relates to us pruning our internal state/doing internal maintenance which affects ids, too.
            pub(crate) fn may_unload_packs(&mut self) -> bool {
                self.num_handles_stable == 0
            }

            pub(crate) fn collect_replace_outcome(&self) -> load_indices::Outcome {
                let indices = self.files.iter().enumerate().filter_map(|(id, file)| {
                    let lookup = match file {
                        IndexAndPacks::Index(bundle) => index_file::SingleOrMulti::Single {
                            index: bundle.index.loaded()?.clone(),
                            data: bundle.data.loaded().cloned(),
                        },
                        IndexAndPacks::MultiIndex(multi) => index_file::SingleOrMulti::Multi {
                            index: multi.multi_index.loaded()?.clone(),
                            data: multi.data.iter().map(|f| f.loaded().cloned()).collect(),
                        },
                    };
                    IndexLookup { file: lookup, id }.into()
                });
                load_indices::Outcome::Replace {
                    indices: todo!(),
                    mark: self.marker(),
                }
            }
        }

        /// A way to indicate which pack indices we have seen already
        pub struct PackIndexMarker {
            /// The generation the `pack_index_sequence` belongs to. Indices of different generations are completely incompatible.
            pub(crate) generation: u8,
            /// An ever increasing number within a generation indicating the maximum number of loaded pack indices and
            /// the amount of slots we have for indices and packs.
            pub(crate) pack_index_sequence: usize,
        }

        /// Define how packs will be refreshed when all indices are loaded, which is useful if a lot of objects are missing.
        #[derive(Clone, Copy)]
        pub enum RefreshMode {
            /// Check for new or changed pack indices (and pack data files) when the last known index is loaded.
            /// During runtime we will keep pack indices stable by never reusing them, however, there is the option for
            /// clearing internal cashes which is likely to change pack ids and it will trigger unloading of packs as they are missing on disk.
            AfterAllIndicesLoaded,
            /// Use this if you expect a lot of missing objects that shouldn't trigger refreshes even after all packs are loaded.
            /// This comes at the risk of not learning that the packs have changed in the mean time.
            Never,
        }

        pub mod load_indices {
            use crate::odb::{policy, policy::PackIndexMarker};

            /// TODO: turn this into a copy-on-write data structure that we just return as whole, with enough information to know
            /// which indices are new so the caller doesn't have to check all new indices.
            pub(crate) enum Outcome {
                /// Drop all data and fully replace it with `indices`.
                /// This happens if we have witnessed a generational change invalidating all of our ids and causing currently loaded
                /// indices and maps to be dropped.
                Replace {
                    indices: Vec<policy::IndexLookup>, // should probably be small vec to get around most allocations
                    mark: PackIndexMarker,             // use to show where the caller left off last time
                },
                /// Extend with the given indices and keep searching, while dropping invalidated/unloaded ids.
                Extend {
                    indices: Vec<policy::IndexLookup>, // should probably be small vec to get around most allocations
                    mark: PackIndexMarker,             // use to show where the caller left off last time
                },
                /// No new indices to look at, caller should give up
                NoMoreIndices,
            }
        }

        /// A snapshot about resource usage.
        pub struct Metrics {
            pub num_handles: usize,
            pub open_indices: usize,
            pub open_packs: usize,
        }
    }

    /// Note that each store is strictly per repository, and that we don't implement any kind of limit of file handles.
    /// The reason for the latter is that it's close to impossible to enforce correctly without heuristics that are probably
    /// better done on the server if there is an indication.
    ///
    /// There is, however, a way to obtain the current amount of open files held by this instance and it's possible to trigger them
    /// to be closed. Alternatively, it's safe to replace this instance with a new one which starts fresh.
    ///
    /// Note that it's possible to let go of loaded files here as well, even though that would be based on a setting allowed file handles
    /// which would be managed elsewhere.
    ///
    /// For maintenance, I think it would be best create an instance when a connection comes in, share it across connections to the same
    /// repository, and remove it as the last connection to it is dropped.
    pub struct Store {
        state: parking_lot::RwLock<policy::State>,

        /// The below state is read-only unless the `state` lock is held.
        pub(crate) files: ArcSwap<Vec<policy::IndexAndPacks>>,
        /// The next index we should load, also useful as marker value.
        pub(crate) next_index_to_load: AtomicUsize,

        /// Generations are incremented whenever we decide to clear out our vectors if they are too big and contains too many empty slots.
        /// If we are not allowed to unload files, the generation will never be changed.
        pub(crate) generation: AtomicU8,
        /// The amount of handles that would prevent us from unloading packs or indices
        pub(crate) num_handles_stable: AtomicUsize,
        /// The amount of handles that don't affect our ability to compact our internal data structures.
        pub(crate) num_handles_unstable: AtomicUsize,
    }

    impl Store {
        pub fn at(objects_directory: impl Into<PathBuf>) -> Arc<Self> {
            Arc::new(Store {
                files: ArcSwap::new(Arc::new(Vec::new())),
                next_index_to_load: Default::default(),
                generation: Default::default(),
                num_handles_stable: Default::default(),
                num_handles_unstable: Default::default(),
                state: parking_lot::RwLock::new(policy::State {
                    objects_directory: objects_directory.into(),
                    ..policy::State::default()
                }),
            })
        }

        /// Allow actually finding things
        pub fn to_handle(self: &Arc<Self>) -> Handle {
            let mode = self.register_handle();
            Handle {
                store: self.clone(),
                refresh_mode: policy::RefreshMode::AfterAllIndicesLoaded,
                mode: mode.into(),
            }
        }

        /// Get a snapshot of the current amount of handles and open packs and indices.
        /// If there are no handles, we are only consuming resources, which might indicate that this instance should be
        /// discarded.
        pub fn state_metrics(&self) -> policy::Metrics {
            self.state.read().metrics()
        }
    }

    /// Handle interaction
    impl Store {
        pub(crate) fn register_handle(&self) -> policy::HandleModeToken {
            let mut state = self.state.write();
            state.num_handles_unstable += 1;
            policy::HandleModeToken::DeletedPacksAreInaccessible
        }
        pub(crate) fn remove_handle(&self, mode: policy::HandleModeToken) {
            let mut state = self.state.write();
            match mode {
                policy::HandleModeToken::KeepDeletedPacksAvailable => state.num_handles_stable -= 1,
                policy::HandleModeToken::DeletedPacksAreInaccessible => state.num_handles_unstable -= 1,
            }
        }
        pub(crate) fn upgrade_handle(&self, mode: policy::HandleModeToken) -> policy::HandleModeToken {
            if let policy::HandleModeToken::DeletedPacksAreInaccessible = mode {
                let mut state = self.state.write();
                state.num_handles_unstable -= 1;
                state.num_handles_stable += 1;
            }
            policy::HandleModeToken::KeepDeletedPacksAvailable
        }
    }

    impl Store {
        /// If Ok(None) is returned, the pack-id was stale and referred to an unloaded pack or a pack which couldn't be
        /// loaded as its file didn't exist on disk anymore.
        /// If the oid is known, just load indices again to continue
        /// (objects rarely ever removed so should be present, maybe in another pack though),
        /// and redo the entire lookup for a valid pack id whose pack can probably be loaded next time.
        pub(crate) fn load_pack(
            &self,
            id: policy::PackId,
            marker: PackIndexMarker,
        ) -> std::io::Result<Option<Arc<git_pack::data::File>>> {
            let state = self.state.read();
            if state.generation != marker.generation {
                return Ok(None);
            }
            match id.multipack_index {
                None => {
                    match state.files.get(id.index) {
                        Some(f) => match f {
                            policy::IndexAndPacks::Index(bundle) => match bundle.data.loaded() {
                                Some(pack) => Ok(Some(pack.clone())),
                                None => {
                                    drop(state);
                                    let mut state = self.state.write();
                                    let f = &mut state.files[id.index];
                                    match f {
                                        policy::IndexAndPacks::Index(bundle) => Ok(bundle
                                            .data
                                            .do_load(|path| {
                                                git_pack::data::File::at(path).map(Arc::new).map_err(|err| match err {
                                                    git_odb::pack::data::header::decode::Error::Io {
                                                        source, ..
                                                    } => source,
                                                    other => std::io::Error::new(std::io::ErrorKind::Other, other),
                                                })
                                            })?
                                            .cloned()),
                                        _ => unreachable!(),
                                    }
                                }
                            },
                            // This can also happen if they use an old index into our new and refreshed data which might have a multi-index
                            // here.
                            policy::IndexAndPacks::MultiIndex(_) => Ok(None),
                        },
                        // This can happen if we condense our data, returning None tells the caller to refresh their indices
                        None => Ok(None),
                    }
                }
                Some(multipack_id) => todo!("load from given multipack which needs additional lookup"),
            }
        }
        pub(crate) fn load_next_indices(
            &self,
            refresh_mode: policy::RefreshMode,
            marker: Option<policy::PackIndexMarker>,
        ) -> std::io::Result<load_indices::Outcome> {
            let state = self.state.read();
            if state.db_paths.is_empty() {
                drop(state);
                return self.state.write().refresh();
            }

            Ok(match marker {
                Some(marker) => {
                    if marker.generation != state.generation {
                        state.collect_replace_outcome()
                    } else if marker.pack_index_sequence == state.files.len() {
                        match refresh_mode {
                            policy::RefreshMode::Never => load_indices::Outcome::NoMoreIndices,
                            policy::RefreshMode::AfterAllIndicesLoaded => return self.state.write().refresh(),
                        }
                    } else {
                        load_indices::Outcome::Extend {
                            indices: todo!("state.files[marker.pack_index_sequence..]"),
                            mark: state.marker(),
                        }
                    }
                }
                None => state.collect_replace_outcome(),
            })
        }
    }

    /// The store shares a policy and keeps a couple of thread-local configuration
    pub struct Handle {
        store: Arc<Store>,
        refresh_mode: policy::RefreshMode,
        mode: Option<policy::HandleModeToken>,
    }

    impl Handle {
        /// Call once if pack ids are stored and later used for lookup, meaning they should always remain mapped and not be unloaded
        /// even if they disappear from disk.
        /// This must be called if there is a chance that git maintenance is happening while a pack is created.
        pub fn prevent_pack_unload(&mut self) {
            self.mode = self.mode.take().map(|mode| self.store.upgrade_handle(mode));
        }
    }

    impl Clone for Handle {
        fn clone(&self) -> Self {
            Handle {
                store: self.store.clone(),
                refresh_mode: self.refresh_mode,
                mode: self.store.register_handle().into(),
            }
        }
    }

    impl Drop for Handle {
        fn drop(&mut self) {
            if let Some(mode) = self.mode.take() {
                self.store.remove_handle(mode)
            }
        }
    }

    impl git_odb::pack::Find for Handle {
        type Error = git_odb::compound::find::Error;

        fn contains(&self, id: impl AsRef<oid>) -> bool {
            todo!()
        }

        fn try_find_cached<'a>(
            &self,
            id: impl AsRef<git_hash::oid>,
            buffer: &'a mut Vec<u8>,
            pack_cache: &mut impl git_pack::cache::DecodeEntry,
        ) -> Result<Option<(git_object::Data<'a>, Option<git_pack::bundle::Location>)>, Self::Error> {
            // TODO: if the generation changes, we need to clear the pack-cache as it depends on pack-ids.
            //       Can we simplify this so it's more obvious what generation does? They must remain stable no matter what
            //       as pack-caches also depend on them and we don't know about these.
            todo!()
        }

        fn location_by_oid(&self, id: impl AsRef<git_hash::oid>, buf: &mut Vec<u8>) -> Option<Location> {
            todo!()
        }

        // TODO: turn this into a pack-id
        fn index_iter_by_pack_id(&self, pack_id: u32) -> Option<Box<dyn Iterator<Item = git_pack::index::Entry> + '_>> {
            todo!()
        }

        fn entry_by_location(&self, location: &Location) -> Option<Entry<'_>> {
            todo!()
        }
    }

    fn try_setup() -> anyhow::Result<()> {
        let policy = Store::at("foo");
        Ok(())
    }
}

mod refs {
    use std::path::{Path, PathBuf};
    use std::sync::Arc;

    pub struct LooseStore {
        path: PathBuf,
        reflog_mode: u32,
        // namespace absent
    }

    pub struct RefTable {
        path: PathBuf,
        reflog_mode: u32,
        // lot's of caching things, no namespace, needs mutability when doing anything (let's not hide it at all)
    }

    mod inner {
        use crate::refs::{LooseStore, RefTable};

        pub(crate) enum StoreSelection {
            Loose(LooseStore),
            RefTable(parking_lot::RwLock<RefTable>),
        }
    }

    pub struct Store {
        inner: inner::StoreSelection,
        packed_refs: parking_lot::RwLock<Option<git_ref::packed::Buffer>>,
        // And of course some more information to track if packed buffer is fresh
    }

    impl Store {
        pub fn new(path: impl AsRef<Path>) -> Arc<Self> {
            Arc::new(Store {
                inner: inner::StoreSelection::Loose(LooseStore {
                    path: path.as_ref().to_owned(),
                    reflog_mode: 0,
                }),
                packed_refs: parking_lot::RwLock::new(None),
            })
        }
        pub fn to_handle(self: &Arc<Self>) -> Handle {
            Handle {
                store: self.clone(),
                namespace: None,
            }
        }

        pub fn to_namespaced_handle(self: &Arc<Self>, namespace: git_ref::Namespace) -> Handle {
            Handle {
                store: self.clone(),
                namespace: namespace.into(),
            }
        }
    }

    #[derive(Clone)]
    pub struct Handle {
        store: Arc<Store>,
        namespace: Option<git_ref::Namespace>,
    }

    // impl common interface but check how all this works with iterators, there is some implementation around that already
    // and maybe this should just be its own State like thing… bet its own Easy so to say.
}

mod repository {
    use crate::{odb, refs};
    use std::sync::Arc;

    mod raw {
        use git_pack::Find;

        /// Let's avoid generics and rather switch the actual implementation with a feature toggle or just for testing.
        /// After all, there is no use for keeping multiple implementations around just for a minor gain and a lot of added complexity.
        /// Definitely run the existing experiments which are exercising the parallel code-paths perfectly and could be adjusted
        /// to also try the access through easy.
        pub struct Repository<Odb>
        where
            Odb: Find, // + Contains + Refresh/Reset maybe?
        {
            odb: Odb,
        }
    }

    /// Maybe we will end up providing a generic version as there still seems to be benefits in having a read-only Store implementation.
    /// MUST BE `Sync`
    #[derive(Clone)]
    pub struct Repository {
        odb: Arc<odb::Store>,
        refs: Arc<refs::Store>,
    }

    #[cfg(test)]
    #[cfg(feature = "thread-safe")]
    mod tests {
        use super::*;

        #[test]
        fn is_sync_and_send() {
            fn spawn<T: Send + Sync>(_v: T) {}
            let repository = Repository {
                odb: odb::Store::at("./foo/objects"),
                refs: refs::Store::new("./foo"),
            };
            spawn(&repository);
            spawn(repository);
        }
    }
}
