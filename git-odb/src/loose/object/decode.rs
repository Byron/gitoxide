use super::stream;
use crate::{loose, zlib};
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
            source(err)
        }
        Parse(err: borrowed::Error) {
            display("Could not parse object object")
            from()
            source(err)
        }
        Io(err: std::io::Error, action: &'static str, path: PathBuf) {
            display("Could not {} data at '{}'", action, path.display())
            source(err)
        }
    }
}

impl loose::Object {
    /// **Note**: Blobs are loaded into memory and are made available that way.
    /// Consider using `stream()` if large Blobs are expected.
    pub fn decode(&mut self) -> Result<borrowed::Object, Error> {
        self.decompress_all()?;
        let bytes = &self.decompressed_data[self.header_size..];
        Ok(borrowed::Object::from_bytes(self.kind, bytes)?)
    }

    pub fn stream(&self) -> Result<stream::Reader, Error> {
        match &self.path {
            Some(path) => Ok(stream::Reader::from_read(
                self.header_size,
                std::fs::File::open(path).map_err(|e| Error::Io(e, "open", path.to_owned()))?,
            )),
            None => Ok(stream::Reader::from_data(
                self.header_size,
                &self.decompressed_data.as_slice(),
            )),
        }
    }

    pub(crate) fn decompress_all(&mut self) -> Result<(), Error> {
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
            let mut file = std::fs::File::open(&path).map_err(|e| Error::Io(e, "open", path.clone()))?;
            let file_size = file
                .metadata()
                .map_err(|e| Error::Io(e, "read metadata", path.clone()))?
                .len() as usize;
            let mut buf = Vec::with_capacity(file_size);
            file.read_to_end(&mut buf).map_err(|e| Error::Io(e, "read", path))?;
            self.compressed_data = SmallVec::from(buf);
        }
        self.decompressed_data = SmallVec::from(decompress_to_vec_zlib(&self.compressed_data[..]).unwrap());
        self.compressed_data = Default::default();
        self.decompressed_data.shrink_to_fit();
        assert!(self.decompressed_data.len() == total_size);
        self.decompression_complete = true;
        Ok(())
    }
}
