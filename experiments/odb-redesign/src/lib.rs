#![allow(dead_code, unused_variables, unreachable_code)]

mod odb {
    use arc_swap::ArcSwap;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    use git_hash::oid;
    use git_odb::pack::{data::entry::Location, find::Entry};

    use crate::odb::store::{load_indices, SlotIndexMarker, SlotMapIndex};

    pub mod store {
        use arc_swap::ArcSwap;
        use std::ops::Deref;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;
        use std::{
            io,
            path::{Path, PathBuf},
        };

        use crate::odb::Store;
        use git_hash::oid;

        mod index_file {
            use crate::odb::store;
            use std::sync::Arc;

            pub enum SingleOrMulti {
                Single {
                    index: Arc<git_pack::index::File>,
                    data: Option<Arc<git_pack::data::File>>,
                },
                Multi {
                    index: Arc<store::MultiIndex>,
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

        pub(crate) struct MutableIndexAndPack {
            pub(crate) files: ArcSwap<IndexAndPacks>,
            pub(crate) write: parking_lot::Mutex<()>,
        }

        #[derive(Default)]
        pub(crate) struct State {
            /// The root level directory from which we resolve alternates files.
            pub(crate) objects_directory: PathBuf,
        }

        impl Store {
            pub(crate) fn marker(&self) -> SlotIndexMarker {
                (self.index.load().deref()).into()
            }

            /// A best-effort evaluation of metrics, as it counts values that are not synchronized with each other.
            pub(crate) fn metrics(&self) -> Metrics {
                let mut open_packs = 0;
                let mut open_indices = 0;

                for f in self.index.load().slot_indices.iter().map(|idx| &self.files[*idx]) {
                    match &**f.files.load() {
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
                    num_handles: self.num_handles_unstable.load(Ordering::SeqCst)
                        + self.num_handles_stable.load(Ordering::SeqCst),
                    open_packs,
                    open_indices,
                }
            }

            /// refresh and possibly clear out our existing data structures, causing all pack ids to be invalidated.
            ///
            /// Note that updates to multi-pack indices always cause the old index to be invalidated (Missing) and transferred
            /// into the new MultiPack index (reusing previous maps as good as possible), effectively treating them as new index entirely.
            /// That way, extension still work as before such that old indices may be pruned, and changes/new ones are always appended.
            pub(crate) fn refresh(&self) -> io::Result<load_indices::Outcome> {
                let state = self.state.lock();
                let mut db_paths = git_odb::alternate::resolve(&state.objects_directory)
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
                // These are in addition to our objects directory
                db_paths.insert(0, state.objects_directory.clone());
                todo!()
            }

            /// If there is no handle with stable pack ids requirements, unload them.
            /// This property also relates to us pruning our internal state/doing internal maintenance which affects ids, too.
            pub(crate) fn may_unload_packs(&mut self) -> bool {
                // TODO: figure out if this needs to have the write lock to function properly
                self.num_handles_stable.load(Ordering::SeqCst) == 0
            }

            pub(crate) fn collect_replace_outcome(&self) -> load_indices::Outcome {
                let guard = self.index.load();
                let indices = guard
                    .slot_indices
                    .iter()
                    .map(|idx| (*idx, &self.files[*idx]))
                    .filter_map(|(id, file)| {
                        let lookup = match &**file.files.load() {
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
                    loose_dbs: Arc::clone(&guard.loose_dbs),
                    mark: self.marker(),
                }
            }
        }

        /// An index that changes only if the packs directory changes and its contents is re-read.
        #[derive(Default)]
        pub struct SlotMapIndex {
            /// The index into the slot map at which we expect an index or pack file. Neither of these might be loaded yet.
            pub(crate) slot_indices: Vec<usize>,
            /// A static value that doesn't ever change for a particular clone of this index.
            pub(crate) generation: u8,
            /// The number of indices loaded thus far when the index of the slot map was last examined, which can change as new indices are loaded
            /// in parallel.
            /// Shared across SlotMapIndex instances of the same generation.
            pub(crate) next_index_to_load: Arc<AtomicUsize>,
            /// Incremented by one up to `slot_indices.len()` once index was actually loaded. If a load failed, there will be no increment.
            /// Shared across SlotMapIndex instances of the same generation.
            pub(crate) loaded_indices: Arc<AtomicUsize>,
            /// A list of loose object databases as resolved by their alternates file in the `object_directory`. The first entry is this objects
            /// directory loose file database. All other entries are the loose stores of alternates.
            /// It's in an Arc to be shared to Handles, but not to be shared across SlotMapIndices
            pub(crate) loose_dbs: Arc<Vec<git_odb::loose::Store>>,
        }

        impl SlotMapIndex {
            pub(crate) fn state_id(self: &Arc<SlotMapIndex>) -> usize {
                // We let the loaded indices take part despite not being part of our own snapshot.
                // This is to account for indices being loaded in parallel without actually changing the snapshot itself.
                (Arc::as_ptr(&self.loose_dbs) as usize ^ Arc::as_ptr(self) as usize)
                    * (self.loaded_indices.load(Ordering::SeqCst) + 1)
            }
        }

        /// Note that this is a snapshot of SlotMapIndex, even though some internal values are shared, it's for sharing to callers, not among
        /// versions of the SlotMapIndex
        impl From<&Arc<SlotMapIndex>> for SlotIndexMarker {
            fn from(v: &Arc<SlotMapIndex>) -> Self {
                SlotIndexMarker {
                    generation: v.generation,
                    state_id: v.state_id(),
                }
            }
        }

        /// A way to indicate which pack indices we have seen already. This type is meant as a snapshot for the internally used `AtomicIndexMarker`.
        #[derive(Clone, Default)]
        pub struct SlotIndexMarker {
            /// The generation the `loaded_until_index` belongs to. Indices of different generations are completely incompatible.
            /// This value changes once the internal representation is compacted, something that may happen only if there is no handle
            /// requiring stable pack indices.
            pub(crate) generation: u8,
            /// A unique id identifying the index state as well as all loose databases we have last observed.
            /// If it changes in any way, the value is different.
            pub(crate) state_id: usize,
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
            use crate::odb::{store, store::SlotIndexMarker};
            use std::sync::Arc;

            pub(crate) enum Outcome {
                /// Drop all data and fully replace it with `indices`.
                /// This happens if we have witnessed a generational change invalidating all of our ids and causing currently loaded
                /// indices and maps to be dropped.
                Replace {
                    indices: Vec<store::IndexLookup>, // should probably be SmallVec to get around most allocations
                    loose_dbs: Arc<Vec<git_odb::loose::Store>>,
                    mark: SlotIndexMarker, // use to show where the caller left off last time
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
    /// File handles are released only once all handles to the store are gone. It's possible that some handles remain open as 'garbage'
    /// as they were held while there was a handle which prevented collection. It's best to close these by opening a new store occasionally
    /// based on the metrics it provides.
    ///
    /// For maintenance, I think it would be best create an instance when a connection comes in, share it across connections to the same
    /// repository, and remove it as the last connection to it is dropped.
    pub struct Store {
        /// Possibly mutable state which is accessed only rarely when the folder index has to be refreshed.
        state: parking_lot::Mutex<store::State>,

        /// A list of indices keeping track of which slots are filled with data. These are usually, but not always, consecutive.
        /// It also keeps track of which
        pub(crate) index: ArcSwap<SlotMapIndex>,

        /// The below state acts like a slot-map with each slot is mutable when the write lock is held, but readable independently of it.
        /// This allows multiple file to be loaded concurrently if there is multiple handles requesting to load packs or additional indices.
        /// The map is static and cannot typically change.
        /// It's read often and changed rarely.
        pub(crate) files: Vec<store::MutableIndexAndPack>,

        /// The amount of handles that would prevent us from unloading packs or indices
        pub(crate) num_handles_stable: AtomicUsize,
        /// The amount of handles that don't affect our ability to compact our internal data structures.
        pub(crate) num_handles_unstable: AtomicUsize,
    }

    impl Store {
        pub fn at(objects_directory: impl Into<PathBuf>) -> Arc<Self> {
            Arc::new(Store {
                state: parking_lot::Mutex::new(store::State {
                    objects_directory: objects_directory.into(),
                }),

                files: vec![], // TODO: initialize with a sane number of slots, probably based on the contents of the packs directory.
                index: ArcSwap::new(Arc::new(SlotMapIndex::default())),

                num_handles_stable: Default::default(),
                num_handles_unstable: Default::default(),
            })
        }

        /// Allow actually finding things
        pub fn to_handle(self: &Arc<Self>) -> Handle {
            let mode = self.register_handle();
            Handle {
                store: self.clone(),
                refresh_mode: store::RefreshMode::AfterAllIndicesLoaded,
                mode: mode.into(),
            }
        }
    }

    /// Handle interaction
    impl Store {
        pub(crate) fn register_handle(&self) -> store::HandleModeToken {
            self.num_handles_unstable.fetch_add(1, Ordering::Relaxed);
            store::HandleModeToken::DeletedPacksAreInaccessible
        }
        pub(crate) fn remove_handle(&self, mode: store::HandleModeToken) {
            match mode {
                store::HandleModeToken::KeepDeletedPacksAvailable => {
                    let _ = self.state.lock();
                    self.num_handles_stable.fetch_sub(1, Ordering::SeqCst)
                }
                store::HandleModeToken::DeletedPacksAreInaccessible => {
                    self.num_handles_unstable.fetch_sub(1, Ordering::Relaxed)
                }
            };
        }
        pub(crate) fn upgrade_handle(&self, mode: store::HandleModeToken) -> store::HandleModeToken {
            if let store::HandleModeToken::DeletedPacksAreInaccessible = mode {
                let _ = self.state.lock();
                self.num_handles_stable.fetch_add(1, Ordering::SeqCst);
                self.num_handles_unstable.fetch_sub(1, Ordering::SeqCst);
            }
            store::HandleModeToken::KeepDeletedPacksAvailable
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
            id: store::PackId,
            marker: SlotIndexMarker,
        ) -> std::io::Result<Option<Arc<git_pack::data::File>>> {
            let index = self.index.load();
            if index.generation != marker.generation {
                return Ok(None);
            }
            match id.multipack_index {
                None => {
                    let f = &self.files[id.index];
                    match &**f.files.load() {
                        store::IndexAndPacks::Index(bundle) => match bundle.data.loaded() {
                            Some(pack) => Ok(Some(pack.clone())),
                            None => {
                                let _lock = f.write.lock();
                                // let f = &mut files[id.index];
                                // match f {
                                //     policy::IndexAndPacks::Index(bundle) => Ok(bundle
                                //         .data
                                //         .do_load(|path| {
                                //             git_pack::data::File::at(path).map(Arc::new).map_err(|err| match err {
                                //                 git_odb::pack::data::header::decode::Error::Io {
                                //                     source, ..
                                //                 } => source,
                                //                 other => std::io::Error::new(std::io::ErrorKind::Other, other),
                                //             })
                                //         })?
                                //         .cloned()),
                                //     _ => unreachable!(),
                                // }
                                todo!()
                            }
                        },
                        // This can also happen if they use an old index into our new and refreshed data which might have a multi-index
                        // here.
                        store::IndexAndPacks::MultiIndex(_) => Ok(None),
                    }
                }
                Some(multipack_id) => todo!("load from given multipack which needs additional lookup"),
            }
        }
        pub(crate) fn load_next_indices(
            &self,
            refresh_mode: store::RefreshMode,
            marker: Option<store::SlotIndexMarker>,
        ) -> std::io::Result<load_indices::Outcome> {
            let index = self.index.load();
            if index.loose_dbs.is_empty() {
                // TODO: figure out what kind of refreshes we need. This one loads in the initial slot map, but I think this cost is paid
                //       in full during instantiation.
                return self.refresh();
            }

            Ok(match marker {
                Some(marker) => {
                    if marker.generation != index.generation {
                        self.collect_replace_outcome()
                    } else if marker.state_id == index.state_id() {
                        match refresh_mode {
                            store::RefreshMode::Never => load_indices::Outcome::NoMoreIndices,
                            store::RefreshMode::AfterAllIndicesLoaded => return self.refresh(),
                        }
                    } else {
                        self.collect_replace_outcome()
                    }
                }
                None => self.collect_replace_outcome(),
            })
        }
    }

    /// The store shares a policy and keeps a couple of thread-local configuration
    pub struct Handle {
        store: Arc<Store>,
        refresh_mode: store::RefreshMode,
        mode: Option<store::HandleModeToken>,
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
        ) -> Result<Option<(git_object::Data<'a>, Option<Location>)>, Self::Error> {
            todo!()
        }

        fn location_by_oid(&self, id: impl AsRef<git_hash::oid>, buf: &mut Vec<u8>) -> Option<Location> {
            todo!()
        }

        /// This requires a one-time mapping/find operation to find the actual pack. That's OK here.
        fn index_iter_by_pack_id(&self, pack_id: u32) -> Option<Box<dyn Iterator<Item = git_pack::index::Entry> + '_>> {
            todo!()
        }

        /// This operation can be more expensive unless the handle has a local mapping between u32 pack id and the PackId that shows the actual
        /// index. There is no other way to quickly get from actual pack id to where it's stored in our slot-map, and possibly which multi-index
        /// it belongs to.
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
