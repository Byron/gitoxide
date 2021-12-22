#![allow(missing_docs, unused)]

use filebuffer::FileBuffer;
use std::ops::Range;
use std::path::PathBuf;

/// Known multi-index file versions
#[derive(PartialEq, Eq, Ord, PartialOrd, Debug, Hash, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub enum Version {
    V1 = 1,
}

impl Default for Version {
    fn default() -> Self {
        Version::V1
    }
}

/// A representation of an index file for multiple packs at the same time, typically stored in a file
/// named 'multi-pack-index'.
pub struct File {
    data: FileBuffer,
    path: std::path::PathBuf,
    version: Version,
    hash_len: usize,
    object_hash: git_hash::Kind,
    num_chunks: u8,
    /// The amount of pack files contained within
    num_packs: u32,
    num_objects: u32,

    fan: [u32; 256],
    index_names: Vec<PathBuf>,
    lookup: Range<usize>,
    offsets: Range<usize>,
    large_offsets: Option<Range<usize>>,
}

///
pub mod access {
    use crate::multi_index::File;
    use std::convert::TryFrom;

    impl File {
        pub fn num_packs(&self) -> u32 {
            self.num_packs
        }
        pub fn num_objects(&self) -> u32 {
            self.num_objects
        }
        pub fn object_hash(&self) -> git_hash::Kind {
            self.object_hash
        }
        pub fn checksum(&self) -> git_hash::ObjectId {
            git_hash::ObjectId::from(&self.data[self.data.len() - self.hash_len..])
        }
    }
}

///
pub mod chunk;

///
pub mod init;
