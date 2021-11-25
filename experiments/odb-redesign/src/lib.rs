#![allow(dead_code, unused_variables)]

mod features {
    mod threaded {
        use std::sync::Arc;

        pub type OwnShared<T> = Arc<T>;
        pub type MutableOnDemand<T> = parking_lot::RwLock<T>;

        pub fn get_ref_upgradeable<T>(v: &MutableOnDemand<T>) -> parking_lot::RwLockUpgradableReadGuard<'_, T> {
            v.upgradable_read()
        }

        pub fn get_ref<T>(v: &MutableOnDemand<T>) -> parking_lot::RwLockReadGuard<'_, T> {
            v.read()
        }

        pub fn get_mut<T>(v: &MutableOnDemand<T>) -> parking_lot::RwLockWriteGuard<'_, T> {
            v.write()
        }

        pub fn upgrade_ref_to_mut<T>(
            v: parking_lot::RwLockUpgradableReadGuard<'_, T>,
        ) -> parking_lot::RwLockWriteGuard<'_, T> {
            parking_lot::RwLockUpgradableReadGuard::upgrade(v)
        }
    }

    mod local {
        use std::cell::{Ref, RefCell, RefMut};
        use std::rc::Rc;

        pub type OwnShared<T> = Rc<T>;
        pub type MutableOnDemand<T> = RefCell<T>;

        pub fn get_ref_upgradeable<T>(v: &RefCell<T>) -> RefMut<'_, T> {
            v.borrow_mut()
        }

        pub fn get_mut<T>(v: &RefCell<T>) -> RefMut<'_, T> {
            v.borrow_mut()
        }

        pub fn get_ref<T>(v: &RefCell<T>) -> Ref<'_, T> {
            v.borrow()
        }

        pub fn upgrade_ref_to_mut<T>(v: RefMut<'_, T>) -> RefMut<'_, T> {
            v
        }
    }

    #[cfg(not(feature = "thread-safe"))]
    pub use local::*;
    #[cfg(feature = "thread-safe")]
    pub use threaded::*;
}

mod odb {
    use crate::features;
    use crate::features::{get_mut, get_ref, get_ref_upgradeable, upgrade_ref_to_mut};
    use crate::odb::policy::{load_indices, PackIndexMarker};
    use git_odb::data::Object;
    use git_odb::pack::bundle::Location;
    use git_odb::pack::cache::DecodeEntry;
    use git_odb::pack::find::Entry;
    use git_odb::pack::Bundle;
    use std::path::PathBuf;

    pub mod policy {
        use crate::features;
        use crate::odb::policy;
        use git_hash::oid;
        use std::collections::BTreeMap;
        use std::io;
        use std::path::PathBuf;

        mod index_file {
            pub enum SingleOrMulti {
                Single(git_pack::index::File),
                Multi(()),
            }
        }

        pub(crate) struct IndexForObjectInPack {
            /// The internal identifier of the pack itself, which either is referred to by an index or a multi-pack index.
            pub(crate) pack_id: PackId,
            /// The index of the object within the pack
            object_index_in_pack: u32,
        }

        /// An id to refer to an index file or a multipack index file
        pub struct IndexId(u32);

        /// A way to load and refer to a pack uniquely, namespaced by their indexing mechanism, aka multi-pack or not.
        pub struct PackId {
            /// Note that if `multipack_index = None`, this id is corresponding to the index id.
            /// If it is a multipack index, `id` is the id / offset of the pack in the `multipack_index`.
            pub(crate) index: u32,
            pub(crate) multipack_index: Option<IndexId>,
        }

        pub(crate) struct IndexLookup {
            file: index_file::SingleOrMulti,
            pub id: IndexId,
        }

        impl IndexLookup {
            fn lookup(&self, object_id: &oid) -> Option<IndexForObjectInPack> {
                match &self.file {
                    index_file::SingleOrMulti::Single(file) => {
                        file.lookup(object_id).map(|object_index_in_pack| IndexForObjectInPack {
                            pack_id: PackId {
                                index: self.id.0,
                                multipack_index: None,
                            },
                            object_index_in_pack,
                        })
                    }
                    index_file::SingleOrMulti::Multi(()) => todo!("required with multi-pack index"),
                }
            }
        }

        pub(crate) struct OnDiskFile<T> {
            /// The last known path of the file
            path: PathBuf,
            state: OnDiskFileState<T>,
        }

        pub(crate) enum OnDiskFileState<T> {
            /// The file is on disk and can be loaded from there.
            Unloaded,
            Loaded(T),
            /// The file was loaded, but appeared to be missing on disk after reconciling our state with what's on disk.
            /// As there were handles that required pack-id stability we had to keep the pack.
            Garbage(T),
            /// File is missing on disk and could not be loaded when we tried or turned missing after reconciling our state.
            Missing,
        }

        impl<T> OnDiskFile<T> {
            /// Return true if we hold a memory map of the file already.
            pub fn is_loaded(&self) -> bool {
                matches!(self.state, OnDiskFileState::Loaded(_) | OnDiskFileState::Garbage(_))
            }
        }

        type PackDataFiles = Vec<Option<features::OwnShared<OnDiskFile<git_pack::data::File>>>>;

        pub(crate) type HandleId = u32;
        /// These are not to be created by anyone except for the State
        pub(crate) enum HandleModeToken {
            /// Pack-ids may change which may cause lookups by pack-id (without an oid available) to fail.
            Unstable,
            Stable,
        }

        pub(crate) struct IndexFileBundle {
            index: OnDiskFile<git_pack::index::File>,
            path: PathBuf,
            data: OnDiskFile<git_pack::data::File>,
        }

        pub(crate) struct MultiIndexFileBundle {
            multi_index: OnDiskFile<git_pack::index::File>, // TODO: turn that into multi-index file when available
            path: PathBuf,
            data: Vec<OnDiskFile<git_pack::data::File>>,
        }

        pub(crate) enum IndexAndPacks {
            Index(IndexFileBundle),
            /// Note that there can only be one multi-pack file per repository, but thanks to git alternates, there can be multiple overall.
            MultiIndex(MultiIndexFileBundle),
        }

        #[derive(Default)]
        pub(crate) struct State {
            /// The root level directory from which we resolve alternates files.
            pub(crate) objects_directory: PathBuf,
            pub(crate) db_paths: Vec<PathBuf>,
            pub(crate) files: Vec<IndexAndPacks>,
            pub(crate) loaded_indices: Vec<Option<features::OwnShared<policy::IndexLookup>>>,
            pub(crate) loaded_packs: PackDataFiles,
            /// Each change in the multi-pack index creates a new index entry here and typically drops all knowledge about its removed packs.
            /// We do this because there can be old pack ids around that refer to the old (now deleted) multi-pack index along with a possibly
            /// now invalid (or ambiguous) local pack id, i.e. pack A was meant, but now that's pack B because the multi-index has changed.
            pub(crate) loaded_packs_by_multi_index: BTreeMap<IndexId, PackDataFiles>,
            /// Generations are incremented whenever we decide to clear out our vectors if they are too big and contains too many empty slots.
            /// If we are not allowed to unload files, the generation will never be changed.
            pub(crate) generation: u8,

            pub(crate) num_handles_stable: usize,
            pub(crate) num_handles_unstable: usize,
        }

        impl State {
            pub(crate) fn snapshot(&self) -> StateInformation {
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

                StateInformation {
                    num_handles: self.num_handles_unstable + self.num_handles_stable,
                    open_packs,
                    open_indices,
                }
            }

            /// refresh and possibly clear out our existing data structures, causing all pack ids to be invalidated.
            pub(crate) fn refresh(&mut self) -> io::Result<load_indices::Outcome> {
                self.db_paths = git_odb::alternate::resolve(&self.objects_directory)
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
                todo!()
            }

            /// If there is no handle with stable pack ids requirements, unload them.
            /// This property also relates to us pruning our internal state/doing internal maintenance which affects ids, too.
            pub(crate) fn may_unload_packs(&mut self) -> bool {
                self.num_handles_stable == 0
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
            use crate::features;
            use crate::odb::policy;
            use crate::odb::policy::{IndexId, PackId, PackIndexMarker};

            pub(crate) enum Outcome {
                /// Drop all data and fully replace it with `indices`.
                /// This happens if we have witnessed a generational change invalidating all of our ids and causing currently loaded
                /// indices and maps to be dropped.
                Replace {
                    indices: Vec<features::OwnShared<policy::IndexLookup>>, // should probably be small vec to get around most allocations
                    mark: PackIndexMarker, // use to show where the caller left off last time
                },
                /// Extend with the given indices and keep searching, while dropping invalidated/unloaded ids.
                Extend {
                    drop_packs: Vec<PackId>,    // which packs to forget about because they were removed from disk.
                    drop_indices: Vec<IndexId>, // which packs to forget about because they were removed from disk.
                    indices: Vec<features::OwnShared<policy::IndexLookup>>, // should probably be small vec to get around most allocations
                    mark: PackIndexMarker, // use to show where the caller left off last time
                },
                /// No new indices to look at, caller should give up
                NoMoreIndices,
            }
        }

        /// A snapshot about resource usage.
        pub struct StateInformation {
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
        state: features::MutableOnDemand<policy::State>,
    }

    impl Store {
        pub fn at(objects_directory: impl Into<PathBuf>) -> features::OwnShared<Self> {
            features::OwnShared::new(Store {
                state: features::MutableOnDemand::new(policy::State {
                    objects_directory: objects_directory.into(),
                    ..policy::State::default()
                }),
            })
        }

        /// Allow actually finding things
        pub fn to_handle(self: &features::OwnShared<Self>) -> Handle {
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
        pub fn state_snapshot(&self) -> policy::StateInformation {
            get_ref(&self.state).snapshot()
        }
    }

    /// Handle interaction
    impl Store {
        pub(crate) fn register_handle(&self) -> policy::HandleModeToken {
            let mut state = get_mut(&self.state);
            state.num_handles_unstable += 1;
            policy::HandleModeToken::Unstable
        }
        pub(crate) fn remove_handle(&self, mode: policy::HandleModeToken) {
            let mut state = get_mut(&self.state);
            match mode {
                policy::HandleModeToken::Stable => state.num_handles_stable -= 1,
                policy::HandleModeToken::Unstable => state.num_handles_unstable -= 1,
            }
        }
        pub(crate) fn upgrade_handle(&self, mode: policy::HandleModeToken) -> policy::HandleModeToken {
            if let policy::HandleModeToken::Unstable = mode {
                let mut state = get_mut(&self.state);
                state.num_handles_unstable -= 1;
                state.num_handles_stable += 1;
            }
            policy::HandleModeToken::Stable
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
        ) -> std::io::Result<Option<features::OwnShared<git_pack::data::File>>> {
            match id.multipack_index {
                None => {
                    let state = get_ref_upgradeable(&self.state);
                    // state.loaded_packs.get(id.index).map(|p| p.clone()).flatten()
                    // If there file on disk wasn't found, reconcile the on-disk state with our state right away and try again.
                    todo!()
                }
                Some(multipack_id) => todo!("load from given multipack which needs additional lookup"),
            }
        }
        pub(crate) fn load_next_indices(
            &self,
            refresh_mode: policy::RefreshMode,
            marker: Option<policy::PackIndexMarker>,
        ) -> std::io::Result<load_indices::Outcome> {
            let state = get_ref_upgradeable(&self.state);
            if state.db_paths.is_empty() {
                return upgrade_ref_to_mut(state).refresh();
            }

            Ok(match marker {
                Some(marker) => {
                    if marker.generation != state.generation {
                        load_indices::Outcome::Replace {
                            indices: state
                                .loaded_indices
                                .iter()
                                .filter_map(Option::as_ref)
                                .cloned()
                                .collect(),
                            mark: PackIndexMarker {
                                generation: state.generation,
                                pack_index_sequence: state.loaded_indices.len(),
                            },
                        }
                    } else {
                        if marker.pack_index_sequence == state.loaded_indices.len() {
                            match refresh_mode {
                                policy::RefreshMode::Never => load_indices::Outcome::NoMoreIndices,
                                policy::RefreshMode::AfterAllIndicesLoaded => {
                                    return upgrade_ref_to_mut(state).refresh()
                                }
                            }
                        } else {
                            load_indices::Outcome::Extend {
                                indices: state.loaded_indices[marker.pack_index_sequence..]
                                    .iter()
                                    .filter_map(Option::as_ref)
                                    .cloned()
                                    .collect(),
                                drop_packs: Vec::new(),
                                drop_indices: Vec::new(),
                                mark: PackIndexMarker {
                                    generation: state.generation,
                                    pack_index_sequence: state.loaded_indices.len(),
                                },
                            }
                        }
                    }
                }
                None => load_indices::Outcome::Replace {
                    indices: state
                        .loaded_indices
                        .iter()
                        .filter_map(Option::as_ref)
                        .cloned()
                        .collect(),
                    mark: PackIndexMarker {
                        generation: state.generation,
                        pack_index_sequence: state.loaded_indices.len(),
                    },
                },
            })
        }
    }

    /// The store shares a policy and keeps a couple of thread-local configuration
    pub struct Handle {
        store: features::OwnShared<Store>,
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
            self.mode.take().map(|mode| self.store.remove_handle(mode));
        }
    }

    impl git_odb::Find for Handle {
        type Error = git_odb::compound::find::Error;

        fn try_find<'a>(
            &self,
            id: impl AsRef<git_hash::oid>,
            buffer: &'a mut Vec<u8>,
            pack_cache: &mut impl DecodeEntry,
        ) -> Result<Option<Object<'a>>, Self::Error> {
            // TODO: if the generation changes, we need to clear the pack-cache as it depends on pack-ids.
            //       Can we simplify this so it's more obvious what generation does?
            todo!()
        }

        fn location_by_oid(&self, id: impl AsRef<git_hash::oid>, buf: &mut Vec<u8>) -> Option<Location> {
            todo!()
        }

        // TODO: turn this into a pack-id
        fn bundle_by_pack_id(&self, pack_id: u32) -> Option<&Bundle> {
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
    use crate::features;
    use std::path::{Path, PathBuf};

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
        use crate::features;
        use crate::refs::{LooseStore, RefTable};

        pub(crate) enum StoreSelection {
            Loose(LooseStore),
            RefTable(features::MutableOnDemand<RefTable>),
        }
    }

    pub struct Store {
        inner: inner::StoreSelection,
        packed_refs: features::MutableOnDemand<Option<git_ref::packed::Buffer>>,
        // And of course some more information to track if packed buffer is fresh
    }

    impl Store {
        pub fn new(path: impl AsRef<Path>) -> features::OwnShared<Self> {
            features::OwnShared::new(Store {
                inner: inner::StoreSelection::Loose(LooseStore {
                    path: path.as_ref().to_owned(),
                    reflog_mode: 0,
                }),
                packed_refs: features::MutableOnDemand::new(None),
            })
        }
        pub fn to_handle(self: &features::OwnShared<Self>) -> Handle {
            Handle {
                store: self.clone(),
                namespace: None,
            }
        }

        pub fn to_namespaced_handle(self: &features::OwnShared<Self>, namespace: git_ref::Namespace) -> Handle {
            Handle {
                store: self.clone(),
                namespace: namespace.into(),
            }
        }
    }

    #[derive(Clone)]
    pub struct Handle {
        store: features::OwnShared<Store>,
        namespace: Option<git_ref::Namespace>,
    }

    // impl common interface but check how all this works with iterators, there is some implementation around that already
    // and maybe this should just be its own State like thing… bet its own Easy so to say.
}

mod repository {
    use crate::{features, odb, refs};

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
        odb: features::OwnShared<odb::Store>,
        refs: features::OwnShared<refs::Store>,
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
