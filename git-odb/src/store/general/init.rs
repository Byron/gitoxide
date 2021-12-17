use std::{iter::FromIterator, path::PathBuf, sync::Arc};

use arc_swap::ArcSwap;

use crate::general::store::{MutableIndexAndPack, SlotMapIndex};

/// Configures the amount of slots in the index slotmap, which is fixed throughout the existence of the store.
pub enum Slots {
    /// The amount of slots to use, that is the total amount of indices we can hold at a time.
    /// Using this has the advantage of avoiding an initial directory listing of the repository, and is recommended
    /// on the server side where the repository setup is controlled.
    ///
    /// Note that this won't affect their packs, as each index can have one or more packs associated with it.
    Given(u16),
    /// Compute the amount of slots needed, as probably best used on the client side where a variety of repositories is encountered.
    AsNeededByDiskState {
        /// 1.0 means no safety, 1.1 means 10% more slots than needed
        multiplier: f32,
        /// The minimum amount of slots to assume
        minimum: usize,
    },
}

impl Default for Slots {
    fn default() -> Self {
        Slots::AsNeededByDiskState {
            multiplier: 1.1,
            minimum: 32,
        }
    }
}

impl super::Store {
    /// Open the store at `objects_dir` (containing loose objects and `packs/`), which must only be a directory for
    /// the store to be created without any additional work being done.
    /// `slots` defines how many multi-pack-indices as well as indices we can know about at a time, which includes
    /// the allowance for all additional object databases coming in via `alternates` as well.
    /// Note that the `slot_count` isn't used for packs, these are included with their multi-index or index respectively.
    /// In a repository with 250m objects and geometric packing one would expect 27 index/pack pairs, or a single multi-pack index.
    pub fn at_opts(objects_dir: impl Into<PathBuf>, slots: Slots) -> std::io::Result<Self> {
        let objects_dir = objects_dir.into();
        if !objects_dir.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other, // TODO: use NotADirectory when stabilized
                format!("'{}' wasn't a directory", objects_dir.display()),
            ));
        }
        let slot_count = match slots {
            Slots::Given(n) => n as usize,
            Slots::AsNeededByDiskState { multiplier, minimum } => {
                let mut db_paths = crate::alternate::resolve(&objects_dir)
                    .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
                db_paths.insert(0, objects_dir.clone());
                let num_slots = super::Store::collect_indices_and_mtime_sorted_by_size(db_paths, None)
                    .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?
                    .len();

                ((num_slots as f32 * multiplier) as usize).max(minimum)
            }
        };
        if slot_count >= 1 << 15 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Cannot use more than 1^15 slots",
            ));
        }
        Ok(super::Store {
            write: Default::default(),
            path: objects_dir,
            files: Vec::from_iter(std::iter::repeat_with(MutableIndexAndPack::default).take(slot_count)),
            index: ArcSwap::new(Arc::new(SlotMapIndex::default())),
            num_handles_stable: Default::default(),
            num_handles_unstable: Default::default(),
            num_disk_state_consolidation: Default::default(),
        })
    }
}
