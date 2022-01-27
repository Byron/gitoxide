use crate::File;
use std::sync::atomic::AtomicBool;

mod error {
    use quick_error::quick_error;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Io(err: std::io::Error) {
                display("Could not read index file to generate hash")
                source(err)
                from()
            }
            ChecksumMismatch { actual: git_hash::ObjectId, expected: git_hash::ObjectId }{
                display("Index checksum should have been {}, but was {}", expected, actual)
            }
        }
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
