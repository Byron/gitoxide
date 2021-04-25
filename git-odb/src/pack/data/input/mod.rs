use crate::pack;
use std::io;

/// Returned by [`EntriesFromBytesIter::new_from_header()`] and as part of `Item` of [`EntriesFromBytesIter`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("An IO operation failed while streaming an entry")]
    Io(#[from] io::Error),
    #[error(transparent)]
    PackParse(#[from] pack::data::header::decode::Error),
    #[error("pack checksum in trailer was {expected}, but actual checksum was {actual}")]
    ChecksumMismatch {
        expected: git_hash::ObjectId,
        actual: git_hash::ObjectId,
    },
    #[error("pack is incomplete: it was decompressed into {actual} bytes but {expected} bytes where expected.")]
    IncompletePack { actual: u64, expected: u64 },
}

/// An item of the iteration produced by [`EntriesFromBytesIter`]
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry {
    /// The header of a pack entry
    pub header: pack::data::entry::Header,
    /// The amount of bytes used to encode the `header`. `pack_offset + header_size` is the beginning of
    /// the compressed data in the pack.
    pub header_size: u16,
    /// The first byte of the entry at which the `header` can be read.
    pub pack_offset: u64,
    /// The bytes consumed while producing `decompressed`
    /// These do not contain the header, which makes it possible to easily replace a RefDelta with offset deltas
    /// when resolving thin packs.
    /// Depends on `CompressionMode` when the iterator is initialized.
    pub compressed: Option<Vec<u8>>,
    /// The amount of bytes the compressed portion of the entry takes, i.e. the portion behind behind the header.
    pub compressed_size: u64,
    /// The CRC32 over the complete entry, that is encoded header and compressed object data.
    /// Depends on `CompressionMode` when the iterator is initialized
    pub crc32: Option<u32>,
    /// The amount of decompressed bytes of the entry.
    pub decompressed_size: u64,
    /// Set for the last object in the iteration, providing the hash over all bytes of the iteration
    /// for use as trailer in a pack or to verify it matches the trailer.
    pub trailer: Option<git_hash::ObjectId>,
}

/// Iteration Mode
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Mode {
    /// Provide the trailer as read from the pack
    AsIs,
    /// Generate an own hash and trigger an error on the last iterated object
    /// if it does not match the hash provided with the pack.
    ///
    /// This way the one iterating the data cannot miss corruption as long as
    /// the iteration is continued through to the end.
    Verify,
    /// Generate an own hash and if there was an error or the objects are depleted early
    /// due to partial packs, return the last valid entry and with our own hash thus far.
    /// Note that the existing pack hash, if present, will be ignored.
    /// As we won't know which objects fails, every object will have the hash obtained thus far.
    /// This also means that algorithms must know about this possibility, or else might wrongfully
    /// assume the pack is finished.
    Restore,
}

/// Define what to do with the compressed bytes portion of a pack [`Entry`]
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum EntryDataMode {
    /// Do nothing with the compressed bytes we read
    Ignore,
    /// Only create a CRC32 of the entry, otherwise similar to `Ignore`
    Crc32,
    /// Keep them and pass them along in a newly allocated buffer
    Keep,
    /// As above, but also compute a CRC32
    KeepAndCrc32,
}

impl EntryDataMode {
    /// Returns true if a crc32 should be computed
    pub fn crc32(&self) -> bool {
        match self {
            EntryDataMode::KeepAndCrc32 | EntryDataMode::Crc32 => true,
            EntryDataMode::Keep | EntryDataMode::Ignore => false,
        }
    }
    /// Returns true if compressed bytes should be kept
    pub fn keep(&self) -> bool {
        match self {
            EntryDataMode::Keep | EntryDataMode::KeepAndCrc32 => true,
            EntryDataMode::Ignore | EntryDataMode::Crc32 => false,
        }
    }
}

mod iter;
pub use iter::EntriesFromBytesIter;
