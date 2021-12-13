#![allow(missing_docs, unused, dead_code)]

use std::cell::RefCell;
use std::{ops::Deref, path::PathBuf, sync::atomic::AtomicUsize};

use arc_swap::ArcSwap;

/// This effectively acts like a handle but exists to be usable from the actual `crate::Handle` implementation which adds caches on top.
/// Each store is quickly cloned and contains thread-local state for shared packs.
pub struct Handle<S>
where
    S: Deref<Target = Store> + Clone,
{
    pub(crate) store: S,
    pub refresh_mode: crate::RefreshMode,

    pub(crate) token: Option<handle::Mode>,
    snapshot: RefCell<load_index::Snapshot>,
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

pub mod load_index;

pub mod load_pack {
    // /// If Ok(None) is returned, the pack-id was stale and referred to an unloaded pack or a pack which couldn't be
    // /// loaded as its file didn't exist on disk anymore.
    // /// If the oid is known, just load indices again to continue
    // /// (objects rarely ever removed so should be present, maybe in another pack though),
    // /// and redo the entire lookup for a valid pack id whose pack can probably be loaded next time.
    // pub(crate) fn load_pack(
    //     &self,
    //     id: store::PackId,
    //     marker: SlotIndexMarker,
    // ) -> std::io::Result<Option<Arc<git_pack::data::File>>> {
    //     let index = self.index.load();
    //     if index.generation != marker.generation {
    //         return Ok(None);
    //     }
    //     match id.multipack_index {
    //         None => {
    //             let f = &self.files[id.index];
    //             match &**f.files.load() {
    //                 store::IndexAndPacks::Index(bundle) => match bundle.data.loaded() {
    //                     Some(pack) => Ok(Some(pack.clone())),
    //                     None => {
    //                         let _lock = f.write.lock();
    //                         // let f = &mut files[id.index];
    //                         // match f {
    //                         //     policy::IndexAndPacks::Index(bundle) => Ok(bundle
    //                         //         .data
    //                         //         .do_load(|path| {
    //                         //             git_pack::data::File::at(path).map(Arc::new).map_err(|err| match err {
    //                         //                 git_odb::pack::data::header::decode::Error::Io {
    //                         //                     source, ..
    //                         //                 } => source,
    //                         //                 other => std::io::Error::new(std::io::ErrorKind::Other, other),
    //                         //             })
    //                         //         })?
    //                         //         .cloned()),
    //                         //     _ => unreachable!(),
    //                         // }
    //                         todo!()
    //                     }
    //                 },
    //                 // This can also happen if they use an old index into our new and refreshed data which might have a multi-index
    //                 // here.
    //                 store::IndexAndPacks::MultiIndex(_) => Ok(None),
    //             }
    //         }
    //         Some(multipack_id) => todo!("load from given multipack which needs additional lookup"),
    //     }
    // }
}

mod metrics;
