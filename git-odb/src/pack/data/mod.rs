//! data within a pack file
use filebuffer::FileBuffer;
use std::{convert::TryInto, path::Path};

pub mod decode;
mod header;
pub use header::*;

pub mod init;
pub mod parse;
pub mod verify;

pub mod iter;
pub use iter::Iter;

pub type EntrySlice = std::ops::Range<u64>;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Kind {
    V2,
    V3,
}

pub struct File {
    data: FileBuffer,
    path: std::path::PathBuf,
    kind: Kind,
    num_objects: u32,
}

impl File {
    pub const HEADER_LEN: usize = 12;
    pub fn kind(&self) -> Kind {
        self.kind
    }
    pub fn num_objects(&self) -> u32 {
        self.num_objects
    }
    /// The length of all mapped data, including the pack header and the pack trailer
    pub fn data_len(&self) -> usize {
        self.data.len()
    }
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn entry_slice(&self, slice: EntrySlice) -> &[u8] {
        let entry_end: usize = slice.end.try_into().expect("end of pack fits into usize");
        let entry_start = slice.start as usize;
        &self.data[entry_start..entry_end]
    }

    /// Currently only done during pack verification - finding the right size is only possible by decompressing
    /// the pack entry beforehand, or by using the (to be sorted) offsets stored in an index file.
    pub fn entry_crc32(&self, pack_offset: u64, size: usize) -> u32 {
        let pack_offset: usize = pack_offset.try_into().expect("pack_size fits into usize");
        git_features::hash::crc32(&self.data[pack_offset..pack_offset + size])
    }
}
