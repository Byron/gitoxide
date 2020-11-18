use crate::pack::index::{self, Kind, FAN_LEN, V2_SIGNATURE};
use byteorder::{BigEndian, ByteOrder};
use filebuffer::FileBuffer;
use git_object::SHA1_SIZE;
use std::{convert::TryFrom, mem::size_of, path::Path};

/// Returned by [`index::File::at()`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Could not open pack index file at '{path}'")]
    Io {
        source: std::io::Error,
        path: std::path::PathBuf,
    },
    #[error("{message}")]
    Corrupt { message: String },
    #[error("Unsupported index version: {version})")]
    UnsupportedVersion { version: u32 },
}

const N32_SIZE: usize = size_of::<u32>();
const FOOTER_SIZE: usize = SHA1_SIZE * 2;

/// Instantiation
impl index::File {
    /// Open the pack index file at the given `path`.
    pub fn at(path: impl AsRef<Path>) -> Result<index::File, Error> {
        Self::try_from(path.as_ref())
    }
}

impl TryFrom<&Path> for index::File {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let data = FileBuffer::open(&path).map_err(|e| Error::Io {
            source: e,
            path: path.to_owned(),
        })?;
        let idx_len = data.len();
        if idx_len < FAN_LEN * N32_SIZE + FOOTER_SIZE {
            return Err(Error::Corrupt {
                message: format!("Pack index of size {} is too small for even an empty index", idx_len),
            });
        }
        let (kind, fan, num_objects) = {
            let (kind, d) = {
                let (sig, d) = data.split_at(V2_SIGNATURE.len());
                if sig == V2_SIGNATURE {
                    (Kind::V2, d)
                } else {
                    (Kind::V1, &data[..])
                }
            };
            let d = {
                if let Kind::V2 = kind {
                    let (vd, dr) = d.split_at(N32_SIZE);
                    let version = BigEndian::read_u32(vd);
                    if version != Kind::V2 as u32 {
                        return Err(Error::UnsupportedVersion { version });
                    }
                    dr
                } else {
                    d
                }
            };
            let (fan, bytes_read) = read_fan(d);
            let (_, _d) = d.split_at(bytes_read);
            let num_objects = fan[FAN_LEN - 1];

            (kind, fan, num_objects)
        };
        Ok(index::File {
            data,
            path: path.to_owned(),
            kind,
            num_objects,
            fan,
        })
    }
}

fn read_fan(d: &[u8]) -> ([u32; FAN_LEN], usize) {
    let mut fan = [0; FAN_LEN];
    for (c, f) in d.chunks(N32_SIZE).zip(fan.iter_mut()) {
        *f = BigEndian::read_u32(c);
    }
    (fan, FAN_LEN * N32_SIZE)
}
