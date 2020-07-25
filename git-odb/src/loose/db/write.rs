use super::Db;
use crate::{loose, zlib::stream::DeflateWriter};
use git_features::hash::Sha1;
use git_object::{owned, HashKind};
use quick_error::quick_error;
use std::{fs, io, io::Write, path::PathBuf};
use tempfile::NamedTempFile;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error, msg: &'static str, path: PathBuf) {
            display("Could not {} '{}'", msg, path.display())
            source(err)
        }
        IoRaw(err: io::Error) {
            source(err)
            from()
        }
        Persist(err: tempfile::PersistError, target: PathBuf) {
            display("Could not turn temporary file into persisted file at '{}'", target.display())
            source(err)
       }
    }
}

struct HashWrite {
    hash: Sha1,
    inner: DeflateWriter<NamedTempFile>,
}
impl io::Write for HashWrite {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.hash.update(buf);
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl crate::Write for Db {
    type Error = Error;

    fn write_buf(&self, kind: git_object::Kind, from: &[u8], hash: HashKind) -> Result<owned::Id, Self::Error> {
        match hash {
            HashKind::Sha1 => {
                let mut to = HashWrite {
                    hash: Sha1::default(),
                    inner: DeflateWriter::new(
                        NamedTempFile::new_in(&self.path)
                            .map_err(|err| Error::Io(err, "create named temp file in", self.path.to_owned()))?,
                    ),
                };

                loose::object::header::encode(kind, from.len() as u64, &mut to)
                    .map_err(|err| Error::Io(err, "write header to tempfile in", self.path.to_owned()))?;
                to.write_all(from)
                    .map_err(|err| Error::Io(err, "stream all data into tempfile in", self.path.to_owned()))?;
                to.flush()?;

                let HashWrite { hash, inner: file } = to;
                let id = owned::Id::from(hash.digest());
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
                file.persist(&object_path)
                    .map_err(|err| Error::Persist(err, object_path))?;
                Ok(id)
            }
        }
    }

    fn write_stream(
        &self,
        kind: git_object::Kind,
        size: u64,
        mut from: impl io::Read,
        hash: HashKind,
    ) -> Result<owned::Id, Self::Error> {
        match hash {
            HashKind::Sha1 => {
                let mut to = HashWrite {
                    hash: Sha1::default(),
                    inner: DeflateWriter::new(
                        NamedTempFile::new_in(&self.path)
                            .map_err(|err| Error::Io(err, "create named temp file in", self.path.to_owned()))?,
                    ),
                };

                loose::object::header::encode(kind, size, &mut to)
                    .map_err(|err| Error::Io(err, "write header to tempfile in", self.path.to_owned()))?;
                io::copy(&mut from, &mut to)
                    .map_err(|err| Error::Io(err, "stream all data into tempfile in", self.path.to_owned()))?;
                to.flush()?;

                let HashWrite { hash, inner: file } = to;
                let id = owned::Id::from(hash.digest());
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
                file.persist(&object_path)
                    .map_err(|err| Error::Persist(err, object_path))?;
                Ok(id)
            }
        }
    }
}
