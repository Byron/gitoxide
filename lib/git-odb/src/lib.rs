#[macro_use]
extern crate failure;
extern crate hex;
extern crate miniz_oxide;
extern crate walkdir;

use failure::Error;

pub type ObjectId = [u8; 20];
#[derive(PartialEq, Eq, Debug)]
pub enum ObjectKind {
    Tag,
}

impl ObjectKind {
    fn from_bytes(s: &[u8]) -> Result<ObjectKind, Error> {
        Ok(match s {
            b"tag" => ObjectKind::Tag,
            _ => bail!("Unknown object kind: {:?}", std::str::from_utf8(s)),
        })
    }
}

pub mod loose;
