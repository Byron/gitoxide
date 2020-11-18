use crate::pack::data::File;
use git_features::progress::Progress;
use git_object::{owned, HashKind, SHA1_SIZE};

/// Returned by [`File::verify_checksum()`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("pack checksum mismatch: expected {expected}, got {actual}")]
    Mismatch { expected: owned::Id, actual: owned::Id },
    #[error("could not read pack file")]
    Io(#[from] std::io::Error),
}

/// Checksums and verify checksums
impl File {
    /// The checksum in the trailer of this pack data file
    pub fn checksum(&self) -> owned::Id {
        owned::Id::from_20_bytes(&self.data[self.data.len() - SHA1_SIZE..])
    }

    /// Verifies that the checksum of the packfile over all bytes preceding it indeed matches the actual checksum,
    /// returning the actual checksum equivalent to the return value of [`checksum()`][File::checksum()] if there
    /// is no mismatch.
    ///
    /// Note that if no `progress` is desired, one can pass [`git_features::progress::Discard`].
    ///
    /// Have a look at [`index::File::verify_integrity(…)`][crate::pack::index::File::verify_integrity()] for an
    /// even more thorough integrity check.
    pub fn verify_checksum(&self, mut progress: impl Progress) -> Result<owned::Id, Error> {
        let right_before_trailer = self.data.len() - SHA1_SIZE;
        let actual =
            match git_features::hash::bytes_of_file(&self.path, right_before_trailer, HashKind::Sha1, &mut progress) {
                Ok(id) => id,
                Err(_io_err) => {
                    let start = std::time::Instant::now();
                    let mut hasher = git_features::hash::Sha1::default();
                    hasher.update(&self.data[..right_before_trailer]);
                    progress.inc_by(right_before_trailer);
                    progress.show_throughput(start);
                    owned::Id::new_sha1(hasher.digest())
                }
            };

        let expected = self.checksum();
        if actual == expected {
            Ok(actual)
        } else {
            Err(Error::Mismatch { actual, expected })
        }
    }
}
