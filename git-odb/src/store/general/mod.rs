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
    pub refresh_mode: crate::RefreshMode,

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

    /// The amount of times we re-read the disk state to consolidate our in-memory representation.
    pub(crate) num_disk_state_consolidation: AtomicUsize,
}

mod find;

mod init;

pub mod store;

pub mod handle;

pub mod load_indices;

mod metrics;
