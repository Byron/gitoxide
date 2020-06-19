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
    }
}

pub struct Object {
    pub kind: object::Kind,
    pub size: usize,
    pub(crate) decompressed_data: SmallVec<[u8; HEADER_READ_UNCOMPRESSED_BYTES]>,
    pub(crate) compressed_data: SmallVec<[u8; HEADER_READ_COMPRESSED_BYTES]>,
    pub(crate) header_size: usize,
    pub(crate) _path: Option<PathBuf>,
    pub(crate) is_decompressed: bool,
}

impl Object {
    pub fn parsed(&mut self) -> Result<borrowed::Object, Error> {
        Ok(match self.kind {
            object::Kind::Tag | object::Kind::Commit | object::Kind::Tree => {
                if !self.is_decompressed {
                    let total_size = self.header_size + self.size;
                    let cap = self.decompressed_data.capacity();
                    if cap < total_size {
                        self.decompressed_data.reserve_exact(total_size - cap);
                    }
                    // This works because above we assured there is total_size bytes available.
                    // Those may not be initialized, but it will be overwritten entirely by zlib
                    // which decompresses everything into the memory region.
                    #[allow(unsafe_code)]
                    unsafe {
                        assert!(self.decompressed_data.capacity() >= total_size);
                        self.decompressed_data.set_len(total_size);
                    }
                    let mut cursor = Cursor::new(&mut self.decompressed_data[..]);
                    // TODO Performance opportunity
                    // here we do some additional work as we decompress parts again that we already covered
                    // when getting the header, if we could re-use the previous state.
                    // This didn't work for some reason in 2018! Maybe worth another try
                    let mut deflate = zlib::Inflate::default();
                    deflate.all_till_done(&self.compressed_data[..], &mut cursor)?;
                    self.is_decompressed = deflate.is_done;
                    debug_assert!(deflate.is_done);
                    self.compressed_data = Default::default();
                }
                let bytes = &self.decompressed_data[self.header_size..];
                match self.kind {
                    object::Kind::Tag => borrowed::Object::Tag(borrowed::Tag::from_bytes(bytes)?),
                    _ => unimplemented!(),
                }
            }
            object::Kind::Blob => unimplemented!("todo implement blob streaming!"),
        })
    }
}
