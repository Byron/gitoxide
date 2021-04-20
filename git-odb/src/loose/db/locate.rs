use crate::{
    borrowed,
    loose::{db::sha1_path, object::header, Db, Object, HEADER_READ_COMPRESSED_BYTES, HEADER_READ_UNCOMPRESSED_BYTES},
    zlib,
};
use git_object as object;
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

    /// Return the object identified by the given [`ObjectId`][git_hash::ObjectId] if present in this database.
    ///
    /// Returns `Err` if there was an error locating or reading the object. Returns `Ok<None>` if
    /// there was no such object.
    pub fn locate(&self, id: impl AsRef<git_hash::oid>) -> Result<Option<Object>, Error> {
        match self.locate_inner(id.as_ref()) {
            Ok(obj) => Ok(Some(obj)),
            Err(err) => match err {
                Error::Io {
                    source: err,
                    action,
                    path,
                } => {
                    if action == Self::OPEN_ACTION && err.kind() == std::io::ErrorKind::NotFound {
                        Ok(None)
                    } else {
                        Err(Error::Io {
                            source: err,
                            action,
                            path,
                        })
                    }
                }
                err => Err(err),
            },
        }
    }

    #[allow(missing_docs)]
    pub fn locate2<'a>(
        &self,
        id: impl AsRef<git_hash::oid>,
        out: &'a mut Vec<u8>,
    ) -> Result<Option<borrowed::Object<'a>>, Error> {
        match self.locate_inner2(id.as_ref(), out) {
            Ok(obj) => Ok(Some(obj)),
            Err(err) => match err {
                Error::Io {
                    source: err,
                    action,
                    path,
                } => {
                    if action == Self::OPEN_ACTION && err.kind() == std::io::ErrorKind::NotFound {
                        Ok(None)
                    } else {
                        Err(Error::Io {
                            source: err,
                            action,
                            path,
                        })
                    }
                }
                err => Err(err),
            },
        }
    }

    fn locate_inner2<'a>(&self, id: &git_hash::oid, buf: &'a mut Vec<u8>) -> Result<borrowed::Object<'a>, Error> {
        let path = sha1_path(id, self.path.clone());

        let mut inflate = zlib::Inflate::default();
        let ((status, consumed_in, consumed_out), bytes_read) = {
            let mut istream = fs::File::open(&path).map_err(|e| Error::Io {
                source: e,
                action: Self::OPEN_ACTION,
                path: path.to_owned(),
            })?;

            let bytes_read = istream.read_to_end(buf).map_err(|e| Error::Io {
                source: e,
                action: "read",
                path: path.to_owned(),
            })?;
            buf.resize(bytes_read + HEADER_READ_UNCOMPRESSED_BYTES, 0);
            let (input, output) = buf.split_at_mut(bytes_read);
            (
                inflate
                    .once(&input[..bytes_read], output)
                    .map_err(|e| Error::DecompressFile {
                        source: e,
                        path: path.to_owned(),
                    })?,
                bytes_read,
            )
        };
        assert_ne!(
            status,
            flate2::Status::BufError,
            "Buffer errors might mean we encountered huge headers"
        );

        let decompressed_start = bytes_read;
        let (kind, size, header_size) = header::decode(&buf[decompressed_start..decompressed_start + consumed_out])?;
        let size: usize = size.try_into().expect("object size fits into machine architecture");

        if status == flate2::Status::StreamEnd {
            let decompressed_body_bytes_sans_header =
                decompressed_start + header_size..decompressed_start + consumed_out;
            assert_eq!(
                consumed_out,
                size + header_size,
                "At this point we have decompressed everything and given 'size' should match"
            );
            buf.copy_within(decompressed_body_bytes_sans_header, 0);
            buf.resize(size, 0);
            // TODO: assure both branches run via tests
            Ok(borrowed::Object { kind, data: buf })
        } else {
            buf.resize(bytes_read + size + header_size, 0);
            {
                let (input, output) = buf.split_at_mut(bytes_read);
                let num_decompressed_bytes = zlib::stream::inflate::read(
                    &mut std::io::Cursor::new(&mut input[consumed_in..]),
                    &mut inflate.state,
                    &mut output[consumed_out..],
                )
                .map_err(|e| Error::Io {
                    source: e,
                    action: "deflate",
                    path: path.to_owned(),
                })?;
                assert_eq!(
                    num_decompressed_bytes + consumed_out,
                    size + header_size,
                    "Object should have been decompressed entirely and match given 'size'"
                );
            };
            buf.copy_within(decompressed_start + header_size.., 0);
            buf.resize(size, 0);
            Ok(borrowed::Object { kind, data: buf })
        }
    }

    fn locate_inner(&self, id: &git_hash::oid) -> Result<Object, Error> {
        let path = sha1_path(id, self.path.clone());

        let mut inflate = zlib::Inflate::default();
        let mut decompressed = [0; HEADER_READ_UNCOMPRESSED_BYTES];
        let mut compressed = [0; HEADER_READ_COMPRESSED_BYTES];
        let ((status, _consumed_in, consumed_out), bytes_read, mut input_stream) = {
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

        let (compressed, path) = if status == flate2::Status::StreamEnd {
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
            decompression_complete: status == flate2::Status::StreamEnd,
        })
    }
}
