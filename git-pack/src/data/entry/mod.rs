use crate::data::Entry;
use git_hash::SIZE_OF_SHA1_DIGEST as SHA1_SIZE;

const _TYPE_EXT1: u8 = 0;
const COMMIT: u8 = 1;
const TREE: u8 = 2;
const BLOB: u8 = 3;
const TAG: u8 = 4;
const _TYPE_EXT2: u8 = 5;
const OFS_DELTA: u8 = 6;
const REF_DELTA: u8 = 7;

/// Access
impl Entry {
    /// Compute the pack offset to the base entry of the object represented by this entry.
    pub fn base_pack_offset(&self, distance: u64) -> u64 {
        let pack_offset = self.data_offset - self.header_size() as u64;
        pack_offset.checked_sub(distance).expect("in-bound distance of deltas")
    }
    /// The pack offset at which this entry starts
    pub fn pack_offset(&self) -> u64 {
        self.data_offset - self.header_size() as u64
    }
    /// The amount of bytes used to describe this entry in the pack. The header starts at [`Self::pack_offset()`]
    pub fn header_size(&self) -> usize {
        self.header.size(self.decompressed_size)
    }
}

mod decode;

mod header;
pub use header::Header;
