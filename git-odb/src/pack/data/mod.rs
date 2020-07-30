//! data within a pack file
use filebuffer::FileBuffer;
use std::path::Path;

pub mod decode;
mod decoded;
pub use decoded::*;

pub mod init;
pub mod parse;
pub mod verify;

pub mod iter;
pub use iter::Iter;

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
}
