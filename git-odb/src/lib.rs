#![forbid(unsafe_code)]

mod zlib;

pub mod loose;
pub mod pack;

mod sink {
    use crate::loose;
    use git_object::{owned::Id, Kind};
    use std::{convert::TryInto, io};

    pub struct Sink {
        _priv: (),
    }

    pub fn sink() -> Sink {
        Sink { _priv: () }
    }

    impl crate::Write for Sink {
        type Error = io::Error;

        fn write_stream(&self, kind: Kind, size: u64, mut from: impl io::Read) -> Result<Id, Self::Error> {
            use git_features::hash::Sha1;
            let mut hasher = Sha1::default();

            let mut buf = [0u8; 8096];
            let header_len = loose::object::header::encode(kind, size as usize, &mut buf[..])?;
            hasher.update(&buf[..header_len]);

            let mut size: usize = size.try_into().unwrap();
            while size != 0 {
                let bytes = size.min(buf.len());
                from.read_exact(&mut buf[..bytes])?;
                hasher.update(&buf[..bytes]);
                size -= bytes;
            }

            Ok(hasher.digest().into())
        }
    }
}

pub use sink::{sink, Sink};

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
