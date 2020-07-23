use crate::loose;
use git_object::{owned::Id, HashKind};
use std::{convert::TryInto, io};

pub struct Sink {
    _priv: (),
}

pub fn sink() -> Sink {
    Sink { _priv: () }
}

impl crate::Write for Sink {
    type Error = io::Error;

    fn write_stream(
        &self,
        kind: git_object::Kind,
        size: u64,
        mut from: impl io::Read,
        hash: HashKind,
    ) -> Result<Id, Self::Error> {
        use git_features::hash::Sha1;
        let mut buf = [0u8; 8096];

        match hash {
            HashKind::Sha1 => {
                let mut hasher = Sha1::default();
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
}
