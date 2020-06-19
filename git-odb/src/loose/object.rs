use crate::{
    loose::{HEADER_READ_COMPRESSED_BYTES, HEADER_READ_UNCOMPRESSED_BYTES},
    zlib,
};
use git_object as object;
use object::borrowed;
use quick_error::quick_error;
use smallvec::SmallVec;
use std::{io::Cursor, path::PathBuf};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Decompress(err: zlib::Error) {
            display("decompression of object data failed")
            from()
            cause(err)
        }
        ParseTag(err: borrowed::Error) {
            display("Could not parse tag object")
            from()
            cause(err)
        }
        BlobsCannotBeParsed {
            display("Blob objects cannot be parsed - they can only be streamed")
        }
    }
}

pub struct Object {
    pub kind: object::Kind,
    pub size: usize,
    pub(crate) decompressed_data: SmallVec<[u8; HEADER_READ_UNCOMPRESSED_BYTES]>,
    pub(crate) compressed_data: SmallVec<[u8; HEADER_READ_COMPRESSED_BYTES]>,
    pub(crate) header_size: usize,
    pub(crate) path: Option<PathBuf>,
    pub(crate) decompression_complete: bool,
}

impl Object {
    pub fn parsed(&mut self) -> Result<borrowed::Object, Error> {
        Ok(match self.kind {
            object::Kind::Tag | object::Kind::Commit | object::Kind::Tree => {
                if !self.decompression_complete {
                    let total_size = self.header_size + self.size;
                    if self.decompressed_data.capacity() < total_size {
                        self.decompressed_data
                            .reserve_exact(total_size - self.decompressed_data.len());
                    }
                    self.decompressed_data.resize(total_size, 0);
                    let mut cursor = Cursor::new(&mut self.decompressed_data[..]);
                    // TODO Performance opportunity
                    // here we do some additional work as we decompress parts again that we already covered
                    // when getting the header, if we could re-use the previous state.
                    // This didn't work for some reason in 2018! Maybe worth another try
                    let mut deflate = zlib::Inflate::default();
                    deflate.all_till_done(&self.compressed_data[..], &mut cursor)?;
                    self.decompression_complete = deflate.is_done;
                    debug_assert!(deflate.is_done);
                    self.compressed_data = Default::default();
                }
                let bytes = &self.decompressed_data[self.header_size..];
                match self.kind {
                    object::Kind::Tag => borrowed::Object::Tag(borrowed::Tag::from_bytes(bytes)?),
                    object::Kind::Tree => {
                        borrowed::Object::Tree(borrowed::Tree::from_bytes(bytes)?)
                    }
                    object::Kind::Commit => {
                        borrowed::Object::Commit(borrowed::Commit::from_bytes(bytes)?)
                    }
                    object::Kind::Blob => unreachable!("Blobs are handled in another branch"),
                }
            }
            object::Kind::Blob => return Err(Error::BlobsCannotBeParsed),
        })
    }
}
