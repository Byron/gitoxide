use crate::{hash::Write as HashWrite, loose};
use git_object::{borrowed, owned};
use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("reading of object failed")]
    Io(#[from] io::Error),
    #[error("Decoding of object failed")]
    Decode(#[from] super::decode::Error),
    #[error("Object expected to have id {desired}, but actual id was {actual}")]
    ChecksumMismatch { desired: owned::Id, actual: owned::Id },
}

impl loose::Object {
    pub fn verify_checksum(&mut self, desired: borrowed::Id<'_>) -> Result<(), Error> {
        let mut sink = HashWrite::new(io::sink(), desired.kind());
        let (kind, size) = (self.kind, self.size);
        let mut reader = self.stream()?;

        loose::object::header::encode(kind, size as u64, &mut sink).expect("hash to always work");
        io::copy(&mut reader, &mut sink)?;

        let actual = owned::Id::from(sink.hash.digest());
        if desired != actual.to_borrowed() {
            return Err(Error::ChecksumMismatch {
                desired: desired.into(),
                actual,
            });
        }
        Ok(())
    }
}
