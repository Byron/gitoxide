use crate::loose::{HEADER_READ_COMPRESSED_BYTES, HEADER_READ_UNCOMPRESSED_BYTES};
use git_object as object;
use smallvec::SmallVec;
use std::path::PathBuf;

/// A representation of a loose object on disk, which is fully or partially read into memory
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Object {
    /// The kind of object
    pub kind: object::Kind,
    /// The uncompressed size of the object's data/payload
    pub size: usize,
    pub(crate) decompressed_data: SmallVec<[u8; HEADER_READ_UNCOMPRESSED_BYTES]>,
    pub(crate) compressed_data: SmallVec<[u8; HEADER_READ_COMPRESSED_BYTES]>,
    pub(crate) header_size: usize,
    pub(crate) path: Option<PathBuf>,
    pub(crate) decompression_complete: bool,
}

impl Object {
    /// Writes all decompressed, raw data, into `buf`.
    pub fn data(&mut self, buf: &mut Vec<u8>) -> Result<(), decode::Error> {
        self.decompress_all()?;
        buf.resize(self.decompressed_data.len() - self.header_size, 0);
        buf.copy_from_slice(&self.decompressed_data[self.header_size..]);
        Ok(())
    }
}

///
pub mod decode;
///
pub mod header;
///
pub mod stream;
///
pub mod verify;
