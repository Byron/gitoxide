use super::Db;
use crate::{loose, zlib::stream::DeflateWriter};
use git_features::hash::Sha1;
use git_object::{owned, HashKind};
use quick_error::quick_error;
use std::{fs, io, io::Write};
use tempfile::NamedTempFile;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            source(err)
            from()
        }
        PersistError(err: tempfile::PersistError) {
            source(err)
            from()
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
                    inner: DeflateWriter::new(NamedTempFile::new_in(&self.path)?),
                };

                loose::object::header::encode(kind, size as usize, &mut to)?;
                io::copy(&mut from, &mut to)?;
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
                file.into_inner().persist(object_path)?;
                Ok(id)
            }
        }
    }
}
