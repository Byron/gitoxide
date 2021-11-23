#![allow(dead_code, unused_variables)]

mod features {
    mod threaded {
        use std::sync::Arc;

        pub type OwnShared<T> = Arc<T>;
        pub type Mutable<T> = parking_lot::Mutex<T>;

        pub fn get_mut<T>(v: &Mutable<T>) -> parking_lot::MutexGuard<'_, T> {
            v.lock()
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
    use crate::features::get_mut;
    use crate::odb::policy::{load_indices, PackIndexMarker};
    use git_odb::data::Object;
    use git_odb::pack::bundle::Location;
    use git_odb::pack::cache::DecodeEntry;
    use git_odb::pack::find::Entry;
    use git_odb::pack::Bundle;
    use std::io;
    use std::ops::DerefMut;
    use std::path::{Path, PathBuf};

    pub mod policy {

        pub(crate) enum State {
            Eager(eager::State),
        }

        pub(crate) mod eager {
            use crate::features;
            use std::path::PathBuf;

            #[derive(Default)]
            pub struct State {
                pub(crate) db_paths: Vec<PathBuf>,
                pub(crate) indices: Vec<features::OwnShared<git_pack::index::File>>,
                pub(crate) generation: u8,
            }
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
            /// Check for new or changed pack indices when the last known index is loaded.
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

    pub struct Policy {
        state: features::Mutable<policy::State>,
    }

    impl Policy {
        fn eager() -> Self {
            Policy {
                state: features::Mutable::new(policy::State::Eager(policy::eager::State::default())),
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
            let mut state = get_mut(&self.state);
            match state.deref_mut() {
                policy::State::Eager(state) => {
                    if state.db_paths.is_empty() {
                        return Self::refresh(state, objects_directory);
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
                                            return Self::refresh(state, objects_directory)
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
            }
        }

        /// If there is only additive changes, there is no need for a new `generation` actually, which helps
        /// callers to retain stability.
        fn refresh(
            policy::eager::State {
                db_paths,
                indices: bundles,
                generation,
            }: &mut policy::eager::State,
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
    pub struct PolicyStore {
        policy: features::OwnShared<Policy>,
        objects_directory: PathBuf,
    }

    impl git_odb::Find for PolicyStore {
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
        let policy = Policy::eager();
        // let policy = Box::new(EagerLocal::default())
        //     as Box<
        //         dyn DynPolicy<
        //             PackIndex = features::OwnShared<git_pack::data::File>,
        //             PackData = features::OwnShared<git_pack::data::File>,
        //         >,
        //     >;
        Ok(())
    }
}

mod repository {
    use crate::odb;

    mod raw {
        use git_pack::Find;

        pub struct Repository<Odb>
        where
            Odb: Find, // + Contains + Refresh/Reset maybe?
        {
            odb: Odb,
        }
    }

    /// Maybe we will end up providing a generic version as there still seems to be benefits in having a read-only Store implementation.
    pub struct Repository {
        odb: odb::PolicyStore,
    }
}
