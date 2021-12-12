#![allow(missing_docs, unused, dead_code)]

use arc_swap::ArcSwap;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;

/// This effectively acts like a handle but exists to be usable from the actual `crate::Handle` implementation which adds caches on top.
/// Each store is quickly cloned and contains thread-local state for shared packs.
pub struct Handle<S>
where
    S: Deref<Target = Store> + Clone,
{
    pub(crate) store: S,
    pub(crate) token: Option<handle::Mode>,
}

pub struct Store {
    /// The source directory from which all content is loaded, and the central write lock for use when a directory refresh is needed.
    path: parking_lot::Mutex<PathBuf>,

    /// A list of indices keeping track of which slots are filled with data. These are usually, but not always, consecutive.
    pub(crate) index: ArcSwap<store::SlotMapIndex>,

    /// The below state acts like a slot-map with each slot is mutable when the write lock is held, but readable independently of it.
    /// This allows multiple file to be loaded concurrently if there is multiple handles requesting to load packs or additional indices.
    /// The map is static and cannot typically change.
    /// It's read often and changed rarely.
    pub(crate) files: Vec<store::MutableIndexAndPack>,

    /// The amount of handles that would prevent us from unloading packs or indices
    pub(crate) num_handles_stable: AtomicUsize,
    /// The amount of handles that don't affect our ability to compact our internal data structures or unload packs or indices.
    pub(crate) num_handles_unstable: AtomicUsize,
}

mod find {
    use git_hash::oid;
    use git_object::Data;
    use git_pack::cache::DecodeEntry;
    use git_pack::data::entry::Location;
    use git_pack::index::Entry;
    use std::ops::Deref;

    impl<S> crate::pack::Find for super::Handle<S>
    where
        S: Deref<Target = super::Store> + Clone,
    {
        type Error = crate::compound::find::Error;

        fn contains(&self, id: impl AsRef<oid>) -> bool {
            todo!()
        }

        fn try_find_cached<'a>(
            &self,
            id: impl AsRef<oid>,
            buffer: &'a mut Vec<u8>,
            pack_cache: &mut impl DecodeEntry,
        ) -> Result<Option<(Data<'a>, Option<Location>)>, Self::Error> {
            todo!()
        }

        fn location_by_oid(&self, id: impl AsRef<oid>, buf: &mut Vec<u8>) -> Option<Location> {
            todo!()
        }

        fn index_iter_by_pack_id(&self, pack_id: u32) -> Option<Box<dyn Iterator<Item = Entry> + '_>> {
            todo!()
        }

        fn entry_by_location(&self, location: &Location) -> Option<git_pack::find::Entry<'_>> {
            todo!()
        }
    }
}

mod init {
    use crate::general::store;
    use crate::general::store::{MutableIndexAndPack, SlotMapIndex};
    use arc_swap::ArcSwap;
    use git_features::threading::OwnShared;
    use std::iter::FromIterator;
    use std::ops::Deref;
    use std::path::PathBuf;
    use std::sync::atomic::AtomicUsize;
    use std::sync::Arc;

    impl super::Store {
        pub fn at(objects_dir: impl Into<PathBuf>) -> std::io::Result<Self> {
            let objects_dir = objects_dir.into();
            if !objects_dir.is_dir() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other, // TODO: use NotADirectory when stabilized
                    format!("'{}' wasn't a directory", objects_dir.display()),
                ));
            }
            Ok(super::Store {
                path: parking_lot::Mutex::new(objects_dir),
                files: Vec::from_iter(std::iter::repeat_with(MutableIndexAndPack::default).take(256)), // TODO: figure this out from the amount of files currently present
                index: ArcSwap::new(Arc::new(SlotMapIndex::default())),
                num_handles_stable: Default::default(),
                num_handles_unstable: Default::default(),
            })
        }
    }
}

mod store {
    use arc_swap::ArcSwap;
    use std::ops::BitXor;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    /// An id to refer to an index file or a multipack index file
    pub type IndexId = usize;
    pub(crate) type StateId = usize;

    /// A way to indicate which pack indices we have seen already and which of them are loaded, along with an idea
    /// of whether stored `PackId`s are still usable.
    #[derive(Default)]
    pub struct SlotIndexMarker {
        /// The generation the `loaded_until_index` belongs to. Indices of different generations are completely incompatible.
        /// This value changes once the internal representation is compacted, something that may happen only if there is no handle
        /// requiring stable pack indices.
        pub(crate) generation: u8,
        /// A unique id identifying the index state as well as all loose databases we have last observed.
        /// If it changes in any way, the value is different.
        pub(crate) state_id: StateId,
    }

    /// A way to load and refer to a pack uniquely, namespaced by their indexing mechanism, aka multi-pack or not.
    pub struct PackId {
        /// Note that if `multipack_index = None`, this index is corresponding to the index id.
        /// So a pack is always identified by its corresponding index.
        /// If it is a multipack index, this is the id / offset of the pack in the `multipack_index`.
        pub(crate) index: IndexId,
        pub(crate) multipack_index: Option<IndexId>,
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
        pub(crate) loose_dbs: Arc<Vec<crate::loose::Store>>,
    }

    impl SlotMapIndex {
        pub(crate) fn state_id(self: &Arc<SlotMapIndex>) -> StateId {
            // We let the loaded indices take part despite not being part of our own snapshot.
            // This is to account for indices being loaded in parallel without actually changing the snapshot itself.
            (Arc::as_ptr(&self.loose_dbs) as usize)
                .bitxor(Arc::as_ptr(self) as usize)
                .wrapping_mul(self.loaded_indices.load(Ordering::SeqCst) + 1)
        }

        pub(crate) fn marker(self: &Arc<SlotMapIndex>) -> SlotIndexMarker {
            self.into()
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
        pub fn do_load(&mut self, load: impl FnOnce(&Path) -> std::io::Result<T>) -> std::io::Result<Option<&T>> {
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
                    Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                        self.state = OnDiskFileState::Missing;
                        Ok(None)
                    }
                    Err(err) => Err(err),
                },
            }
        }
    }

    #[derive(Clone)]
    pub(crate) struct IndexFileBundle {
        pub index: OnDiskFile<Arc<git_pack::index::File>>,
        pub data: OnDiskFile<Arc<git_pack::data::File>>,
    }

    #[derive(Clone)]
    pub(crate) struct MultiIndexFileBundle {
        pub multi_index: OnDiskFile<Arc<super::handle::multi_index::File>>,
        pub data: Vec<OnDiskFile<Arc<git_pack::data::File>>>,
    }

    #[derive(Clone)]
    pub(crate) enum IndexAndPacks {
        Index(IndexFileBundle),
        /// Note that there can only be one multi-pack file per repository, but thanks to git alternates, there can be multiple overall.
        MultiIndex(MultiIndexFileBundle),
    }

    #[derive(Default)]
    pub(crate) struct MutableIndexAndPack {
        pub(crate) files: ArcSwap<Option<IndexAndPacks>>,
        pub(crate) write: parking_lot::Mutex<()>,
    }
}

pub mod handle {
    use crate::general::store;
    use git_features::threading::OwnShared;
    use std::ops::Deref;
    use std::sync::atomic::Ordering;
    use std::sync::Arc;

    pub(crate) mod multi_index {
        // TODO: replace this one with an actual implementation of a multi-pack index.
        pub type File = ();
    }

    pub enum SingleOrMultiIndex {
        Single {
            index: Arc<git_pack::index::File>,
            data: Option<Arc<git_pack::data::File>>,
        },
        Multi {
            index: Arc<multi_index::File>,
            data: Vec<Option<Arc<git_pack::data::File>>>,
        },
    }

    pub struct IndexLookup {
        pub(crate) file: SingleOrMultiIndex,
        pub(crate) id: store::IndexId,
    }

    pub struct IndexForObjectInPack {
        /// The internal identifier of the pack itself, which either is referred to by an index or a multi-pack index.
        pack_id: store::PackId,
        /// The index of the object within the pack
        object_index_in_pack: u32,
    }

    pub(crate) mod index_lookup {
        use crate::general::{handle, store};
        use git_hash::oid;
        use std::sync::Arc;

        impl handle::IndexLookup {
            /// See if the oid is contained in this index, and return its full id for lookup possibly alongside its data file if already
            /// loaded.
            /// If it is not loaded, ask it to be loaded and put it into the returned mutable option for safe-keeping.
            fn lookup(
                &mut self,
                object_id: &oid,
            ) -> Option<(handle::IndexForObjectInPack, &mut Option<Arc<git_pack::data::File>>)> {
                let id = self.id;
                match &mut self.file {
                    handle::SingleOrMultiIndex::Single { index, data } => {
                        index.lookup(object_id).map(|object_index_in_pack| {
                            (
                                handle::IndexForObjectInPack {
                                    pack_id: store::PackId {
                                        index: id,
                                        multipack_index: None,
                                    },
                                    object_index_in_pack,
                                },
                                data,
                            )
                        })
                    }
                    handle::SingleOrMultiIndex::Multi { index, data } => {
                        todo!("find respective pack and return it as &mut Option<>")
                    }
                }
            }
        }
    }

    pub(crate) enum Mode {
        DeletedPacksAreInaccessible,
        /// This mode signals that we should not unload packs even after they went missing.
        KeepDeletedPacksAvailable,
    }

    /// Handle registration
    impl super::Store {
        pub(crate) fn register_handle(&self) -> Mode {
            self.num_handles_unstable.fetch_add(1, Ordering::Relaxed);
            Mode::DeletedPacksAreInaccessible
        }
        pub(crate) fn remove_handle(&self, mode: Mode) {
            match mode {
                Mode::KeepDeletedPacksAvailable => {
                    let _ = self.path.lock();
                    self.num_handles_stable.fetch_sub(1, Ordering::SeqCst)
                }
                Mode::DeletedPacksAreInaccessible => self.num_handles_unstable.fetch_sub(1, Ordering::Relaxed),
            };
        }
        pub(crate) fn upgrade_handle(&self, mode: Mode) -> Mode {
            if let Mode::DeletedPacksAreInaccessible = mode {
                let _ = self.path.lock();
                self.num_handles_stable.fetch_add(1, Ordering::SeqCst);
                self.num_handles_unstable.fetch_sub(1, Ordering::SeqCst);
            }
            Mode::KeepDeletedPacksAvailable
        }
    }

    /// Handle creation
    impl super::Store {
        pub fn to_handle(self: &OwnShared<Self>) -> super::Handle<OwnShared<super::Store>> {
            let token = self.register_handle();
            super::Handle {
                store: self.clone(),
                token: Some(token),
            }
        }
    }

    impl<S> super::Handle<S>
    where
        S: Deref<Target = super::Store> + Clone,
    {
        /// Call once if pack ids are stored and later used for lookup, meaning they should always remain mapped and not be unloaded
        /// even if they disappear from disk.
        /// This must be called if there is a chance that git maintenance is happening while a pack is created.
        pub fn prevent_pack_unload(&mut self) {
            self.token = self.token.take().map(|token| self.store.upgrade_handle(token));
        }
    }

    impl<S> Drop for super::Handle<S>
    where
        S: Deref<Target = super::Store> + Clone,
    {
        fn drop(&mut self) {
            if let Some(token) = self.token.take() {
                self.store.remove_handle(token)
            }
        }
    }

    impl<S> Clone for super::Handle<S>
    where
        S: Deref<Target = super::Store> + Clone,
    {
        fn clone(&self) -> Self {
            super::Handle {
                store: self.store.clone(),
                token: self.store.register_handle().into(),
            }
        }
    }
}

pub mod load_indices {
    use crate::general::{handle, store};
    use std::path::PathBuf;

    /// Define how packs will be refreshed when all indices are loaded, which is useful if a lot of objects are missing.
    #[derive(Clone, Copy)]
    pub enum RefreshMode {
        /// Check for new or changed pack indices (and pack data files) when the last known index is loaded.
        /// During runtime we will keep pack indices stable by never reusing them, however, there is the option for
        /// clearing internal caches which is likely to change pack ids and it will trigger unloading of packs as they are missing on disk.
        AfterAllIndicesLoaded,
        /// Use this if you expect a lot of missing objects that shouldn't trigger refreshes even after all packs are loaded.
        /// This comes at the risk of not learning that the packs have changed in the mean time.
        Never,
    }

    use crate::general::store::StateId;
    use std::sync::atomic::Ordering;
    use std::sync::Arc;

    pub(crate) enum Outcome {
        /// Drop all data and fully replace it with `indices`.
        /// This happens if we have witnessed a generational change invalidating all of our ids and causing currently loaded
        /// indices and maps to be dropped.
        Replace {
            indices: Vec<handle::IndexLookup>, // should probably be SmallVec to get around most allocations
            loose_dbs: Arc<Vec<crate::loose::Store>>,
            marker: store::SlotIndexMarker, // use to show where the caller left off last time
        },
        /// No new indices to look at, caller should give up
        NoMoreIndices,
    }

    impl super::Store {
        pub(crate) fn load_next_indices(
            &self,
            refresh_mode: RefreshMode,
            marker: Option<store::SlotIndexMarker>,
        ) -> std::io::Result<Outcome> {
            let index = self.index.load();
            let state_id = index.state_id();
            if index.loose_dbs.is_empty() {
                // TODO: figure out what kind of refreshes we need. This one loads in the initial slot map, but I think this cost is paid
                //       in full during instantiation.
                return self.consolidate_with_disk_state(state_id);
            }

            Ok(match marker {
                Some(marker) => {
                    if marker.generation != index.generation {
                        self.collect_replace_outcome()
                    } else if marker.state_id == state_id {
                        match refresh_mode {
                            RefreshMode::Never => Outcome::NoMoreIndices,
                            RefreshMode::AfterAllIndicesLoaded => return self.consolidate_with_disk_state(state_id),
                        }
                    } else {
                        self.collect_replace_outcome()
                    }
                }
                None => self.collect_replace_outcome(),
            })
        }

        /// refresh and possibly clear out our existing data structures, causing all pack ids to be invalidated.
        fn consolidate_with_disk_state(&self, seen: StateId) -> std::io::Result<Outcome> {
            let objects_directory = self.path.lock();
            if seen != self.index.load().state_id() {
                todo!("return …")
            }
            let mut db_paths = crate::alternate::resolve(&*objects_directory)
                .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
            // These are in addition to our objects directory
            db_paths.insert(0, objects_directory.clone());
            todo!()
        }

        /// If there is no handle with stable pack ids requirements, unload them.
        /// This property also relates to us pruning our internal state/doing internal maintenance which affects ids, too.
        ///
        /// Note that this must be called with a lock to the relevant state held to assure these values don't change while
        /// we are working.
        fn may_unload_packs(&mut self, guard: &parking_lot::MutexGuard<'_, PathBuf>) -> bool {
            self.num_handles_stable.load(Ordering::SeqCst) == 0
        }

        fn collect_replace_outcome(&self) -> Outcome {
            let index = self.index.load();
            let indices = index
                .slot_indices
                .iter()
                .map(|idx| (*idx, &self.files[*idx]))
                .filter_map(|(id, file)| {
                    let lookup = match (&**file.files.load()).as_ref()? {
                        store::IndexAndPacks::Index(bundle) => handle::SingleOrMultiIndex::Single {
                            index: bundle.index.loaded()?.clone(),
                            data: bundle.data.loaded().cloned(),
                        },
                        store::IndexAndPacks::MultiIndex(multi) => handle::SingleOrMultiIndex::Multi {
                            index: multi.multi_index.loaded()?.clone(),
                            data: multi.data.iter().map(|f| f.loaded().cloned()).collect(),
                        },
                    };
                    handle::IndexLookup { file: lookup, id }.into()
                })
                .collect();
            Outcome::Replace {
                indices,
                loose_dbs: Arc::clone(&index.loose_dbs),
                marker: index.marker(),
            }
        }
    }
}
