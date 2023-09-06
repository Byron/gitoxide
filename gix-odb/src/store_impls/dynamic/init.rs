use std::{iter::FromIterator, path::PathBuf, sync::Arc};

use arc_swap::ArcSwap;

use crate::{
    store::types::{MutableIndexAndPack, SlotMapIndex},
    Store,
};

/// Options for use in [`Store::at_opts()`].
#[derive(Clone, Debug)]
pub struct Options {
    /// How to obtain a size for the slot map.
    pub slots: Slots,
    /// The kind of hash we expect in our packs and would use for loose object iteration and object writing.
    pub object_hash: gix_hash::Kind,
    /// If false, no multi-pack indices will be used. If true, they will be used if their hash matches `object_hash`.
    pub use_multi_pack_index: bool,
    /// The current directory of the process at the time of instantiation.
    /// If unset, it will be retrieved using `std::env::current_dir()`.
    pub current_dir: Option<std::path::PathBuf>,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            slots: Default::default(),
            object_hash: Default::default(),
            use_multi_pack_index: true,
            current_dir: None,
        }
    }
}

/// Configures the amount of slots in the index slotmap, which is fixed throughout the existence of the store.
#[derive(Copy, Clone, Debug)]
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

impl Store {
    /// Open the store at `objects_dir` (containing loose objects and `packs/`), which must only be a directory for
    /// the store to be created without any additional work being done.
    /// `slots` defines how many multi-pack-indices as well as indices we can know about at a time, which includes
    /// the allowance for all additional object databases coming in via `alternates` as well.
    /// Note that the `slots` isn't used for packs, these are included with their multi-index or index respectively.
    /// For example, In a repository with 250m objects and geometric packing one would expect 27 index/pack pairs,
    /// or a single multi-pack index.
    /// `replacements` is an iterator over pairs of old and new object ids for replacement support.
    /// This means that when asking for object `X`, one will receive object `X-replaced` given an iterator like `Some((X, X-replaced))`.
    pub fn at_opts(
        objects_dir: PathBuf,
        replacements: &mut dyn Iterator<Item = (gix_hash::ObjectId, gix_hash::ObjectId)>,
        Options {
            slots,
            object_hash,
            use_multi_pack_index,
            current_dir,
        }: Options,
    ) -> std::io::Result<Self> {
        let _span = gix_features::trace::detail!("gix_odb::Store::at()");
        let current_dir = current_dir.map_or_else(std::env::current_dir, Ok)?;
        if !objects_dir.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other, // TODO: use NotADirectory when stabilized
                format!("'{}' wasn't a directory", objects_dir.display()),
            ));
        }
        let slot_count = match slots {
            Slots::Given(n) => n as usize,
            Slots::AsNeededByDiskState { multiplier, minimum } => {
                let mut db_paths = crate::alternate::resolve(objects_dir.clone(), &current_dir)
                    .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
                db_paths.insert(0, objects_dir.clone());
                let num_slots = super::Store::collect_indices_and_mtime_sorted_by_size(db_paths, None, None)
                    .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?
                    .len();

                ((num_slots as f32 * multiplier) as usize).max(minimum)
            }
        };
        if slot_count > crate::store::types::PackId::max_indices() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Cannot use more than 1^15 slots",
            ));
        }
        let mut replacements: Vec<_> = replacements.collect();
        replacements.sort_by(|a, b| a.0.cmp(&b.0));

        Ok(Store {
            current_dir,
            write: Default::default(),
            replacements,
            path: objects_dir,
            files: Vec::from_iter(std::iter::repeat_with(MutableIndexAndPack::default).take(slot_count)),
            index: ArcSwap::new(Arc::new(SlotMapIndex::default())),
            use_multi_pack_index,
            object_hash,
            num_handles_stable: Default::default(),
            num_handles_unstable: Default::default(),
            num_disk_state_consolidation: Default::default(),
        })
    }
}
