use std::io::Read;

use git_object::Kind;

use crate::store_impls::{linked, loose};

impl crate::traits::Write for linked::Store {
    type Error = loose::write::Error;

    fn write(&self, object: impl git_object::WriteTo) -> Result<git_hash::ObjectId, Self::Error> {
        self.dbs[0].loose.write(object)
    }

    fn write_buf(&self, object: Kind, from: &[u8]) -> Result<git_hash::ObjectId, Self::Error> {
        self.dbs[0].loose.write_buf(object, from)
    }

    fn write_stream(&self, kind: Kind, size: u64, from: impl Read) -> Result<git_hash::ObjectId, Self::Error> {
        self.dbs[0].loose.write_stream(kind, size, from)
    }
}
