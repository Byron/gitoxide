use git_hash::ObjectId;

///
pub mod objects;
pub use objects::to_entry_iter;

///
pub mod write;

///
pub mod entry;

/// The error returned by the pack generation function [`to_entry_iter()`][crate::pack::data::to_entry_iter()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error<LocateErr>
where
    LocateErr: std::error::Error + 'static,
{
    #[error(transparent)]
    Locate(#[from] LocateErr),
    #[error("Object id {oid} wasn't found in object database")]
    NotFound { oid: ObjectId },
    #[error("Entry expected to have hash {expected}, but it had {actual}")]
    PackToPackCopyCrc32Mismatch { actual: u32, expected: u32 },
    #[error(transparent)]
    NewEntry(entry::Error),
}

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
