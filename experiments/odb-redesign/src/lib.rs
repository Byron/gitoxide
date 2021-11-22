#![allow(dead_code)]

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

pub mod policy {
    pub struct IndexMarker(u32);

    pub mod next_indices {
        use crate::policy::IndexMarker;

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

trait Policy {
    type PackData: Deref<Target = git_pack::data::File>;
    type PackIndex: Deref<Target = git_pack::index::File>;

    fn next_indices(&mut self) -> std::io::Result<policy::next_indices::Outcome<Self::PackIndex>>;
}

struct EagerLocal {}

type DynRcPolicy = dyn Policy<PackIndex = Rc<git_pack::data::File>, PackData = Rc<git_pack::data::File>>;
type DynArcPolicy = dyn Policy<PackIndex = Arc<git_pack::data::File>, PackData = Arc<git_pack::data::File>>;

/// Formerly RepositoryLocal
struct SharedLocal {
    pack_policy: Rc<RefCell<dyn Policy<PackIndex = Rc<git_pack::data::File>, PackData = Rc<git_pack::data::File>>>>,
}

/// Formerly Repository
struct Shared {
    pack_policy: Arc<parking_lot::Mutex<DynArcPolicy>>,
}

/// Using generics here would mean we need policy to handle its mutability itself, pushing it down might be easiest if generics
/// should be a thing.
/// Without generics, there would be a thread-safe and thread-local version of everything.
/// Maybe this should be solved with a feature toggle instead? Aka thread-safe or not?
struct SharedGeneric<PackPolicy: Policy> {
    pack_policy: PackPolicy,
}
