#![allow(dead_code, unused_variables)]

mod features {
    mod threaded {
        use std::sync::Arc;

        pub type OwnShared<T> = Arc<T>;
        pub type MutableOnDemand<T> = parking_lot::RwLock<T>;

        pub fn get_ref<T>(v: &MutableOnDemand<T>) -> parking_lot::RwLockUpgradableReadGuard<'_, T> {
            v.upgradable_read()
        }

        pub fn upgrade_ref_to_mut<T>(
            v: parking_lot::RwLockUpgradableReadGuard<'_, T>,
        ) -> parking_lot::RwLockWriteGuard<'_, T> {
            parking_lot::RwLockUpgradableReadGuard::upgrade(v)
        }
    }

    mod local {
        use std::cell::{RefCell, RefMut};
        use std::rc::Rc;

        pub type OwnShared<T> = Rc<T>;
        pub type MutableOnDemand<T> = RefCell<T>;

        pub fn get_ref<T>(v: &RefCell<T>) -> RefMut<'_, T> {
            v.borrow_mut()
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
    use crate::features::{get_ref, upgrade_ref_to_mut};
    use crate::odb::policy::{load_indices, PackIndexMarker};
    use git_odb::data::Object;
    use git_odb::pack::bundle::Location;
    use git_odb::pack::cache::DecodeEntry;
    use git_odb::pack::find::Entry;
    use git_odb::pack::Bundle;
    use std::io;
    use std::path::{Path, PathBuf};

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
            pub(crate) allow_unload: bool,
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
            use crate::odb::policy::{IndexId, PackId, PackIndexMarker, RefreshMode};
            use std::path::Path;

            pub(crate) struct Options<'a> {
                pub objects_directory: &'a Path,
                pub refresh_mode: RefreshMode,
            }

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
    }

    pub struct Store {
        state: features::MutableOnDemand<policy::State>,
    }

    impl Store {
        pub fn new(unload_mode: policy::PackDataUnloadMode) -> features::OwnShared<Self> {
            features::OwnShared::new(Store {
                state: features::MutableOnDemand::new(policy::State {
                    allow_unload: matches!(unload_mode, policy::PackDataUnloadMode::WhenDiskFileIsMissing),
                    ..policy::State::default()
                }),
            })
        }
    }

    impl Store {
        /// If Ok(None) is returned, the pack-id was stale and referred to an unloaded pack.
        /// If pack-ids are kept long enough for this to happen or things are racy, the store policy must be changed to never unload packs
        /// along with regular cleanup to not run out of handles while getting some reuse, or if the oid is known, just refresh indices
        /// and redo the entire lookup for a valid pack id whose pack can probably be loaded next time.
        pub(crate) fn load_pack(
            &self,
            id: policy::PackId,
        ) -> std::io::Result<Option<features::OwnShared<git_pack::data::File>>> {
            match id.multipack_index {
                None => {
                    let state = get_ref(&self.state);
                    // state.loaded_packs.get(id.index).map(|p| p.clone()).flatten()
                    todo!()
                }
                Some(multipack_id) => todo!("load from given multipack which needs additional lookup"),
            }
        }
        pub(crate) fn load_next_indices(
            &self,
            load_indices::Options {
                objects_directory,
                refresh_mode,
            }: load_indices::Options<'_>,
            marker: Option<policy::PackIndexMarker>,
        ) -> std::io::Result<load_indices::Outcome> {
            let state = get_ref(&self.state);
            if state.db_paths.is_empty() {
                let mut state = upgrade_ref_to_mut(state);
                return Self::refresh(&mut state, objects_directory);
            } else {
                debug_assert_eq!(
                    Some(objects_directory),
                    state.db_paths.get(0).map(|p| p.as_path()),
                    "Eager policy can't be shared across different repositories"
                );
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
                                    return Self::refresh(&mut state, objects_directory);
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

        /// If there is only additive changes, there is no need for a new `generation` actually, which helps
        /// callers to retain stability.
        fn refresh(
            policy::State {
                db_paths,
                allow_unload: _,
                loaded_indices: bundles,
                loaded_packs: _,
                loaded_packs_by_multi_index: _,
                generation,
            }: &mut policy::State,
            objects_directory: &Path,
        ) -> io::Result<policy::load_indices::Outcome> {
            db_paths.extend(
                git_odb::alternate::resolve(objects_directory)
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?,
            );
            todo!()
        }
    }

    /// The store shares a policy and keeps a couple of thread-local configuration
    pub struct Handle {
        store: features::OwnShared<Store>,
        objects_directory: PathBuf,
        refresh_mode: policy::RefreshMode,
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
        let policy = Store::new(policy::PackDataUnloadMode::Never);
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
                odb: odb::Store::new(odb::policy::PackDataUnloadMode::WhenDiskFileIsMissing),
                refs: refs::Store::new("./foo"),
            };
            spawn(&repository);
            spawn(repository);
        }
    }
}
