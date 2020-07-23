use git_object::{owned, HashKind};
use std::io;

pub trait Write {
    type Error: std::error::Error + From<io::Error>;

    fn write(&self, object: &owned::Object, hash: HashKind) -> Result<owned::Id, Self::Error> {
        let mut buf = Vec::with_capacity(2048);
        object.write_to(&mut buf)?;
        self.write_stream(object.kind(), buf.len() as u64, buf.as_slice(), hash)
    }
    fn write_buf(&self, object: git_object::Kind, from: &[u8], hash: HashKind) -> Result<owned::Id, Self::Error> {
        self.write_stream(object, from.len() as u64, from, hash)
    }
    fn write_stream(
        &self,
        kind: git_object::Kind,
        size: u64,
        from: impl io::Read,
        hash: HashKind,
    ) -> Result<owned::Id, Self::Error>;
}
