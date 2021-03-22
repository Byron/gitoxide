use std::cell::UnsafeCell;

/// Returned when using various methods on a [`Tree`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Pack offsets must only increment. The previous pack offset was {last_pack_offset}, the current one is {pack_offset}")]
    InvariantIncreasingPackOffset {
        /// The last seen pack offset
        last_pack_offset: u64,
        /// The invariant violating offset
        pack_offset: u64,
    },
    #[error("Is there ever a need to create empty indices? If so, please post a PR.")]
    InvariantNonEmpty,
    #[error("The delta at pack offset {delta_pack_offset} could not find its base at {base_pack_offset} - it should have been seen already")]
    InvariantBasesBeforeDeltasNeedThem {
        /// The delta pack offset whose base we could not find
        delta_pack_offset: u64,
        /// The base pack offset which was not yet added to the tree
        base_pack_offset: u64,
    },
}

mod iter;
pub use iter::{Chunks, Node};
///
pub mod traverse;

///
pub mod from_offsets;

/// An item stored within the [`Tree`]
pub struct Item<T> {
    /// The offset into the pack file at which the pack entry's data is located.
    pub offset: u64,
    /// If true, this object may have children but is not a child itself. It's thus the root of a tree.
    is_root: bool,
    /// Data to store with each Item, effectively data associated with each entry in a pack.
    pub data: T,
    children: Vec<usize>,
}
/// A tree that allows one-time iteration over all nodes and their children, consuming it in the process,
/// while being shareable among threads without a lock.
/// It does this by making the run-time guarantee that iteration only happens once.
pub struct Tree<T> {
    items: UnsafeCell<Vec<Item<T>>>,
    last_added_offset: u64,
    one_past_last_seen_root: usize,
    pack_entries_end: Option<u64>,
}

/// SAFETY: We solemnly swear…that this is sync because without the unsafe cell, it is also sync.
/// But that's really the only reason why I would dare to know.
#[allow(unsafe_code)]
unsafe impl<T> Sync for Tree<T> {}

impl<T> Tree<T> {
    /// Instantiate a empty tree capable of storing `num_objects` amounts of items.
    pub fn with_capacity(num_objects: usize) -> Result<Self, Error> {
        if num_objects == 0 {
            return Err(Error::InvariantNonEmpty);
        }
        Ok(Tree {
            items: UnsafeCell::new(Vec::with_capacity(num_objects)),
            last_added_offset: 0,
            one_past_last_seen_root: 0,
            pack_entries_end: None,
        })
    }

    fn assert_is_incrementing(&mut self, offset: u64) -> Result<u64, Error> {
        if offset > self.last_added_offset {
            self.last_added_offset = offset;
            Ok(offset)
        } else {
            Err(Error::InvariantIncreasingPackOffset {
                last_pack_offset: self.last_added_offset,
                pack_offset: offset,
            })
        }
    }

    /// Add a new root node, one that only has children but is not a child itself, at the given pack `offset` and associate
    /// custom `data` with it.
    pub fn add_root(&mut self, offset: u64, data: T) -> Result<(), Error> {
        // SAFETY: Because we passed the assertion above which implies no other access is possible as per
        // standard borrow check rules.
        #[allow(unsafe_code)]
        let items = unsafe { &mut *(self.items.get()) };
        let offset = self.assert_is_incrementing(offset)?;
        items.push(Item {
            offset,
            data,
            is_root: true,
            children: Default::default(),
        });
        self.one_past_last_seen_root = items.len();
        Ok(())
    }

    /// Add a child of the item at `base_offset` which itself resides at pack `offset` and associate custom `data` with it.
    pub fn add_child(&mut self, base_offset: u64, offset: u64, data: T) -> Result<(), Error> {
        // SAFETY: Because we passed the assertion above which implies no other access is possible as per
        // standard borrow check rules.
        #[allow(unsafe_code)]
        let items = unsafe { &mut *(self.items.get()) };
        let offset = self.assert_is_incrementing(offset)?;
        let base_index = items.binary_search_by_key(&base_offset, |e| e.offset).map_err(|_| {
            Error::InvariantBasesBeforeDeltasNeedThem {
                delta_pack_offset: offset,
                base_pack_offset: base_offset,
            }
        })?;
        let child_index = items.len();
        items[base_index].children.push(child_index);
        items.push(Item {
            is_root: false,
            offset,
            data,
            children: Default::default(),
        });
        Ok(())
    }

    /// Transform this `Tree` into its items.
    pub fn into_items(self) -> Vec<Item<T>> {
        self.items.into_inner()
    }
}
