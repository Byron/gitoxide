use std::io::Read;

use git_object::Kind;

use crate::store_impls::{compound, loose};

impl crate::traits::Write for compound::Store {
    type Error = loose::write::Error;

    fn write(&self, object: impl git_object::WriteTo) -> Result<git_hash::ObjectId, Self::Error> {
        self.loose.write(object)
    }

    fn write_buf(&self, object: Kind, from: &[u8]) -> Result<git_hash::ObjectId, Self::Error> {
        self.loose.write_buf(object, from)
    }

    fn write_stream(&self, kind: Kind, size: u64, from: impl Read) -> Result<git_hash::ObjectId, Self::Error> {
        self.loose.write_stream(kind, size, from)
    }
}
