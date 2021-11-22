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
    use std::io;
    use std::ops::Deref;
    use std::path::Path;

    pub mod pack {
        pub struct IndexMarker(u32);

        pub mod next_indices {
            use crate::odb::pack::IndexMarker;

            pub enum Outcome<IndexRef> {
                Next {
                    indices: Vec<IndexRef>, // should probably be small vec to get around most allocations
                    mark: IndexMarker,      // use to show where you left off next time you call
                },
                /// No new indices to look at, caller should stop give up
                NoMoreIndices,
            }
        }
    }

    pub trait Policy {
        type PackData: Deref<Target = git_pack::data::File>;
        type PackIndex: Deref<Target = git_pack::index::File>;

        fn next_indices(
            &self,
            objects_directory: &Path,
            marker: Option<pack::IndexMarker>,
        ) -> std::io::Result<pack::next_indices::Outcome<Self::PackIndex>>;
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
            pub(crate) bundles: Vec<features::OwnShared<git_pack::Bundle>>,
        }
    }

    impl Policy for Eager {
        type PackData = features::OwnShared<git_pack::data::File>;
        type PackIndex = features::OwnShared<git_pack::index::File>;

        fn next_indices(
            &self,
            objects_directory: &Path,
            marker: Option<pack::IndexMarker>,
        ) -> std::io::Result<crate::odb::pack::next_indices::Outcome<Self::PackIndex>> {
            let mut state = get_mut(&self.state);
            if state.db_paths.is_empty() {
                state.db_paths.extend(
                    git_odb::alternate::resolve(objects_directory)
                        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?,
                );
            } else {
                debug_assert_eq!(
                    Some(objects_directory),
                    state.db_paths.get(0).map(|p| p.as_path()),
                    "Eager policy can't be shared across different repositories"
                );
            }

            // if bundles.is_empty()
            // match marker {
            //     Some(marker) if marker == bundles.len() => todo!()
            // }
            // git_odb::alternate::resolve(objects_directory)
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
