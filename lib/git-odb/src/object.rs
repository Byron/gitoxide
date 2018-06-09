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

pub mod parsed {
    use failure::Error;
    use object::{Id, Kind};
    use std::ops::Range;

    #[derive(PartialEq, Eq, Debug, Hash)]
    pub enum Object<'data> {
        Tag(Tag<'data>),
    }

    impl<'data> Object<'data> {
        pub fn kind(&self) -> Kind {
            match self {
                Object::Tag(_) => Kind::Tag,
            }
        }
    }

    #[derive(PartialEq, Eq, Debug, Hash)]
    pub struct Tag<'data> {
        pub data: &'data [u8],
        pub target: Range<usize>,
        pub target_kind: Kind,
    }

    impl<'data> Tag<'data> {
        pub fn from_bytes(_input: &'data [u8]) -> Result<Tag<'data>, Error> {
            unimplemented!()
        }
    }
}
