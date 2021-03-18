use crate::loose;
use flate2::Decompress;
use git_object as object;
use object::borrowed;
use smallvec::SmallVec;
use std::{io::Read, path::PathBuf};

/// Returned by [`loose::Object::decode()`] and [`loose::Object::stream()`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("decompression of object data failed")]
    Decompress(#[from] flate2::DecompressError),
    #[error(transparent)]
    Parse(#[from] borrowed::Error),
    #[error("Could not {action} data at '{path}'")]
    Io {
        source: std::io::Error,
        action: &'static str,
        path: PathBuf,
    },
}

// Decoding and streaming
impl loose::Object {
    /// Decode the object to make it's fields accessible in case of Trees, Tags and Commits.
    ///
    /// This is a zero-copy operation with data read from disk if needed and stored in memory.
    /// The returned [`borrowed::Object`] references this data where possible.
    ///
    /// **Note**: Blobs are also loaded into memory and are made available that way.
    /// Consider using `stream()` if large Blobs are expected.
    pub fn decode(&mut self) -> Result<borrowed::Object<'_>, Error> {
        self.decompress_all()?;
        let bytes = &self.decompressed_data[self.header_size..];
        Ok(borrowed::Object::from_bytes(self.kind, bytes)?)
    }

    /// Returns an implementation of [`std::io::Read`], which decompresses the objects data on the fly.
    ///
    /// **Note**: This is most useful for big blobs as these won't be read into memory in full. Use [`decode()`][loose::Object::decode()] for
    /// Trees, Tags and Commits instead for convenient access to their payload.
    pub fn stream(&mut self) -> Result<loose::object::stream::Reader<'_>, Error> {
        match &self.path {
            Some(path) => Ok(loose::object::stream::Reader::from_file(
                self.header_size,
                std::fs::File::open(path).map_err(|source| Error::Io {
                    source,
                    action: "open",
                    path: path.to_owned(),
                })?,
            )),
            None => {
                self.decompress_all()?;
                Ok(loose::object::stream::Reader::from_data(
                    self.header_size,
                    &self.decompressed_data.as_slice(),
                ))
            }
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
            let mut file = std::fs::File::open(&path).map_err(|source| Error::Io {
                source,
                action: "open",
                path: path.clone(),
            })?;
            let file_size = file
                .metadata()
                .map_err(|source| Error::Io {
                    source,
                    action: "read metadata",
                    path: path.clone(),
                })?
                .len() as usize;
            let mut buf = Vec::with_capacity(file_size);
            file.read_to_end(&mut buf).map_err(|source| Error::Io {
                source,
                action: "read",
                path,
            })?;
            self.compressed_data = SmallVec::from(buf);
        }
        let mut vec = Vec::with_capacity(std::cmp::min(self.compressed_data.len() * 2, usize::MAX));
        Decompress::new(true).decompress_vec(&self.compressed_data[..], &mut vec, flate2::FlushDecompress::None)?;
        self.decompressed_data = SmallVec::from(vec);
        self.compressed_data = Default::default();
        self.decompressed_data.shrink_to_fit();
        assert!(self.decompressed_data.len() == total_size);
        self.decompression_complete = true;
        Ok(())
    }
}
