use crate::{
    loose::{db::sha1_path, object::header, Db, Object, HEADER_READ_COMPRESSED_BYTES, HEADER_READ_UNCOMPRESSED_BYTES},
    zlib,
};
use git_object as object;
use object::borrowed;
use smallvec::SmallVec;
use std::{convert::TryInto, fs, io::Read, path::PathBuf};

/// Returned by [`Db::locate()`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("decompression of loose object at '{path}' failed")]
    DecompressFile { source: zlib::Error, path: PathBuf },
    #[error(transparent)]
    Decode(#[from] header::Error),
    #[error("Could not {action} data at '{path}'")]
    Io {
        source: std::io::Error,
        action: &'static str,
        path: PathBuf,
    },
}

/// Object lookup
impl Db {
    const OPEN_ACTION: &'static str = "open";

    /// Return the object identified by the given [`id][borrowed::Id] if present in this database.
    ///
    /// Returns `None` if the object did not exist in the database.
    pub fn locate(&self, id: borrowed::Id<'_>) -> Option<Result<Object, Error>> {
        match self.locate_inner(id) {
            Ok(obj) => Some(Ok(obj)),
            Err(err) => match err {
                Error::Io {
                    source: err,
                    action,
                    path,
                } => {
                    if action == Self::OPEN_ACTION && err.kind() == std::io::ErrorKind::NotFound {
                        None
                    } else {
                        Some(Err(Error::Io {
                            source: err,
                            action,
                            path,
                        }))
                    }
                }
                err => Some(Err(err)),
            },
        }
    }

    fn locate_inner(&self, id: borrowed::Id<'_>) -> Result<Object, Error> {
        let path = sha1_path(id, self.path.clone());

        let mut inflate = zlib::Inflate::default();
        let mut decompressed = [0; HEADER_READ_UNCOMPRESSED_BYTES];
        let mut compressed = [0; HEADER_READ_COMPRESSED_BYTES];
        let ((_status, _consumed_in, consumed_out), bytes_read, mut input_stream) = {
            let mut istream = fs::File::open(&path).map_err(|e| Error::Io {
                source: e,
                action: Self::OPEN_ACTION,
                path: path.to_owned(),
            })?;
            let bytes_read = istream.read(&mut compressed[..]).map_err(|e| Error::Io {
                source: e,
                action: "read",
                path: path.to_owned(),
            })?;
            (
                inflate
                    .once(&compressed[..bytes_read], &mut decompressed[..])
                    .map_err(|e| Error::DecompressFile {
                        source: e,
                        path: path.to_owned(),
                    })?,
                bytes_read,
                istream,
            )
        };

        let (kind, size, header_size) = header::decode(&decompressed[..consumed_out])?;
        let mut decompressed = SmallVec::from_buf(decompressed);
        decompressed.resize(consumed_out, 0);

        let (compressed, path) = if inflate.is_done {
            (SmallVec::default(), None)
        } else {
            match kind {
                object::Kind::Tree | object::Kind::Commit | object::Kind::Tag => {
                    let mut compressed = SmallVec::from_buf(compressed);
                    // Read small objects right away and store them in memory while we
                    // have a data handle available and 'hot'. Note that we don't decompress yet!
                    let file_size = input_stream
                        .metadata()
                        .map_err(|e| Error::Io {
                            source: e,
                            action: "read metadata",
                            path: path.to_owned(),
                        })?
                        .len();
                    assert!(file_size <= ::std::usize::MAX as u64);
                    let file_size = file_size as usize;
                    if bytes_read == file_size {
                        (compressed, None)
                    } else {
                        let cap = compressed.capacity();
                        if cap < file_size {
                            compressed.reserve_exact(file_size - cap);
                            debug_assert!(file_size == compressed.capacity());
                        }

                        compressed.resize(file_size, 0);
                        input_stream
                            .read_exact(&mut compressed[bytes_read..])
                            .map_err(|e| Error::Io {
                                source: e,
                                action: "read",
                                path: path.to_owned(),
                            })?;
                        (compressed, None)
                    }
                }
                object::Kind::Blob => (SmallVec::default(), Some(path)), // we will open the data again when needed. Maybe we can load small sized objects anyway
            }
        };

        Ok(Object {
            kind,
            size: size.try_into().expect("actual size to potentially fit into memory"),
            decompressed_data: decompressed,
            compressed_data: compressed,
            header_size,
            path,
            decompression_complete: inflate.is_done,
        })
    }
}
