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
        PackNames(#[from] chunk::index_names::from_slice::Error),
        #[error("multi-index chunk {:?} has invalid size: {message}", String::from_utf8_lossy(.id))]
        InvalidChunkSize { id: git_chunk::Id, message: &'static str },
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

        let index_names = chunks.data_by_id(&data, chunk::index_names::ID)?;
        let index_names = chunk::index_names::from_slice(index_names, num_packs)?;

        let fan = chunks.data_by_id(&data, chunk::fanout::ID)?;
        let fan = chunk::fanout::from_slice(fan).ok_or(Error::MultiPackFanSize)?;
        let num_objects = fan[255];

        let lookup = chunks.usize_offset_by_id(chunk::lookup::ID)?;
        if !chunk::lookup::is_valid(&lookup, hash_kind, num_objects) {
            return Err(Error::InvalidChunkSize {
                id: chunk::lookup::ID,
                message: "The chunk with alphabetically ordered object ids doesn't have the correct size",
            });
        }
        let offsets = chunks.usize_offset_by_id(chunk::offsets::ID)?;
        if !chunk::offsets::is_valid(&offsets, num_objects) {
            return Err(Error::InvalidChunkSize {
                id: chunk::offsets::ID,
                message: "The chunk with offsets into the pack doesn't have the correct size",
            });
        }
        let large_offsets = chunks.usize_offset_by_id(chunk::large_offsets::ID).ok();
        if !chunk::large_offsets::is_valid(large_offsets.as_ref()) {
            return Err(Error::InvalidChunkSize {
                id: chunk::large_offsets::ID,
                message: "The chunk with large offsets into the pack doesn't have the correct size",
            });
        }

        let checksum_offset = chunks.highest_offset() as usize;
        let trailer = &data[checksum_offset..];
        if trailer.len() != hash_kind.len_in_bytes() {
            return Err(Error::Corrupt {
                message:
                    "Trailing checksum didn't have the expected size or there were unknown bytes after the checksum.",
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
