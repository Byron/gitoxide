use std::str;

use failure::Error;

pub const SHA1_LEN: usize = 20;

/// A SHA1 identifying objects
pub type Id = [u8; SHA1_LEN];

pub fn id_from_20_bytes(b: &[u8]) -> Id {
    let mut id = [0; SHA1_LEN];
    id.copy_from_slice(b);
    id
}

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
