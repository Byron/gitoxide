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
    pub fn at(objects_dir: impl Into<PathBuf>) -> std::io::Result<Self> {
        let objects_dir = objects_dir.into();
        if !objects_dir.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other, // TODO: use NotADirectory when stabilized
                format!("'{}' wasn't a directory", objects_dir.display()),
            ));
        }
        Ok(super::Store {
            path: parking_lot::Mutex::new(objects_dir),
            files: Vec::from_iter(std::iter::repeat_with(MutableIndexAndPack::default).take(256)), // TODO: figure this out from the amount of files currently present
            index: ArcSwap::new(Arc::new(SlotMapIndex::default())),
            num_handles_stable: Default::default(),
            num_handles_unstable: Default::default(),
            num_disk_state_consolidation: Default::default(),
        })
    }
}
