use crate::{
    loose::{HEADER_READ_COMPRESSED_BYTES, HEADER_READ_UNCOMPRESSED_BYTES},
    zlib,
};
use git_object as object;
use miniz_oxide::inflate::decompress_to_vec_zlib;
use object::borrowed;
use quick_error::quick_error;
use smallvec::SmallVec;
use std::{io::Read, path::PathBuf};

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
        Io(err: std::io::Error, action: &'static str, path: PathBuf) {
            display("Could not {} file at '{}'", action, path.display())
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
    pub(crate) path: Option<PathBuf>,
    pub(crate) decompression_complete: bool,
}

impl Object {
    // Note: Blobs are loaded or mapped into memory and are made available that way.
    // Consider the streaming API if large Blobs are expected.
    pub fn decode(&mut self) -> Result<borrowed::Object, Error> {
        self.decompress_all()?;
        let bytes = &self.decompressed_data[self.header_size..];
        Ok(match self.kind {
            object::Kind::Tag => borrowed::Object::Tag(borrowed::Tag::from_bytes(bytes)?),
            object::Kind::Tree => borrowed::Object::Tree(borrowed::Tree::from_bytes(bytes)?),
            object::Kind::Commit => borrowed::Object::Commit(borrowed::Commit::from_bytes(bytes)?),
            object::Kind::Blob => borrowed::Object::Blob(borrowed::Blob { data: bytes }),
        })
    }

    fn decompress_all(&mut self) -> Result<(), Error> {
        if self.decompression_complete {
            debug_assert!(
                self.size + self.header_size == self.decompressed_data.len(),
                "when decompression is done, we have stored everything in memory"
            );
            return Ok(());
        }
        let total_size = self.header_size + self.size;
        if let Some(path) = self.path.take() {
            // NOTE: For now we just re-read everything from the beginning without seeking, as our buffer
            // is small so the seek might be more expensive than just reading everything.
            let mut file =
                std::fs::File::open(&path).map_err(|e| Error::Io(e, "open", path.clone()))?;
            let file_size = file
                .metadata()
                .map_err(|e| Error::Io(e, "read metadata", path.clone()))?
                .len() as usize;
            let mut buf = Vec::with_capacity(file_size);
            file.read_to_end(&mut buf)
                .map_err(|e| Error::Io(e, "read", path))?;
            self.compressed_data = SmallVec::from(buf);
        }
        self.decompressed_data =
            SmallVec::from(decompress_to_vec_zlib(&self.compressed_data[..]).unwrap());
        self.decompressed_data.shrink_to_fit();
        assert!(self.decompressed_data.len() == total_size);
        self.decompression_complete = true;
        self.compressed_data = Default::default();
        Ok(())
    }
}
