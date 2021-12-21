//! a pack data file
use std::{convert::TryInto, path::Path};

use filebuffer::FileBuffer;

/// An representing an full- or delta-object within a pack
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry {
    /// The entry's header
    pub header: entry::Header,
    /// The decompressed size of the object in bytes
    pub decompressed_size: u64,
    /// absolute offset to compressed object data in the pack, just behind the entry's header
    pub data_offset: u64,
}

mod file;
pub use file::{decode_entry, verify, ResolvedBase};
///
pub mod header;

///
pub mod entry;

///
pub mod input;

/// Utilities to encode pack data entries and write them to a `Write` implementation to resemble a pack data file.
pub mod output;

/// A slice into a pack file denoting a pack entry.
///
/// An entry can be decoded into an object.
pub type EntryRange = std::ops::Range<u64>;

/// Supported versions of a pack data file
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub enum Version {
    V2,
    V3,
}

impl Default for Version {
    fn default() -> Self {
        Version::V2
    }
}

/// A pack data file
pub struct File {
    data: FileBuffer,
    path: std::path::PathBuf,
    /// A value to represent this pack uniquely when used with cache lookup, or a way to identify this pack by its location on disk.
    /// The same location on disk should yield the same id.
    ///
    /// These must be unique per pack and must be stable, that is they don't change if the pack doesn't change.
    /// If the same id is assigned (or reassigned) to different packs, pack creation or cache access will fail in hard-to-debug ways.
    ///
    /// This value is controlled by the owning object store, which can use it in whichever way it wants as long as the above constraints are met.
    pub id: u32,
    version: Version,
    num_objects: u32,
}

/// Information about the pack data file itself
impl File {
    /// The pack data version of this file
    pub fn version(&self) -> Version {
        self.version
    }
    /// The number of objects stored in this pack data file
    pub fn num_objects(&self) -> u32 {
        self.num_objects
    }
    /// The length of all mapped data, including the pack header and the pack trailer
    pub fn data_len(&self) -> usize {
        self.data.len()
    }

    /// The position of the byte one past the last pack entry, or in other terms, the first byte of the trailing hash.
    pub fn pack_end(&self) -> usize {
        self.data.len() - git_hash::Kind::Sha1.len_in_bytes()
    }

    /// The path to the pack data file on disk
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the pack data at the given slice if its range is contained in the mapped pack data
    pub fn entry_slice(&self, slice: EntryRange) -> Option<&[u8]> {
        let entry_end: usize = slice.end.try_into().expect("end of pack fits into usize");
        let entry_start = slice.start as usize;
        self.data.get(entry_start..entry_end)
    }

    /// Returns the CRC32 of the pack data indicated by `pack_offset` and the `size` of the mapped data.
    ///
    /// _Note:_ finding the right size is only possible by decompressing
    /// the pack entry beforehand, or by using the (to be sorted) offsets stored in an index file.
    ///
    /// # Panics
    ///
    /// If `pack_offset` or `size` are pointing to a range outside of the mapped pack data.
    pub fn entry_crc32(&self, pack_offset: u64, size: usize) -> u32 {
        let pack_offset: usize = pack_offset.try_into().expect("pack_size fits into usize");
        git_features::hash::crc32(&self.data[pack_offset..pack_offset + size])
    }
}

pub(crate) mod delta;
