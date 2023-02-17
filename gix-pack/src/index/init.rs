use std::{mem::size_of, path::Path};

use crate::index::{self, Version, FAN_LEN, V2_SIGNATURE};

/// Returned by [`index::File::at()`].
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

/// Instantiation
impl index::File {
    /// Open the pack index file at the given `path`.
    ///
    /// The `object_hash` is a way to read (and write) the same file format with different hashes, as the hash kind
    /// isn't stored within the file format itself.
    pub fn at(path: impl AsRef<Path>, object_hash: gix_hash::Kind) -> Result<index::File, Error> {
        Self::at_inner(path.as_ref(), object_hash)
    }

    fn at_inner(path: &Path, object_hash: gix_hash::Kind) -> Result<index::File, Error> {
        let data = crate::mmap::read_only(path).map_err(|source| Error::Io {
            source,
            path: path.to_owned(),
        })?;
        let idx_len = data.len();
        let hash_len = object_hash.len_in_bytes();

        let footer_size = hash_len * 2;
        if idx_len < FAN_LEN * N32_SIZE + footer_size {
            return Err(Error::Corrupt {
                message: format!("Pack index of size {idx_len} is too small for even an empty index"),
            });
        }
        let (kind, fan, num_objects) = {
            let (kind, d) = {
                let (sig, d) = data.split_at(V2_SIGNATURE.len());
                if sig == V2_SIGNATURE {
                    (Version::V2, d)
                } else {
                    (Version::V1, &data[..])
                }
            };
            let d = {
                if let Version::V2 = kind {
                    let (vd, dr) = d.split_at(N32_SIZE);
                    let version = crate::read_u32(vd);
                    if version != Version::V2 as u32 {
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
            version: kind,
            num_objects,
            fan,
            hash_len,
            object_hash,
        })
    }
}

fn read_fan(d: &[u8]) -> ([u32; FAN_LEN], usize) {
    let mut fan = [0; FAN_LEN];
    for (c, f) in d.chunks(N32_SIZE).zip(fan.iter_mut()) {
        *f = crate::read_u32(c);
    }
    (fan, FAN_LEN * N32_SIZE)
}
