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
#[allow(clippy::empty_docs)]
pub mod traverse;

///
#[allow(clippy::empty_docs)]
pub mod from_offsets;

/// Tree datastructure
// kept in separate module to encapsulate unsafety (it has field invariants)
mod tree;

pub use tree::{Item, Tree};

#[cfg(test)]
mod tests {

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
