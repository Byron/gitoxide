use crate::{hash::Write as HashWrite, loose};
use std::io;

/// Returned by [`loose::Object::verify_checksum()`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Access(#[from] loose::object::access::Error),
    #[error("Object expected to have id {desired}, but actual id was {actual}")]
    ChecksumMismatch {
        desired: git_hash::ObjectId,
        actual: git_hash::ObjectId,
    },
}

impl loose::Object {
    /// Generate the git hash of this object, reading it in the process, and compare it with the given `desired` [ObjectId][git_hash::ObjectId].
    ///
    /// Returns an error with the actual id if the hashes don't match.
    pub fn verify_checksum(&mut self, desired: impl AsRef<git_hash::oid>) -> Result<(), Error> {
        let desired = desired.as_ref();
        let mut sink = HashWrite::new(io::sink(), desired.kind());
        let mut buf = Vec::with_capacity(self.size);
        // TODO: performance note: use an inflate-read with skip-bytes support for header (similar to what the StreamReader did
        self.data(&mut buf)?;

        loose::object::header::encode(self.kind, self.size as u64, &mut sink).expect("hash to always work");
        io::copy(&mut buf.as_slice(), &mut sink).expect("copy from memory to always work");

        let actual = git_hash::ObjectId::from(sink.hash.digest());
        if desired != actual {
            return Err(Error::ChecksumMismatch {
                desired: desired.into(),
                actual,
            });
        }
        Ok(())
    }
}
