use std::str;

use failure::Error;

pub type Id = [u8; 20];

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd)]
pub enum Kind {
    Tag,
    Commit,
}

impl Kind {
    pub fn from_bytes(s: &[u8]) -> Result<Kind, Error> {
        Ok(match s {
            b"tag" => Kind::Tag,
            b"commit" => Kind::Commit,
            _ => bail!("Unknown object kind: {:?}", str::from_utf8(s)),
        })
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Tag {
    pub target: Id,
    pub target_kind: Kind,
}

impl Tag {
    pub fn from_bytes(input: &[u8]) -> Result<Self, Error> {
        unimplemented!()
    }
}
