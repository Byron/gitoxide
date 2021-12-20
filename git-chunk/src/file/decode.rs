pub use error::Error;
use std::convert::TryInto;
use std::ops::Range;

mod error {
    use quick_error::quick_error;
    quick_error! {
        /// The value returned by [crate::FileRef::from_bytes()
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            EarlySentinelValue {
                display("Sentinel value encountered while still processing chunks.")
            }
            MissingSentinelValue { actual: crate::Kind } {
                display("Sentinel value wasn't found, saw {:#016x}", actual)
            }
            ChunkSizeOutOfBounds { offset: crate::file::Offset, file_length: u64 } {
                display("The chunk offset {} went past the file of length {} - was it truncated?", offset, file_length)
            }
            DuplicateChunk(kind: crate::Kind) {
                display("The chunk of kind {:#016x} was encountered more than once", kind)
            }
            TocTooSmall { actual: usize, expected: usize } {
                display("The table of contents would be {} bytes, but got only {}", expected, actual)
            }
            Empty {
                display("Empty chunk indices are not allowed as the point of chunked files is to have chunks.")
            }
        }
    }
}

use crate::file;
use crate::file::index;

impl file::Index {
    /// Provided a mapped file at the beginning via `data`, starting at `toc_offset` decode all chunk information to return
    /// an index with `num_chunks` chunks.
    pub fn from_bytes(data: &[u8], toc_offset: usize, num_chunks: u32) -> Result<Self, Error> {
        if num_chunks == 0 {
            return Err(Error::Empty);
        }

        let data_len: u64 = data.len() as u64;
        let mut chunks = Vec::with_capacity(num_chunks as usize);
        let mut toc_entry = &data[toc_offset..];
        let expected_min_size = (num_chunks as usize + 1) * file::Index::ENTRY_SIZE;
        if toc_entry.len() < expected_min_size {
            return Err(Error::TocTooSmall {
                expected: expected_min_size,
                actual: toc_entry.len(),
            });
        }

        for _ in 0..num_chunks {
            let (kind, offset) = toc_entry.split_at(4);
            let kind = be_u32(kind);
            if kind == crate::SENTINEL {
                return Err(Error::EarlySentinelValue);
            }
            if chunks.iter().any(|c: &index::Entry| c.kind == kind) {
                return Err(Error::DuplicateChunk(kind));
            }

            let offset = be_u64(offset);
            if offset > data_len {
                return Err(Error::ChunkSizeOutOfBounds {
                    offset,
                    file_length: data_len,
                });
            }
            toc_entry = &toc_entry[file::Index::ENTRY_SIZE..];
            let next_offset = be_u64(&toc_entry[4..]);
            if next_offset > data_len {
                return Err(Error::ChunkSizeOutOfBounds {
                    offset: next_offset,
                    file_length: data_len,
                });
            }
            chunks.push(index::Entry {
                kind,
                offset: Range {
                    start: offset,
                    end: next_offset,
                },
            })
        }

        let sentinel = be_u32(&toc_entry[..4]);
        if sentinel != crate::SENTINEL {
            return Err(Error::MissingSentinelValue { actual: sentinel });
        }

        Ok(file::Index { chunks })
    }
}

fn be_u32(data: &[u8]) -> u32 {
    u32::from_be_bytes(data[..4].try_into().unwrap())
}

fn be_u64(data: &[u8]) -> u64 {
    u64::from_be_bytes(data[..8].try_into().unwrap())
}
