#![forbid(unsafe_code)]

mod zlib;

pub mod loose;
pub mod pack;

mod sink {
    use git_object::{owned::Id, Kind};
    use std::io;

    pub struct Sink;

    impl crate::Write for Sink {
        type Error = io::Error;

        fn write_stream(&self, kind: Kind, size: u64, from: impl io::Read) -> Result<Id, Self::Error> {
            use git_features::hash::Sha1;
            let hasher = Sha1::default();
            unimplemented!()
        }
    }
}

mod traits {
    use git_object::{owned, Kind};
    use std::io;

    pub trait Write {
        type Error: std::error::Error + From<io::Error>;

        fn write(&self, object: &owned::Object) -> Result<owned::Id, Self::Error> {
            let mut buf = Vec::with_capacity(2048);
            object.write_to(&mut buf)?;
            self.write_stream(object.kind(), buf.len() as u64, buf.as_slice())
        }
        fn write_buf(&self, kind: Kind, from: &[u8]) -> Result<owned::Id, Self::Error> {
            self.write_stream(kind, from.len() as u64, from)
        }
        fn write_stream(&self, kind: Kind, size: u64, from: impl io::Read) -> Result<owned::Id, Self::Error>;
    }
}

pub use traits::*;
