use flate2::Decompress;
use smallvec::SmallVec;
use std::{io::Read, path::PathBuf};

/// Returned by [`loose::Object::decode()`] and [`loose::Object::data()`].
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("decompression of object data failed")]
    Decompress(#[from] flate2::DecompressError),
    #[error("Could not {action} data at '{path}'")]
    Io {
        source: std::io::Error,
        action: &'static str,
        path: PathBuf,
    },
}

/// Data access
impl super::Object {
    /// Writes all decompressed, raw data, into `buf`.
    pub fn data(&mut self, buf: &mut Vec<u8>) -> Result<(), Error> {
        self.decompress_all()?;
        buf.resize(self.decompressed_data.len() - self.header_size, 0);
        buf.copy_from_slice(&self.decompressed_data[self.header_size..]);
        Ok(())
    }
}

impl super::Object {
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
