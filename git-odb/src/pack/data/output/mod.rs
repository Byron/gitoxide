use git_hash::ObjectId;

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
    pub kind: entry::Kind,
    /// The size in bytes needed once `data` gets decompressed
    pub decompressed_size: usize,
    /// The compressed data right behind the header
    pub compressed_data: Vec<u8>,
}

///
pub mod entry;

///
pub mod objects_to_entries;
pub use objects_to_entries::objects_to_entries_iter;

///
pub mod entries_to_bytes;
pub use entries_to_bytes::EntriesToBytesIter;
