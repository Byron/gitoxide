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

/// An index into our [`File::index_names()`] array yielding the name of the index and by impclication, its pack file.
pub type PackIndex = u32;

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
    lookup_ofs: usize,
    offsets_ofs: usize,
    large_offsets_ofs: Option<usize>,
}

///
pub mod access;

///
pub mod chunk;

///
pub mod init;
