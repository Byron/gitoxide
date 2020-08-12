use crate::pack::data::File;
use git_features::progress::Progress;
use git_object::{owned, SHA1_SIZE};
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Mismatch { expected: owned::Id, actual: owned::Id } {
            display("pack checksum mismatch: expected {}, got {}", expected, actual)
        }
        Io(err: std::io::Error) {
            display("could not read pack file")
            from()
            source(err)
        }
    }
}

/// Checksums and verify checksums
impl File {
    pub fn checksum(&self) -> owned::Id {
        owned::Id::from_20_bytes(&self.data[self.data.len() - SHA1_SIZE..])
    }
    pub fn verify_checksum(&self, mut progress: impl Progress) -> Result<owned::Id, Error> {
        let right_before_trailer = self.data.len() - SHA1_SIZE;
        let actual = match crate::hash::bytes_of_file(&self.path, right_before_trailer, &mut progress) {
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
