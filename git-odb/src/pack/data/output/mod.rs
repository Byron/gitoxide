use crate::pack::data;
use git_hash::ObjectId;

///
pub mod objects;
pub use objects::to_entry_iter;

///
pub mod write;

///
pub mod entry;

/// An entry to be written to a file.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry {
    /// The hash of the object to write
    pub id: ObjectId,
    /// The kind of packed object
    pub object_kind: git_object::Kind,
    /// The kind of entry represented by `data`. It's used alongside with it to complete the pack entry
    /// at rest or in transit.
    pub entry_kind: entry::Kind,
    /// The size in bytes needed once `data` gets decompressed
    pub decompressed_size: usize,
    /// The compressed data right behind the header
    pub compressed_data: Vec<u8>,
}

impl Entry {
    /// Transform ourselves into pack entry header of `version` which can be written into a pack.
    ///
    /// `index_to_pack(nth_before) -> pack_offset` is a function to convert the base object's offset as index into an
    /// array to an offset into the pack. This information is known to the one calling the method.
    pub fn to_entry_header(
        &self,
        version: data::Version,
        index_to_pack: impl FnOnce(usize) -> u64,
    ) -> data::entry::Header {
        assert!(
            matches!(version, data::Version::V2),
            "we can only write V2 pack entries for now"
        );

        use entry::Kind::*;
        match self.entry_kind {
            Base => {
                use git_object::Kind::*;
                match self.object_kind {
                    Tree => data::entry::Header::Tree,
                    Blob => data::entry::Header::Blob,
                    Commit => data::entry::Header::Commit,
                    Tag => data::entry::Header::Tag,
                }
            }
            DeltaOid { id } => data::entry::Header::RefDelta { base_id: id.to_owned() },
            DeltaRef { nth_before } => data::entry::Header::OfsDelta {
                base_distance: index_to_pack(nth_before),
            },
        }
    }
}
