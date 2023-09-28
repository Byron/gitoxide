use std::sync::atomic::AtomicBool;

use crate::File;

mod error {
    /// The error returned by [File::verify_integrity()][super::File::verify_integrity()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Could not read index file to generate hash")]
        Io(#[from] std::io::Error),
        #[error("Index checksum should have been {expected}, but was {actual}")]
        ChecksumMismatch {
            actual: gix_hash::ObjectId,
            expected: gix_hash::ObjectId,
        },
    }
}
pub use error::Error;

impl File {
    /// Verify the integrity of the index to assure its consistency.
    pub fn verify_integrity(&self) -> Result<(), Error> {
        let _span = gix_features::trace::coarse!("gix_index::File::verify_integrity()");
        if let Some(checksum) = self.checksum {
            let num_bytes_to_hash = self.path.metadata()?.len() - checksum.as_bytes().len() as u64;
            let should_interrupt = AtomicBool::new(false);
            let actual = gix_features::hash::bytes_of_file(
                &self.path,
                num_bytes_to_hash,
                checksum.kind(),
                &mut gix_features::progress::Discard,
                &should_interrupt,
            )?;
            (actual == checksum).then_some(()).ok_or(Error::ChecksumMismatch {
                actual,
                expected: checksum,
            })
        } else {
            Ok(())
        }
    }
}
