//! a pack data file
use std::{convert::TryInto, path::Path};

use filebuffer::FileBuffer;

use git_hash::SIZE_OF_SHA1_DIGEST as SHA1_SIZE;

mod file;
pub use file::{decode_entry, verify, ResolvedBase};
///
pub mod header;

///
pub mod entry;
#[doc(inline)]
pub use entry::Entry;

pub mod object;
pub use object::Object;

///
pub mod input;
#[doc(inline)]
pub use input::BytesToEntriesIter;

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
    /// A hash to represent the `path` field when used with cache lookup, or a way to identify this pack by its location on disk.
    ///
    /// Note that `path` might not be canonicalized, thus different hashes might actually refer to the same pack on disk. This will
    /// only lead to less efficient cache usage.
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
        self.data.len() - SHA1_SIZE
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
