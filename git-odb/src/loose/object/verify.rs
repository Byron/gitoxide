use crate::{hash::Write as HashWrite, loose};
use std::io;

/// Returned by [`loose::Object::verify_checksum()`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("reading of object failed")]
    Io(#[from] io::Error),
    #[error("Decoding of object failed")]
    Decode(#[from] super::decode::Error),
    #[error("Object expected to have id {desired}, but actual id was {actual}")]
    ChecksumMismatch {
        desired: git_hash::ObjectId,
        actual: git_hash::ObjectId,
    },
}

impl loose::Object {
    /// Generate the git hash of this object, reading it in the process, and compare it with the given `desired` [Id][git_hash::borrowed::Id].
    ///
    /// Returns an error with the actual id if the hashes don't match.
    pub fn verify_checksum(&mut self, desired: git_hash::borrowed::Id<'_>) -> Result<(), Error> {
        let mut sink = HashWrite::new(io::sink(), desired.kind());
        let (kind, size) = (self.kind, self.size);
        let mut reader = self.stream()?;

        loose::object::header::encode(kind, size as u64, &mut sink).expect("hash to always work");
        io::copy(&mut reader, &mut sink)?;

        let actual = git_hash::ObjectId::from(sink.hash.digest());
        if desired != actual.to_borrowed() {
            return Err(Error::ChecksumMismatch {
                desired: desired.into(),
                actual,
            });
        }
        Ok(())
    }
}
