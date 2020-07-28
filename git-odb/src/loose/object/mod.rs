use crate::loose::{HEADER_READ_COMPRESSED_BYTES, HEADER_READ_UNCOMPRESSED_BYTES};
use git_object as object;
use smallvec::SmallVec;
use std::path::PathBuf;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Object {
    pub kind: object::Kind,
    pub size: usize,
    pub(crate) decompressed_data: SmallVec<[u8; HEADER_READ_UNCOMPRESSED_BYTES]>,
    pub(crate) compressed_data: SmallVec<[u8; HEADER_READ_COMPRESSED_BYTES]>,
    pub(crate) header_size: usize,
    pub(crate) path: Option<PathBuf>,
    pub(crate) decompression_complete: bool,
}

pub mod decode;
pub mod header;
pub mod stream;
pub(crate) mod verify;
