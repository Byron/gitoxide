use crate::loose;
use git_features::hash;
use git_object::{borrowed, owned, HashKind};
use quick_error::quick_error;
use std::io;

pub(crate) struct HashWrite<T> {
    pub hash: hash::Sha1,
    pub inner: T,
}

impl<T> io::Write for HashWrite<T>
where
    T: io::Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.hash.update(buf);
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl<T> HashWrite<T>
where
    T: io::Write,
{
    pub fn new(inner: T, kind: HashKind) -> Self {
        match kind {
            HashKind::Sha1 => HashWrite {
                inner,
                hash: hash::Sha1::default(),
            },
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            display("reading of object failed")
            from()
            source(err)
        }
        Decode(err: super::decode::Error) {
            display("Decoding of object failed")
            from()
            source(err)
        }
        ChecksumMismatch(desired: owned::Id, actual: owned::Id) {
            display("Object expected to have id {}, but actual id was {}", desired, actual)
        }
    }
}

impl loose::Object {
    pub fn verify_checksum(&self, desired: borrowed::Id) -> Result<(), Error> {
        let mut sink = HashWrite::new(io::sink(), desired.kind());
        let mut reader = self.stream()?;

        loose::object::header::encode(self.kind, self.size as u64, &mut sink).expect("hash to always work");
        io::copy(&mut reader, &mut sink)?;

        let actual_id = owned::Id::from(sink.hash.digest());
        if desired != actual_id.to_borrowed() {
            return Err(Error::ChecksumMismatch(desired.into(), actual_id));
        }
        Ok(())
    }
}
