//! Low-level access to reading and writing chunk file based formats.
//!
//! See the [git documentation](https://github.com/git/git/blob/seen/Documentation/technical/chunk-format.txt) for details.
#![deny(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]

use std::convert::TryInto;
use std::ops::Range;

/// An identifier to describe the kind of chunk, unique within a chunk file.
pub type Kind = u32;

/// A special value denoting the end of the chunk file table of contents.
pub const SENTINEL: Kind = 0;

/// Turn a u64 Range into a usize range safely, to make chunk ranges useful in memory mapped files.
pub fn into_usize_range(Range { start, end }: Range<file::Offset>) -> Option<Range<usize>> {
    let start = start.try_into().ok()?;
    let end = end.try_into().ok()?;
    Some(Range { start, end })
}

///
pub mod file {
    ///
    pub mod index {
        use crate::file::Index;
        use std::ops::Range;

        ///
        pub mod offset_by_kind {
            use std::fmt::{Display, Formatter};

            /// The error returned by [Index::offset_by_kind()][super::Index::offset_by_kind()].
            #[allow(missing_docs)]
            #[derive(Debug)]
            pub struct Error {
                pub kind: crate::Kind,
                pub name: &'static str,
            }

            impl Display for Error {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    write!(
                        f,
                        "Chunk named {:?} (id = {}) was not found in chunk file index",
                        self.name, self.kind
                    )
                }
            }

            impl std::error::Error for Error {}
        }

        ///
        pub mod data_by_kind {
            use quick_error::quick_error;
            quick_error! {
                /// The error returned by [Index::data_by_kind()][super::Index::data_by_kind()].
                #[derive(Debug)]
                #[allow(missing_docs)]
                pub enum Error {
                    NotFound(err: super::offset_by_kind::Error) {
                        display("The chunk wasn't found in the file index")
                        from()
                        source(err)
                    }
                    FileTooLarge {
                        display("The offsets into the file couldn't be represented by usize")
                    }
                }
            }
        }

        /// An entry of a chunk file index
        pub struct Entry {
            /// The kind of the chunk file
            pub kind: crate::Kind,
            /// The offset, relative to the beginning of the file, at which to find the chunk and its end.
            pub offset: Range<crate::file::Offset>,
        }

        impl Index {
            /// The size of a single index entry in bytes
            pub const ENTRY_SIZE: usize = std::mem::size_of::<u32>() + std::mem::size_of::<u64>();
            /// The smallest possible size of an index, consisting only of the sentinel value pointing past itself.
            pub const EMPTY_SIZE: usize = Index::ENTRY_SIZE;

            /// Find a chunk of `kind` and return its offset into the data if found
            pub fn offset_by_kind(
                &self,
                kind: crate::Kind,
                name: &'static str,
            ) -> Result<Range<crate::file::Offset>, offset_by_kind::Error> {
                self.chunks
                    .iter()
                    .find_map(|c| (c.kind == kind).then(|| c.offset.clone()))
                    .ok_or_else(|| offset_by_kind::Error { kind, name })
            }

            /// Find a chunk of `kind` and return its data slice based on its offset.
            pub fn data_by_kind<'a>(
                &self,
                data: &'a [u8],
                kind: crate::Kind,
                name: &'static str,
            ) -> Result<&'a [u8], data_by_kind::Error> {
                let offset = self.offset_by_kind(kind, name)?;
                Ok(&data[crate::into_usize_range(offset).ok_or_else(|| data_by_kind::Error::FileTooLarge)?])
            }
        }
    }

    /// The offset to a chunk as seen relative to the beginning of the file containing it.
    pub type Offset = u64;

    /// A chunk file providing a table into the parent data.
    pub struct Index {
        /// Validated chunks as defined by their index entries.
        pub chunks: Vec<index::Entry>,
    }
    ///
    pub mod decode {
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
    }
}
