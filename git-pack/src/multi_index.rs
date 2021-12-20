#![allow(missing_docs, unused)]
use filebuffer::FileBuffer;

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
}

///
pub mod access {
    use crate::multi_index::File;

    impl File {
        pub fn num_packs(&self) -> u32 {
            self.num_packs
        }
        pub fn hash_kind(&self) -> git_hash::Kind {
            self.hash_kind
        }
    }
}

///
pub mod init {
    use crate::multi_index::{File, Version};
    use byteorder::{BigEndian, ByteOrder};
    use filebuffer::FileBuffer;
    use std::convert::TryFrom;
    use std::path::Path;

    mod error {
        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error("Could not open multi-index file at '{path}'")]
            Io {
                source: std::io::Error,
                path: std::path::PathBuf,
            },
            #[error("{message}")]
            Corrupt { message: String },
            #[error("Unsupported multi-index version: {version})")]
            UnsupportedVersion { version: u8 },
            #[error("Unsupported hash kind: {kind})")]
            UnsupportedHashKind { kind: u8 },
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

            const HEADER_LEN: usize = (4 /*signature*/ + 1 /*version*/ + 1 /*object id version*/ + 1 /* num chunks */ + 1/* num base files */ + 4/*num pack files*/);
            const TRAILER_LEN: usize = git_hash::Kind::longest().len_in_bytes(); /* trailing hash */
            if data.len() < HEADER_LEN + TRAILER_LEN {
                return Err(Error::Corrupt {
                    message: "multi-index file is truncated and too short".into(),
                });
            }

            let (version, hash_kind, num_chunks, num_packs, toc) = {
                let (signature, data) = data.split_at(4);
                if signature != b"MIDX" {
                    return Err(Error::Corrupt {
                        message: "Invalid signature".into(),
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

                let (num_packs, toc) = data.split_at(4);
                let num_packs = BigEndian::read_u32(num_packs);

                (version, hash_kind, num_chunks, num_packs, toc)
            };

            Ok(File {
                data,
                path: path.to_owned(),
                version,
                hash_kind,
                num_chunks,
                num_packs,
            })
        }
    }
}
