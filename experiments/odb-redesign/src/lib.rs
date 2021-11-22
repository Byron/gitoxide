#![allow(dead_code, unused_variables)]

mod features {
    pub mod threaded {
        use std::sync::Arc;

        #[cfg(feature = "thread-safe")]
        #[macro_export]
        macro_rules! marker_traits {
            ($target:ident, $trait:tt) => {
                pub trait $target: $trait + Send + Sync {}
            };
        }

        pub type OwnShared<T> = Arc<T>;
        pub type Mutable<T> = parking_lot::Mutex<T>;

        pub fn into_shared<T>(v: T) -> OwnShared<T> {
            Arc::new(v)
        }
        pub fn with_interior_mutability<T>(v: T) -> Mutable<T> {
            parking_lot::Mutex::new(v)
        }
    }

    pub(crate) mod local {
        use std::cell::RefCell;
        use std::rc::Rc;

        pub type OwnShared<T> = Rc<T>;
        pub type Mutable<T> = RefCell<T>;

        #[cfg(not(feature = "thread-safe"))]
        #[macro_export]
        macro_rules! marker_traits {
            ($target:ident, $trait:ty) => {
                type $target = dyn ($trait);
            };
        }

        pub fn into_shared<T>(v: T) -> Rc<T> {
            Rc::new(v)
        }
        pub fn with_interior_mutability<T>(v: T) -> Mutable<T> {
            RefCell::new(v)
        }
    }

    #[cfg(not(feature = "thread-safe"))]
    pub use local::*;
    #[cfg(feature = "thread-safe")]
    pub use threaded::*;
}

mod odb {
    use crate::features;
    use std::ops::Deref;

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

        fn next_indices(&mut self) -> std::io::Result<pack::next_indices::Outcome<Self::PackIndex>>;
    }

    #[derive(Default)]
    pub struct EagerLocal {
        bundles: features::local::Mutable<Vec<features::local::OwnShared<git_pack::Bundle>>>,
    }

    impl Policy for EagerLocal {
        type PackData = features::local::OwnShared<git_pack::data::File>;
        type PackIndex = features::local::OwnShared<git_pack::index::File>;

        fn next_indices(&mut self) -> std::io::Result<crate::odb::pack::next_indices::Outcome<Self::PackIndex>> {
            todo!()
        }
    }

    fn try_setup() {
        let policy = EagerLocal::default();
        // let policy = Box::new(EagerLocal::default())
        //     as Box<
        //         dyn DynPolicy<
        //             PackIndex = features::OwnShared<git_pack::data::File>,
        //             PackData = features::OwnShared<git_pack::data::File>,
        //         >,
        //     >;
    }

    crate::marker_traits!(DynPolicy, Policy);
}

mod repository {
    // type DynPolicy = dyn Policy<
    //         PackIndex = threading::OwnShared<git_pack::data::File>,
    //         PackData = threading::OwnShared<git_pack::data::File>,
    //     > + Send
    //     + Sync;

    use crate::{features, odb};
    // We probably don't need to use a macro like that as we have a feature toggle in Repository, or do we?
    // We need it, as otherwise there is no way to instantiate the correct version of the policy, or is there?
    // Should that be delegated to the caller, but if so that would lock them in to a choice and need custom code
    // depending on a feature toggle that they should only switch on or off.
    // crate::marker_traits!(DynPolicy, Policy);

    struct Repository {
        pack_policy: features::OwnShared<
            dyn odb::DynPolicy<
                PackIndex = features::OwnShared<git_pack::data::File>,
                PackData = features::OwnShared<git_pack::data::File>,
            >,
        >,
    }

    // /// Using generics here would mean we need policy to handle its mutability itself, pushing it down might be easiest if generics
    // /// should be a thing.
    // /// Without generics, there would be a thread-safe and thread-local version of everything.
    // /// Maybe this should be solved with a feature toggle instead? Aka thread-safe or not?
    // struct RepositoryGeneric<PackPolicy: Policy> {
    //     pack_policy: PackPolicy,
    // }
}
