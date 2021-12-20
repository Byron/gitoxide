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
}

///
pub mod init {
    use crate::multi_index::File;
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
            let data = FileBuffer::open(&path).map_err(|source| Error::Io {
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

            let hash_kind = {
                let (signature, data) = data.split_at(4);
                if signature != b"MIDX" {
                    return Err(Error::Corrupt {
                        message: "Invalid signature".into(),
                    });
                }
                let (version, data) = data.split_at(1);
                if version[0] != 1 {
                    return Err(Error::UnsupportedVersion { version: version[0] });
                }

                let (hash_kind, data) = data.split_at(1);
                match hash_kind[0] {
                    1 => git_hash::Kind::Sha1,
                    // TODO: 2 = SHA256, use it once we know it
                    unknown => return Err(Error::UnsupportedHashKind { kind: unknown }),
                }
                hash_kind
            };

            todo!("read everything")
        }
    }
}
