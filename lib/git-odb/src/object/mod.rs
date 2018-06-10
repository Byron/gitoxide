use std::str;

use failure::Error;

pub type Id = [u8; 20];

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd)]
pub enum Kind {
    Tag,
    Commit,
    Tree,
    Blob,
}

impl Kind {
    pub fn from_bytes(s: &[u8]) -> Result<Kind, Error> {
        Ok(match s {
            b"tag" => Kind::Tag,
            b"commit" => Kind::Commit,
            b"tree" => Kind::Tree,
            b"blob" => Kind::Blob,
            _ => bail!("Unknown object kind: {:?}", str::from_utf8(s)),
        })
    }
}

pub mod parsed;
