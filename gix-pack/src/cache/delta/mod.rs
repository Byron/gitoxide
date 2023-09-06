/// Returned when using various methods on a [`Tree`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Pack offsets must only increment. The previous pack offset was {last_pack_offset}, the current one is {pack_offset}")]
    InvariantIncreasingPackOffset {
        /// The last seen pack offset
        last_pack_offset: crate::data::Offset,
        /// The invariant violating offset
        pack_offset: crate::data::Offset,
    },
}

///
pub mod traverse;

///
pub mod from_offsets;

/// An item stored within the [`Tree`]
pub struct Item<T> {
    /// The offset into the pack file at which the pack entry's data is located.
    pub offset: crate::data::Offset,
    /// The offset of the next item in the pack file.
    pub next_offset: crate::data::Offset,
    /// Data to store with each Item, effectively data associated with each entry in a pack.
    pub data: T,
    /// Indices into our Tree's `items`, one for each pack entry that depends on us.
    ///
    /// Limited to u32 as that's the maximum amount of objects in a pack.
    children: Vec<u32>,
}

/// Identify what kind of node we have last seen
enum NodeKind {
    Root,
    Child,
}

/// A tree that allows one-time iteration over all nodes and their children, consuming it in the process,
/// while being shareable among threads without a lock.
/// It does this by making the guarantee that iteration only happens once.
pub struct Tree<T> {
    /// The root nodes, i.e. base objects
    root_items: Vec<Item<T>>,
    /// The child nodes, i.e. those that rely a base object, like ref and ofs delta objects
    child_items: Vec<Item<T>>,
    /// The last encountered node was either a root or a child.
    last_seen: Option<NodeKind>,
    /// Future child offsets, associating their offset into the pack with their index in the items array.
    /// (parent_offset, child_index)
    future_child_offsets: Vec<(crate::data::Offset, usize)>,
}

impl<T> Tree<T> {
    /// Instantiate a empty tree capable of storing `num_objects` amounts of items.
    pub fn with_capacity(num_objects: usize) -> Result<Self, Error> {
        Ok(Tree {
            root_items: Vec::with_capacity(num_objects / 2),
            child_items: Vec::with_capacity(num_objects / 2),
            last_seen: None,
            future_child_offsets: Vec::new(),
        })
    }

    fn num_items(&self) -> usize {
        self.root_items.len() + self.child_items.len()
    }

    fn assert_is_incrementing_and_update_next_offset(&mut self, offset: crate::data::Offset) -> Result<(), Error> {
        let items = match &self.last_seen {
            Some(NodeKind::Root) => &mut self.root_items,
            Some(NodeKind::Child) => &mut self.child_items,
            None => return Ok(()),
        };
        let item = &mut items.last_mut().expect("last seen won't lie");
        if offset <= item.offset {
            return Err(Error::InvariantIncreasingPackOffset {
                last_pack_offset: item.offset,
                pack_offset: offset,
            });
        }
        item.next_offset = offset;
        Ok(())
    }

    fn set_pack_entries_end_and_resolve_ref_offsets(
        &mut self,
        pack_entries_end: crate::data::Offset,
    ) -> Result<(), traverse::Error> {
        if !self.future_child_offsets.is_empty() {
            for (parent_offset, child_index) in self.future_child_offsets.drain(..) {
                if let Ok(i) = self.child_items.binary_search_by_key(&parent_offset, |i| i.offset) {
                    self.child_items[i].children.push(child_index as u32);
                } else if let Ok(i) = self.root_items.binary_search_by_key(&parent_offset, |i| i.offset) {
                    self.root_items[i].children.push(child_index as u32);
                } else {
                    return Err(traverse::Error::OutOfPackRefDelta {
                        base_pack_offset: parent_offset,
                    });
                }
            }
        }

        self.assert_is_incrementing_and_update_next_offset(pack_entries_end)
            .expect("BUG: pack now is smaller than all previously seen entries");
        Ok(())
    }

    /// Add a new root node, one that only has children but is not a child itself, at the given pack `offset` and associate
    /// custom `data` with it.
    pub fn add_root(&mut self, offset: crate::data::Offset, data: T) -> Result<(), Error> {
        self.assert_is_incrementing_and_update_next_offset(offset)?;
        self.last_seen = NodeKind::Root.into();
        self.root_items.push(Item {
            offset,
            next_offset: 0,
            data,
            children: Default::default(),
        });
        Ok(())
    }

    /// Add a child of the item at `base_offset` which itself resides at pack `offset` and associate custom `data` with it.
    pub fn add_child(
        &mut self,
        base_offset: crate::data::Offset,
        offset: crate::data::Offset,
        data: T,
    ) -> Result<(), Error> {
        self.assert_is_incrementing_and_update_next_offset(offset)?;

        let next_child_index = self.child_items.len();
        if let Ok(i) = self.child_items.binary_search_by_key(&base_offset, |i| i.offset) {
            self.child_items[i].children.push(next_child_index as u32);
        } else if let Ok(i) = self.root_items.binary_search_by_key(&base_offset, |i| i.offset) {
            self.root_items[i].children.push(next_child_index as u32);
        } else {
            self.future_child_offsets.push((base_offset, next_child_index));
        }

        self.last_seen = NodeKind::Child.into();
        self.child_items.push(Item {
            offset,
            next_offset: 0,
            data,
            children: Default::default(),
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    mod tree {
        mod from_offsets_in_pack {
            use std::sync::atomic::AtomicBool;

            use crate as pack;

            const SMALL_PACK_INDEX: &str = "objects/pack/pack-a2bf8e71d8c18879e499335762dd95119d93d9f1.idx";
            const SMALL_PACK: &str = "objects/pack/pack-a2bf8e71d8c18879e499335762dd95119d93d9f1.pack";

            const INDEX_V1: &str = "objects/pack/pack-c0438c19fb16422b6bbcce24387b3264416d485b.idx";
            const PACK_FOR_INDEX_V1: &str = "objects/pack/pack-c0438c19fb16422b6bbcce24387b3264416d485b.pack";

            use gix_testtools::fixture_path;

            #[test]
            fn v1() -> Result<(), Box<dyn std::error::Error>> {
                tree(INDEX_V1, PACK_FOR_INDEX_V1)
            }

            #[test]
            fn v2() -> Result<(), Box<dyn std::error::Error>> {
                tree(SMALL_PACK_INDEX, SMALL_PACK)
            }

            fn tree(index_path: &str, pack_path: &str) -> Result<(), Box<dyn std::error::Error>> {
                let idx = pack::index::File::at(fixture_path(index_path), gix_hash::Kind::Sha1)?;
                crate::cache::delta::Tree::from_offsets_in_pack(
                    &fixture_path(pack_path),
                    idx.sorted_offsets().into_iter(),
                    &|ofs| *ofs,
                    &|id| idx.lookup(id).map(|index| idx.pack_offset_at_index(index)),
                    &mut gix_features::progress::Discard,
                    &AtomicBool::new(false),
                    gix_hash::Kind::Sha1,
                )?;
                Ok(())
            }
        }
    }

    #[test]
    fn size_of_pack_tree_item() {
        use super::Item;
        assert_eq!(std::mem::size_of::<[Item<()>; 7_500_000]>(), 300_000_000);
    }

    #[test]
    fn size_of_pack_verify_data_structure() {
        use super::Item;
        pub struct EntryWithDefault {
            _index_entry: crate::index::Entry,
            _kind: gix_object::Kind,
            _object_size: u64,
            _decompressed_size: u64,
            _compressed_size: u64,
            _header_size: u16,
            _level: u16,
        }

        assert_eq!(std::mem::size_of::<[Item<EntryWithDefault>; 7_500_000]>(), 840_000_000);
    }
}
