use std::sync::atomic::AtomicBool;

use crate::File;

mod error {
    /// The error returned by [File::verify_integrity()][super::File::verify_integrity()].
    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("Could not read index file to generate hash")]
        Io(#[from] std::io::Error),
        #[error("Index checksum should have been {expected}, but was {actual}")]
        ChecksumMismatch {
            actual: git_hash::ObjectId,
            expected: git_hash::ObjectId,
        },
    }
}
pub use error::Error;

impl File {
    pub fn verify_integrity(&self) -> Result<(), Error> {
        let num_bytes_to_hash = self.path.metadata()?.len() - self.checksum.as_bytes().len() as u64;
        let should_interrupt = AtomicBool::new(false);
        let actual = git_features::hash::bytes_of_file(
            &self.path,
            num_bytes_to_hash as usize,
            self.checksum.kind(),
            &mut git_features::progress::Discard,
            &should_interrupt,
        )?;
        (actual == self.checksum).then(|| ()).ok_or(Error::ChecksumMismatch {
            actual,
            expected: self.checksum,
        })
    }
}
