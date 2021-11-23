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
        pub type Mutable<T> = RefCell<T>;

        pub fn get_mut<T>(v: &Mutable<T>) -> RefMut<'_, T> {
            v.borrow_mut()
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
        use std::path::PathBuf;

        /// Unloading here means to drop the shared reference to the mapped pack data file.
        ///
        /// Indices are always handled like that if their file on disk disappears.
        pub enum PackDataUnloadMode {
            /// Keep pack data always available in loaded memory maps even if the underlying data file (and usually index) are gone.
            /// This means algorithms that keep track of packs like pack-generators will always be able to resolve the data they reference.
            /// This also means, however, that one might run out of system resources some time, which means the coordinator of such users
            /// needs to check resource usage vs amount of uses and replace this instance with a new policy to eventually have the memory
            /// mapped packs drop (as references to them disappear once consumers disappear)
            Never,
            /// Forget/drop the mapped pack data file when its file on disk disappeared.
            WhenDiskFileIsMissing,
        }

        impl Default for PackDataUnloadMode {
            fn default() -> Self {
                PackDataUnloadMode::WhenDiskFileIsMissing
            }
        }

        #[derive(Default)]
        pub(crate) struct State {
            pub(crate) db_paths: Vec<PathBuf>,
            pub(crate) indices: Vec<features::OwnShared<git_pack::index::File>>,
            pub(crate) generation: u8,
            pub(crate) allow_unload: bool,
        }

        /// A way to indicate which pack indices we have seen already
        pub struct PackIndexMarker {
            /// The generation the marker belongs to, is incremented to indicate that there were changes that caused the removal of a
            /// pack and require the caller to rebuild their cache to free resources.
            pub generation: u8,
            /// An ever increasing number within a generation indicating the number of loaded pack indices. It's reset with
            /// each new generation.
            pub pack_index_sequence: usize,
        }

        /// Define how packs will be refreshed when all indices are loaded
        pub enum RefreshMode {
            /// Check for new or changed pack indices (and pack data files) when the last known index is loaded.
            /// Note that this might not yield stable pack data or index ids unless the Policy is set to never actually unload indices.
            /// The caller cannot know though.
            AfterAllIndicesLoaded,
            /// Use this if you expect a lot of missing objects that shouldn't trigger refreshes even after all packs are loaded.
            /// This comes at the risk of not learning that the packs have changed in the mean time.
            Never,
        }

        pub mod load_indices {
            use crate::odb::policy::{PackIndexMarker, RefreshMode};
            use std::path::Path;

            pub struct Options<'a> {
                pub objects_directory: &'a Path,
                pub mode: RefreshMode,
            }

            pub enum Outcome<IndexRef> {
                /// Replace your set with the given one
                Replace {
                    indices: Vec<IndexRef>,
                    mark: PackIndexMarker,
                },
                /// Extend with the given indices and keep searching
                Extend {
                    indices: Vec<IndexRef>, // should probably be small vec to get around most allocations
                    mark: PackIndexMarker,  // use to show where you left off next time you call
                },
                /// No new indices to look at, caller should stop give up
                NoMoreIndices,
            }
        }
    }

    #[derive(Default)]
    pub struct Policy {
        state: features::MutableOnDemand<policy::State>,
    }

    impl Policy {
        fn new(unload_mode: policy::PackDataUnloadMode) -> Self {
            Policy {
                state: features::MutableOnDemand::new(policy::State {
                    allow_unload: matches!(unload_mode, policy::PackDataUnloadMode::WhenDiskFileIsMissing),
                    ..policy::State::default()
                }),
            }
        }
    }

    impl Policy {
        pub fn load_next_indices(
            &self,
            load_indices::Options {
                objects_directory,
                mode,
            }: load_indices::Options<'_>,
            marker: Option<policy::PackIndexMarker>,
        ) -> std::io::Result<load_indices::Outcome<features::OwnShared<git_pack::index::File>>> {
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
                            indices: state.indices.clone(),
                            mark: PackIndexMarker {
                                generation: state.generation,
                                pack_index_sequence: state.indices.len(),
                            },
                        }
                    } else {
                        if marker.pack_index_sequence == state.indices.len() {
                            match mode {
                                policy::RefreshMode::Never => load_indices::Outcome::NoMoreIndices,
                                policy::RefreshMode::AfterAllIndicesLoaded => {
                                    let mut state = upgrade_ref_to_mut(state);
                                    return Self::refresh(&mut state, objects_directory);
                                }
                            }
                        } else {
                            load_indices::Outcome::Extend {
                                indices: state.indices[marker.pack_index_sequence..].to_vec(),
                                mark: PackIndexMarker {
                                    generation: state.generation,
                                    pack_index_sequence: state.indices.len(),
                                },
                            }
                        }
                    }
                }
                None => load_indices::Outcome::Replace {
                    indices: state.indices.clone(),
                    mark: PackIndexMarker {
                        generation: state.generation,
                        pack_index_sequence: state.indices.len(),
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
                indices: bundles,
                generation,
            }: &mut policy::State,
            objects_directory: &Path,
        ) -> io::Result<policy::load_indices::Outcome<features::OwnShared<git_pack::index::File>>> {
            db_paths.extend(
                git_odb::alternate::resolve(objects_directory)
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?,
            );
            todo!()
        }
    }

    /// The store shares a policy and
    pub struct Store {
        policy: features::OwnShared<Policy>,
        objects_directory: PathBuf,
    }

    impl git_odb::Find for Store {
        type Error = git_odb::compound::find::Error;

        fn try_find<'a>(
            &self,
            id: impl AsRef<git_hash::oid>,
            buffer: &'a mut Vec<u8>,
            pack_cache: &mut impl DecodeEntry,
        ) -> Result<Option<Object<'a>>, Self::Error> {
            todo!()
        }

        fn location_by_oid(&self, id: impl AsRef<git_hash::oid>, buf: &mut Vec<u8>) -> Option<Location> {
            todo!()
        }

        fn bundle_by_pack_id(&self, pack_id: u32) -> Option<&Bundle> {
            todo!()
        }

        fn entry_by_location(&self, location: &Location) -> Option<Entry<'_>> {
            todo!()
        }
    }

    fn try_setup() -> anyhow::Result<()> {
        let policy = Policy::default();
        Ok(())
    }
}

mod refs {
    use crate::features;
    use std::path::PathBuf;

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

    #[derive(Clone)]
    struct Store {
        inner: features::OwnShared<inner::StoreSelection>,
        namespace: u32,
    }

    // impl common interface but check how all this works with iterators, there is some implementation around that already
    // and maybe this should just be its own State like thing… bet its own Easy so to say.
}

mod repository {
    use crate::odb;

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
    pub struct Repository {
        odb: odb::Store,
    }
}
