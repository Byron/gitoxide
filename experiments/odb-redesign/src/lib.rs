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
    use std::io;
    use std::ops::Deref;
    use std::path::Path;

    pub mod policy {
        /// A way to indicate which pack indices we have seen already
        pub struct PackIndexMarker {
            /// The generation the marker belongs to, is incremented on each refresh and possibly only if there is an actual change
            pub generation: u8,
            /// The amount of pack indices available
            pub pack_index_count: usize,
        }

        /// Define how packs will be refreshed when all indices are loaded
        pub enum RefreshMode {
            AfterAllIndicesLoaded,
            /// Use this if you expect a lot of missing objects that shouldn't trigger refreshes
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

    pub trait Policy {
        type PackData: Deref<Target = git_pack::data::File>;
        type PackIndex: Deref<Target = git_pack::index::File>;

        fn load_next_indices(
            &self,
            options: policy::load_indices::Options<'_>,
            marker: Option<policy::PackIndexMarker>,
        ) -> std::io::Result<policy::load_indices::Outcome<Self::PackIndex>>;
    }

    #[derive(Default)]
    pub struct Eager {
        state: features::Mutable<eager::State>,
    }

    mod eager {
        use crate::features;
        use std::path::PathBuf;

        #[derive(Default)]
        pub struct State {
            pub(crate) db_paths: Vec<PathBuf>,
            pub(crate) indices: Vec<features::OwnShared<git_pack::index::File>>,
            pub(crate) generation: u8,
        }
    }

    impl Policy for Eager {
        type PackData = features::OwnShared<git_pack::data::File>;
        type PackIndex = features::OwnShared<git_pack::index::File>;

        fn load_next_indices(
            &self,
            policy::load_indices::Options {
                objects_directory,
                mode,
            }: policy::load_indices::Options<'_>,
            marker: Option<policy::PackIndexMarker>,
        ) -> std::io::Result<crate::odb::policy::load_indices::Outcome<Self::PackIndex>> {
            let mut state = get_mut(&self.state);
            if state.db_paths.is_empty() {
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
                                pack_index_count: state.indices.len(),
                            },
                        }
                    } else {
                        if marker.pack_index_count == state.indices.len() {
                            match mode {
                                policy::RefreshMode::Never => load_indices::Outcome::NoMoreIndices,
                                policy::RefreshMode::AfterAllIndicesLoaded => {
                                    return Self::refresh(&mut state, objects_directory)
                                }
                            }
                        } else {
                            load_indices::Outcome::Replace {
                                indices: state.indices[marker.pack_index_count..].to_vec(),
                                mark: PackIndexMarker {
                                    generation: state.generation,
                                    pack_index_count: state.indices.len(),
                                },
                            }
                        }
                    }
                }
                None => load_indices::Outcome::Replace {
                    indices: state.indices.clone(),
                    mark: PackIndexMarker {
                        generation: state.generation,
                        pack_index_count: state.indices.len(),
                    },
                },
            })
        }
    }

    impl Eager {
        fn refresh(
            eager::State {
                db_paths,
                indices: bundles,
                generation,
            }: &mut eager::State,
            objects_directory: &Path,
        ) -> io::Result<policy::load_indices::Outcome<features::OwnShared<git_pack::index::File>>> {
            db_paths.extend(
                git_odb::alternate::resolve(objects_directory)
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?,
            );
            todo!()
        }
    }

    fn try_setup() -> anyhow::Result<()> {
        let policy = Eager::default();
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

    pub mod raw {
        use crate::odb;

        /// Using generics here would mean we need policy to handle its mutability itself, pushing it down might be easiest if generics
        /// should be a thing.
        /// Without generics, there would be a thread-safe and thread-local version of everything.
        /// Maybe this should be solved with a feature toggle instead? Aka thread-safe or not?
        pub struct RepositoryGeneric<PackPolicy: odb::Policy> {
            pack_policy: PackPolicy,
        }
    }

    /// Exposed type top-level repository to hide generic complexity, with one-size-fits-most default
    type Repository = raw::RepositoryGeneric<odb::Eager>;
}
