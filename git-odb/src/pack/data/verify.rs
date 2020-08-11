use crate::pack::data::File;
use git_features::progress::{self, Progress};
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
        let mut hasher = git_features::hash::Sha1::default();
        let start = std::time::Instant::now();
        progress.init(Some(self.data_len()), progress::bytes());

        let actual = match std::fs::File::open(&self.path) {
            Ok(mut pack) => {
                use std::io::Read;
                const BUF_SIZE: usize = u16::MAX as usize;
                let mut buf = [0u8; BUF_SIZE];
                let mut bytes_left = self.data.len() - SHA1_SIZE;
                while bytes_left > 0 {
                    let out = &mut buf[..BUF_SIZE.min(bytes_left)];
                    pack.read_exact(out)?;
                    bytes_left -= out.len();
                    progress.inc_by(out.len());
                    hasher.update(out);
                }
                owned::Id::new_sha1(hasher.digest())
            }
            Err(_) => {
                let right_before_trailer = self.data.len() - SHA1_SIZE;
                hasher.update(&self.data[..right_before_trailer]);
                owned::Id::new_sha1(hasher.digest())
            }
        };
        progress.show_throughput(start);

        let expected = self.checksum();
        if actual == expected {
            Ok(actual)
        } else {
            Err(Error::Mismatch { actual, expected })
        }
    }
}
