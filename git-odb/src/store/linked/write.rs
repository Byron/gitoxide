use std::io::Read;

use git_object::{Kind, Object};

use crate::store::{linked, loose};

impl crate::write::Write for linked::Store {
    type Error = loose::write::Error;

    fn write(&self, object: &Object, hash: git_hash::Kind) -> Result<git_hash::ObjectId, Self::Error> {
        self.dbs[0].loose.write(object, hash)
    }

    fn write_buf(&self, object: Kind, from: &[u8], hash: git_hash::Kind) -> Result<git_hash::ObjectId, Self::Error> {
        self.dbs[0].loose.write_buf(object, from, hash)
    }

    fn write_stream(
        &self,
        kind: Kind,
        size: u64,
        from: impl Read,
        hash: git_hash::Kind,
    ) -> Result<git_hash::ObjectId, Self::Error> {
        self.dbs[0].loose.write_stream(kind, size, from, hash)
    }
}
