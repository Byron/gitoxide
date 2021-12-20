#![allow(missing_docs, unused)]

use filebuffer::FileBuffer;
use std::ops::Range;
use std::path::PathBuf;

/// Known multi-index file versions
#[derive(PartialEq, Eq, Ord, PartialOrd, Debug, Hash, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub enum Version {
    V1 = 1,
}

impl Default for Version {
    fn default() -> Self {
        Version::V1
    }
}

/// A representation of an index file for multiple packs at the same time, typically stored in a file
/// named 'multi-pack-index'.
pub struct File {
    data: FileBuffer,
    path: std::path::PathBuf,
    version: Version,
    hash_kind: git_hash::Kind,
    num_chunks: u8,
    /// The amount of pack files contained within
    num_packs: u32,
    num_objects: u32,

    fan: [u32; 256],
    index_names: Vec<PathBuf>,
    lookup: Range<git_chunk::file::Offset>,
    offsets: Range<git_chunk::file::Offset>,
    large_offsets: Option<Range<git_chunk::file::Offset>>,
    checksum_offset: usize,
}

///
pub mod access {
    use crate::multi_index::File;
    use std::convert::TryFrom;

    impl File {
        pub fn num_packs(&self) -> u32 {
            self.num_packs
        }
        pub fn num_objects(&self) -> u32 {
            self.num_objects
        }
        pub fn hash_kind(&self) -> git_hash::Kind {
            self.hash_kind
        }
        pub fn checksum(&self) -> git_hash::ObjectId {
            git_hash::ObjectId::try_from(&self.data[self.checksum_offset..]).expect("prior validation of checksum size")
        }
    }
}

pub mod chunk {
    pub mod pack_names {
        use git_object::bstr::{BString, ByteSlice};
        use os_str_bytes::OsStrBytes;
        use std::path::{Path, PathBuf};

        pub const ID: git_chunk::Kind = 0x504e414d; /* "PNAM" */

        pub mod from_slice {
            use git_object::bstr::BString;

            #[derive(Debug, thiserror::Error)]
            pub enum Error {
                #[error("The pack names were not ordered alphabetically.")]
                NotOrderedAlphabetically,
                #[error("Each pack path name must be terminated with a null byte")]
                MissingNullByte,
                #[error("Couldn't turn path '{path}' into OS path due to encoding issues")]
                PathEncoding { path: BString },
            }
        }

        pub fn from_slice(mut chunk: &[u8], num_packs: u32) -> Result<Vec<PathBuf>, from_slice::Error> {
            let mut out = Vec::new();
            for _ in 0..num_packs {
                let null_byte_pos = chunk.find_byte(b'\0').ok_or(from_slice::Error::MissingNullByte)?;

                let path = &chunk[..null_byte_pos];
                let path = Path::from_raw_bytes(path)
                    .map_err(|_| from_slice::Error::PathEncoding {
                        path: BString::from(path),
                    })?
                    .into_owned();

                if let Some(previous) = out.last() {
                    if previous >= &path {
                        return Err(from_slice::Error::NotOrderedAlphabetically);
                    }
                }
                out.push(path);

                chunk = &chunk[null_byte_pos + 1..];
            }

            // NOTE: git writes garbage into this chunk, usually extra \0 bytes, which we simply ignore. If we were strict
            // about it we couldn't read this chunk data at all.
            Ok(out)
        }
    }
    pub mod fanout {
        use std::convert::TryInto;

        pub const ID: git_chunk::Kind = 0x4f494446; /* "OIDF" */

        pub fn from_slice(chunk: &[u8]) -> Option<[u32; 256]> {
            if chunk.len() != 4 * 256 {
                return None;
            }
            let mut out = [0; 256];
            for (c, f) in chunk.chunks(4).zip(out.iter_mut()) {
                *f = u32::from_be_bytes(c.try_into().unwrap());
            }
            out.into()
        }
    }
    pub mod lookup {
        use git_chunk::file::Offset;
        use git_hash::Kind;
        use std::ops::Range;

        pub const ID: git_chunk::Kind = 0x4f49444c; /* "OIDL" */

        pub fn is_valid(offset: &Range<Offset>, hash: git_hash::Kind, num_objects: u32) -> bool {
            (offset.end - offset.start) as usize / hash.len_in_bytes() == num_objects as usize
        }
    }
    pub mod offsets {
        use git_chunk::file::Offset;
        use std::ops::Range;

        pub const ID: git_chunk::Kind = 0x4f4f4646; /* "OOFF" */

        pub fn is_valid(offset: &Range<Offset>, num_objects: u32) -> bool {
            let entry_size = 4 /* pack-id */ + 4 /* pack-offset */;
            ((offset.end - offset.start) as usize / num_objects as usize) == entry_size
        }
    }
    pub mod large_offsets {
        use git_chunk::file::Offset;
        use std::ops::Range;

        pub const ID: git_chunk::Kind = 0x4c4f4646; /* "LOFF" */
        pub fn is_valid(offset: Option<&Range<Offset>>) -> bool {
            offset
                .map(|offset| (offset.end - offset.start) as usize % 8 == 0)
                .unwrap_or(true)
        }
    }
}

///
pub mod init {
    use crate::multi_index::{chunk, File, Version};
    use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
    use filebuffer::FileBuffer;
    use std::convert::{TryFrom, TryInto};
    use std::path::Path;

    mod error {
        use crate::multi_index::chunk;

        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error("Could not open multi-index file at '{path}'")]
            Io {
                source: std::io::Error,
                path: std::path::PathBuf,
            },
            #[error("{message}")]
            Corrupt { message: &'static str },
            #[error("Unsupported multi-index version: {version})")]
            UnsupportedVersion { version: u8 },
            #[error("Unsupported hash kind: {kind})")]
            UnsupportedHashKind { kind: u8 },
            #[error(transparent)]
            ChunkFileDecode(#[from] git_chunk::file::decode::Error),
            #[error(transparent)]
            MissingChunk(#[from] git_chunk::file::index::offset_by_kind::Error),
            #[error(transparent)]
            FileTooLarge(#[from] git_chunk::file::index::data_by_kind::Error),
            #[error("The multi-pack fan doesn't have the correct size of 256 * 4 bytes")]
            MultiPackFanSize,
            #[error(transparent)]
            PackNames(#[from] chunk::pack_names::from_slice::Error),
            #[error("The chunk with alphabetically ordered object ids doesn't have the correct size")]
            OidLookupSize,
            #[error("The chunk with offsets into the pack doesn't have the correct size")]
            OffsetsSize,
            #[error("The chunk with large offsets into the pack doesn't have the correct size")]
            LargeOffsetsSize,
        }
    }
    pub use error::Error;

    impl File {
        pub fn at(path: impl AsRef<Path>) -> Result<Self, Error> {
            Self::try_from(path.as_ref())
        }
    }

    impl TryFrom<&Path> for File {
        type Error = Error;

        fn try_from(path: &Path) -> Result<Self, Self::Error> {
            let data = FileBuffer::open(path).map_err(|source| Error::Io {
                source,
                path: path.to_owned(),
            })?;

            const HEADER_LEN: usize = 4 /*signature*/ +
                        1 /*version*/ +
                        1 /*object id version*/ +
                        1 /*num chunks */ +
                        1 /*num base files */ +
                        4 /*num pack files*/;
            const TRAILER_LEN: usize = git_hash::Kind::longest().len_in_bytes(); /* trailing hash */
            if data.len() < HEADER_LEN + git_chunk::file::Index::EMPTY_SIZE + TRAILER_LEN {
                return Err(Error::Corrupt {
                    message: "multi-index file is truncated and too short",
                });
            }

            let (version, hash_kind, num_chunks, num_packs) = {
                let (signature, data) = data.split_at(4);
                if signature != b"MIDX" {
                    return Err(Error::Corrupt {
                        message: "Invalid signature",
                    });
                }
                let (version, data) = data.split_at(1);
                let version = match version[0] {
                    1 => Version::V1,
                    version => return Err(Error::UnsupportedVersion { version }),
                };

                let (hash_kind, data) = data.split_at(1);
                let hash_kind = match hash_kind[0] {
                    1 => git_hash::Kind::Sha1,
                    // TODO: 2 = SHA256, use it once we know it
                    unknown => return Err(Error::UnsupportedHashKind { kind: unknown }),
                };
                let (num_chunks, data) = data.split_at(1);
                let num_chunks = num_chunks[0];

                let (_num_base_files, data) = data.split_at(1); // TODO: handle base files once it's clear what this does

                let (num_packs, _) = data.split_at(4);
                let num_packs = BigEndian::read_u32(num_packs);

                (version, hash_kind, num_chunks, num_packs)
            };

            let chunks = git_chunk::file::Index::from_bytes(&data, HEADER_LEN, num_chunks as u32)?;
            let pack_names = chunks.data_by_kind(&data, chunk::pack_names::ID, "PNAM")?;
            let index_names = chunk::pack_names::from_slice(pack_names, num_packs)?;

            let fan = chunks.data_by_kind(&data, chunk::fanout::ID, "OIDF")?;
            let fan = chunk::fanout::from_slice(fan).ok_or(Error::MultiPackFanSize)?;
            let num_objects = fan[255];

            let lookup = chunks.offset_by_kind(chunk::lookup::ID, "OIDL")?;
            if !chunk::lookup::is_valid(&lookup, hash_kind, num_objects) {
                return Err(Error::OidLookupSize);
            }
            let offsets = chunks.offset_by_kind(chunk::offsets::ID, "OOFF")?;
            if !chunk::offsets::is_valid(&offsets, num_objects) {
                return Err(Error::OffsetsSize);
            }
            let large_offsets = chunks.offset_by_kind(chunk::large_offsets::ID, "LOFF").ok();
            if !chunk::large_offsets::is_valid(large_offsets.as_ref()) {
                return Err(Error::LargeOffsetsSize);
            }

            let checksum_offset = chunks.highest_offset() as usize;
            let trailer = &data[checksum_offset..];
            if trailer.len() != hash_kind.len_in_bytes() {
                return Err(Error::Corrupt {
                    message: "Trailing checksum didn't have the expected size or there were unknown bytes after the checksum.",
                });
            }

            Ok(File {
                data,
                path: path.to_owned(),
                version,
                hash_kind,
                fan,
                index_names,
                lookup,
                offsets,
                large_offsets,
                checksum_offset,
                num_objects,
                num_chunks,
                num_packs,
            })
        }
    }
}
