use super::Db;
use crate::{hash, loose, zlib::stream::DeflateWriter};
use std::{fs, io, io::Write, path::PathBuf};
use tempfile::NamedTempFile;

/// Returned by the [`crate::Write`] trait implementation of [`Db`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Could not {message} '{path}'")]
    Io {
        source: io::Error,
        message: &'static str,
        path: PathBuf,
    },
    #[error("An IO error occurred while writing an object")]
    IoRaw(#[from] io::Error),
    #[error("Could not turn temporary file into persisted file at '{target}'")]
    Persist {
        source: tempfile::PersistError,
        target: PathBuf,
    },
}

impl crate::Write for Db {
    type Error = Error;

    /// Write the given buffer in `from` to disk in one syscall at best.
    ///
    /// This will cost at least 4 IO operations.
    fn write_buf(
        &self,
        kind: git_object::Kind,
        from: &[u8],
        hash: git_hash::Kind,
    ) -> Result<git_hash::ObjectId, Self::Error> {
        match hash {
            git_hash::Kind::Sha1 => {
                let mut to = self.write_header(kind, from.len() as u64, hash)?;
                to.write_all(from).map_err(|err| Error::Io {
                    source: err,
                    message: "stream all data into tempfile in",
                    path: self.path.to_owned(),
                })?;
                to.flush()?;
                self.finalize_object(to)
            }
        }
    }

    /// Write the given stream in `from` to disk with at least one syscall.
    ///
    /// This will cost at least 4 IO operations.
    fn write_stream(
        &self,
        kind: git_object::Kind,
        size: u64,
        mut from: impl io::Read,
        hash: git_hash::Kind,
    ) -> Result<git_hash::ObjectId, Self::Error> {
        match hash {
            git_hash::Kind::Sha1 => {
                let mut to = self.write_header(kind, size, hash)?;
                io::copy(&mut from, &mut to).map_err(|err| Error::Io {
                    source: err,
                    message: "stream all data into tempfile in",
                    path: self.path.to_owned(),
                })?;
                to.flush()?;
                self.finalize_object(to)
            }
        }
    }
}

type HashAndTempFile = DeflateWriter<NamedTempFile>;

impl Db {
    fn write_header(
        &self,
        kind: git_object::Kind,
        size: u64,
        hash: git_hash::Kind,
    ) -> Result<hash::Write<HashAndTempFile>, Error> {
        let mut to = hash::Write::new(
            DeflateWriter::new(NamedTempFile::new_in(&self.path).map_err(|err| Error::Io {
                source: err,
                message: "create named temp file in",
                path: self.path.to_owned(),
            })?),
            hash,
        );

        loose::object::header::encode(kind, size, &mut to).map_err(|err| Error::Io {
            source: err,
            message: "write header to tempfile in",
            path: self.path.to_owned(),
        })?;
        Ok(to)
    }

    fn finalize_object(
        &self,
        hash::Write { hash, inner: file }: hash::Write<HashAndTempFile>,
    ) -> Result<git_hash::ObjectId, Error> {
        let id = git_hash::ObjectId::from(hash.digest());
        let object_path = loose::db::sha1_path(id.to_borrowed(), self.path.clone());
        let object_dir = object_path
            .parent()
            .expect("each object path has a 1 hex-bytes directory");
        if let Err(err) = fs::create_dir(object_dir) {
            match err.kind() {
                io::ErrorKind::AlreadyExists => {}
                _ => return Err(err.into()),
            }
        }
        let file = file.into_inner();
        file.persist(&object_path).map_err(|err| Error::Persist {
            source: err,
            target: object_path,
        })?;
        Ok(id)
    }
}
