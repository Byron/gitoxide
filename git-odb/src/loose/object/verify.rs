use crate::{hash::Write as HashWrite, loose};
use git_object::{borrowed, owned};
use quick_error::quick_error;
use std::io;

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
    pub fn verify_checksum(&mut self, desired: borrowed::Id<'_>) -> Result<(), Error> {
        let mut sink = HashWrite::new(io::sink(), desired.kind());
        let (kind, size) = (self.kind, self.size);
        let mut reader = self.stream()?;

        loose::object::header::encode(kind, size as u64, &mut sink).expect("hash to always work");
        io::copy(&mut reader, &mut sink)?;

        let actual_id = owned::Id::from(sink.hash.digest());
        if desired != actual_id.to_borrowed() {
            return Err(Error::ChecksumMismatch(desired.into(), actual_id));
        }
        Ok(())
    }
}
