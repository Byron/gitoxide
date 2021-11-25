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
    use std::io;
    use std::path::PathBuf;

    pub mod policy {
        use crate::features;
        use crate::odb::policy;
        use git_hash::oid;
        use std::collections::BTreeMap;
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

        pub(crate) struct IndexFile {
            file: index_file::SingleOrMulti,
            pub id: IndexId,
        }

        impl IndexFile {
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

        impl Default for PackDataUnloadMode {
            fn default() -> Self {
                PackDataUnloadMode::WhenDiskFileIsMissing
            }
        }

        type PackDataFiles = Vec<Option<features::OwnShared<git_pack::data::File>>>;

        pub(crate) type HandleId = u32;
        pub(crate) enum HandleMode {
            /// Pack-ids may change which may cause lookups by pack-id (without an oid available) to fail.
            Unstable,
            Stable,
        }

        #[derive(Default)]
        pub(crate) struct State {
            pub(crate) db_paths: Vec<PathBuf>,
            pub(crate) loaded_indices: Vec<Option<features::OwnShared<policy::IndexFile>>>,
            pub(crate) loaded_packs: PackDataFiles,
            /// Each change in the multi-pack index creates a new index entry here and typically drops all knowledge about its removed packs.
            /// We do this because there can be old pack ids around that refer to the old (now deleted) multi-pack index along with a possibly
            /// now invalid (or ambiguous) local pack id, i.e. pack A was meant, but now that's pack B because the multi-index has changed.
            pub(crate) loaded_packs_by_multi_index: BTreeMap<IndexId, PackDataFiles>,
            /// Generations are incremented whenever we decide to clear out our vectors if they are too big and contains too many empty slots.
            /// If we are not allowed to unload files, the generation will never be changed.
            pub(crate) generation: u8,
            pub(crate) next_handle_id: HandleId,
            pub(crate) handles: BTreeMap<HandleId, HandleMode>,
            pub(crate) objects_directory: PathBuf,
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

        /// Unloading here means to drop the shared reference to the mapped pack data file.
        ///
        /// Indices are always handled like that if their file on disk disappears as objects usually never disappear unless they are unreachable,
        /// meaning that new indices always contain the old objects in some way.
        pub enum PackDataUnloadMode {
            /// Keep pack data (and multi-pack index-to-pack lookup tables) always available in loaded memory maps even
            /// if the underlying data file (and usually index) are gone.
            /// This means algorithms that keep track of packs like pack-generators will always be able to resolve the data they reference.
            /// This also means, however, that one might run out of system resources some time, which means the coordinator of such users
            /// needs to check resource usage vs amount of uses and replace this instance with a new policy to eventually have the memory
            /// mapped packs drop (as references to them disappear once consumers disappear).
            /// We will also not ask Store handles to remove their pack data references.
            /// We will never rebuild our internal data structures to keep pack ids unique indefinitely (i.e. won't reuse a pack id with a different pack).
            Never,
            /// Forget/drop the mapped pack data file when its file on disk disappeared and store handles to remove their pack data references
            /// for them to be able to go out of scope.
            /// We are allowed to rebuild our internal data structures to save on memory but invalidate all previous pack ids.
            WhenDiskFileIsMissing,
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
                    indices: Vec<features::OwnShared<policy::IndexFile>>, // should probably be small vec to get around most allocations
                    mark: PackIndexMarker, // use to show where the caller left off last time
                },
                /// Extend with the given indices and keep searching, while dropping invalidated/unloaded ids.
                Extend {
                    drop_packs: Vec<PackId>,    // which packs to forget about because they were removed from disk.
                    drop_indices: Vec<IndexId>, // which packs to forget about because they were removed from disk.
                    indices: Vec<features::OwnShared<policy::IndexFile>>, // should probably be small vec to get around most allocations
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
            let next = self.register_handle();
            Handle {
                store: self.clone(),
                refresh_mode: policy::RefreshMode::AfterAllIndicesLoaded, // todo: remove this
                id: next,
            }
        }

        /// Get a snapshot of the current amount of handles and open packs and indices.
        /// If there are no handles, we are only consuming resources, which might indicate that this instance should be
        /// discarded.
        pub fn state_snapshot(&self) -> policy::StateInformation {
            let state = get_ref(&self.state);
            policy::StateInformation {
                num_handles: state.handles.len(),
                open_packs: state.loaded_packs.iter().filter(|p| p.is_some()).count()
                    + state
                        .loaded_packs_by_multi_index
                        .values()
                        .map(|packs| packs.iter().filter(|p| p.is_some()).count())
                        .sum::<usize>(),
                open_indices: state.loaded_indices.iter().filter(|i| i.is_some()).count(),
            }
        }
    }

    impl Store {
        pub(crate) fn register_handle(&self) -> policy::HandleId {
            let mut state = get_mut(&self.state);
            let next = state.next_handle_id;
            state.next_handle_id = state.next_handle_id.wrapping_add(1);
            state.handles.insert(next, policy::HandleMode::Unstable);
            next
        }
        pub(crate) fn remove_handle(&self, id: &policy::HandleId) {
            let mut state = get_mut(&self.state);
            state.handles.remove(id);
        }
        pub(crate) fn upgrade_handle(&self, id: &policy::HandleId) {
            let mut state = get_mut(&self.state);
            *state
                .handles
                .get_mut(&id)
                .expect("BUG: handles must be made known on creation") = policy::HandleMode::Stable;
        }

        /// If Ok(None) is returned, the pack-id was stale and referred to an unloaded pack.
        /// If the oid is known, just refresh indices
        /// and redo the entire lookup for a valid pack id whose pack can probably be loaded next time.
        /// Otherwise one should use or upgrade the handle to enforce stable indices.
        pub(crate) fn load_pack(
            &self,
            id: policy::PackId,
        ) -> std::io::Result<Option<features::OwnShared<git_pack::data::File>>> {
            match id.multipack_index {
                None => {
                    let state = get_ref_upgradeable(&self.state);
                    // state.loaded_packs.get(id.index).map(|p| p.clone()).flatten()
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
                let mut state = upgrade_ref_to_mut(state);
                return Self::refresh(&mut state);
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
                                    let mut state = upgrade_ref_to_mut(state);
                                    return Self::refresh(&mut state);
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

    /// Utilities
    impl Store {
        /// refresh and possibly clear out our existing data structures, causing all pack ids to be invalidated.
        fn refresh(state: &mut policy::State) -> io::Result<policy::load_indices::Outcome> {
            state.db_paths.extend(
                git_odb::alternate::resolve(&state.objects_directory)
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?,
            );
            todo!()
        }

        /// If there is no handle with stable pack ids requirements, unload them.
        /// This property also relates to us pruning our internal state/doing internal maintenance which affects ids, too.
        fn may_unload_packs(state: &policy::State) -> bool {
            state
                .handles
                .values()
                .all(|m| matches!(m, policy::HandleMode::Unstable))
        }
    }

    /// The store shares a policy and keeps a couple of thread-local configuration
    pub struct Handle {
        store: features::OwnShared<Store>,
        refresh_mode: policy::RefreshMode,
        id: policy::HandleId,
    }

    impl Handle {
        /// Call once if pack ids are stored and later used for lookup, meaning they should always remain mapped and not be unloaded
        /// even if they disappear from disk.
        /// This must be called if there is a chance that git maintenance is happening while a pack is created.
        pub fn prevent_pack_unload(&self) {
            self.store.upgrade_handle(&self.id);
        }
    }

    impl Clone for Handle {
        fn clone(&self) -> Self {
            Handle {
                store: self.store.clone(),
                refresh_mode: self.refresh_mode,
                id: self.store.register_handle(),
            }
        }
    }

    impl Drop for Handle {
        fn drop(&mut self) {
            self.store.remove_handle(&self.id)
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
