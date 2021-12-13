use std::{
    iter::FromIterator,
    ops::Deref,
    path::PathBuf,
    sync::{atomic::AtomicUsize, Arc},
};

use arc_swap::ArcSwap;
use git_features::threading::OwnShared;

use crate::general::{
    store,
    store::{MutableIndexAndPack, SlotMapIndex},
};

impl super::Store {
    /// Open the store at `objects_dir` (containing loose objects and `packs/`), which must only be a directory for
    /// the store to be created without any additional work being done.
    /// `slot_count` defines how many multi-pack-indices as well as indices we can know about at a time, which includes
    /// the allowance for all additional object databases coming in via `alternates` as well.
    /// Note that the `slot_count` isn't used for packs, these are included with their multi-index or index respectively.
    /// In a repository with 250m objects and geometric packing one would expect 27 index/pack pairs, or a single multi-pack index.
    pub fn at_opts(objects_dir: impl Into<PathBuf>, slot_count: usize) -> std::io::Result<Self> {
        let objects_dir = objects_dir.into();
        if !objects_dir.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other, // TODO: use NotADirectory when stabilized
                format!("'{}' wasn't a directory", objects_dir.display()),
            ));
        }
        Ok(super::Store {
            path: parking_lot::Mutex::new(objects_dir),
            files: Vec::from_iter(std::iter::repeat_with(MutableIndexAndPack::default).take(slot_count)),
            index: ArcSwap::new(Arc::new(SlotMapIndex::default())),
            num_handles_stable: Default::default(),
            num_handles_unstable: Default::default(),
            num_disk_state_consolidation: Default::default(),
        })
    }
}
