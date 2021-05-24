use std::io::Read;

use git_object::{mutable, Kind};

use crate::store::compound;
use crate::store::loose;

impl crate::write::Write for compound::Backend {
    type Error = loose::backend::write::Error;

    fn write(&self, object: &mutable::Object, hash: git_hash::Kind) -> Result<git_hash::ObjectId, Self::Error> {
        self.loose.write(object, hash)
    }

    fn write_buf(&self, object: Kind, from: &[u8], hash: git_hash::Kind) -> Result<git_hash::ObjectId, Self::Error> {
        self.loose.write_buf(object, from, hash)
    }

    fn write_stream(
        &self,
        kind: Kind,
        size: u64,
        from: impl Read,
        hash: git_hash::Kind,
    ) -> Result<git_hash::ObjectId, Self::Error> {
        self.loose.write_stream(kind, size, from, hash)
    }
}
