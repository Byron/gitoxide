use quick_error::quick_error;
use std::cell::UnsafeCell;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvariantIncreasingPackOffset(last_pack_offset: u64, pack_offset: u64) {
            display("Pack offsets must only increment. The previous pack offset was {}, the current one is {}", last_pack_offset, pack_offset)
        }
        InvariantNonEmpty {
            display("Is there ever a need to create empty indices? If so, please post a PR.")
        }
        InvariantBasesBeforeDeltasNeedThem(delta_pack_offset: u64, base_pack_offset: u64) {
            display("The delta at pack offset {} could not find its base at {} - it should have been seen already", delta_pack_offset, base_pack_offset)
        }
    }
}

mod iter;
pub use iter::{Chunks, Node};

pub mod from_offsets;

pub struct Item<T> {
    pub offset: u64,
    is_root: bool,
    pub data: T,
    // TODO: figure out average amount of children per node and use smallvec instead
    children: Vec<usize>,
}
/// A tree that allows one-time iteration over all nodes and their children, consuming it in the process,
/// while being shareable among threads without a lock.
/// It does this by making the run-time guarantee that iteration only happens once.
pub struct Tree<T> {
    items: UnsafeCell<Vec<Item<T>>>,
    last_added_offset: u64,
    one_past_last_seen_root: usize,
}

/// SAFETY: We solemnly swear…that this is sync because without the unsafe cell, it is also sync.
/// But that's really the only reason why I would dare to know.
#[allow(unsafe_code)]
unsafe impl<T> Sync for Tree<T> {}

impl<T> Tree<T> {
    pub fn with_capacity(num_objects: usize) -> Result<Self, Error> {
        if num_objects == 0 {
            return Err(Error::InvariantNonEmpty);
        }
        Ok(Tree {
            items: UnsafeCell::new(Vec::with_capacity(num_objects)),
            last_added_offset: 0,
            one_past_last_seen_root: 0,
        })
    }

    fn assert_is_incrementing(&mut self, offset: u64) -> Result<u64, Error> {
        if offset > self.last_added_offset {
            self.last_added_offset = offset;
            Ok(offset)
        } else {
            Err(Error::InvariantIncreasingPackOffset(self.last_added_offset, offset))
        }
    }

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

    pub fn add_child(&mut self, base_offset: u64, offset: u64, data: T) -> Result<(), Error> {
        // SAFETY: Because we passed the assertion above which implies no other access is possible as per
        // standard borrow check rules.
        #[allow(unsafe_code)]
        let items = unsafe { &mut *(self.items.get()) };
        let offset = self.assert_is_incrementing(offset)?;
        let base_index = items
            .binary_search_by_key(&base_offset, |e| e.offset)
            .map_err(|_| Error::InvariantBasesBeforeDeltasNeedThem(offset, base_offset))?;
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

    pub fn into_items(self) -> Vec<Item<T>> {
        self.items.into_inner()
    }
}
