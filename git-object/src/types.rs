use quick_error::quick_error;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Time {
    /// time in seconds from epoch
    pub time: u32,
    /// time offset in seconds
    pub offset: i32,
    /// the sign seen in front of -0000
    pub sign: Sign,
}

pub const SHA1_SIZE: usize = 20;

/// A SHA1 identifying objects
pub type Id = [u8; SHA1_SIZE];

pub fn id_from_20_bytes(b: &[u8]) -> Id {
    let mut id = [0; SHA1_SIZE];
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
quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidObjectKind(kind: Vec<u8>) {
            display("Unknown object kind: {:?}", std::str::from_utf8(&kind))
        }
    }
}

impl Kind {
    pub fn from_bytes(s: &[u8]) -> Result<Kind, Error> {
        Ok(match s {
            b"tag" => Kind::Tag,
            b"commit" => Kind::Commit,
            b"tree" => Kind::Tree,
            b"blob" => Kind::Blob,
            _ => return Err(Error::InvalidObjectKind(s.to_owned())),
        })
    }
}
