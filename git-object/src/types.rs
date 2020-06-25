use bstr::ByteSlice;
use nom::lib::std::fmt::Formatter;
use quick_error::quick_error;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
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
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub struct Id(pub [u8; SHA1_SIZE]);

impl Id {
    pub fn encode_to_40_bytes_slice(&self, out: &mut [u8]) -> Result<(), hex::FromHexError> {
        hex::encode_to_slice(self.0, out)
    }

    pub fn from_20_bytes(b: &[u8]) -> Id {
        let mut id = [0; SHA1_SIZE];
        id.copy_from_slice(b);
        Id(id)
    }

    pub fn from_hex(buf: &[u8]) -> Result<Id, hex::FromHexError> {
        use hex::FromHex;
        Ok(Id(<[u8; 20]>::from_hex(buf)?))
    }

    pub fn null() -> Id {
        Id([0u8; 20])
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buf = [0u8; 40];
        self.encode_to_40_bytes_slice(&mut buf).unwrap();
        write!(f, "{}", &buf.as_bstr())
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub enum Kind {
    Tag,
    Commit,
    Tree,
    Blob,
}
quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidObjectKind(kind: bstr::BString) {
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
            _ => return Err(Error::InvalidObjectKind(s.into())),
        })
    }
}
