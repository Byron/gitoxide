use crate::owned::SPACE;
use bstr::ByteSlice;
use quick_error::quick_error;
use std::{fmt, io, ops::Deref};

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Time {
    /// time in seconds from epoch
    pub time: u32,
    /// time offset in seconds, may be negative to match the sign
    pub offset: i32,
    /// the sign seen in front of -0000
    pub sign: Sign,
}

impl Time {
    pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
        itoa::write(&mut out, self.time)?;
        out.write_all(SPACE)?;
        out.write_all(&[match self.sign {
            Sign::Plus => b'+',
            Sign::Minus => b'-',
        }])?;

        const ZERO: &[u8; 1] = b"0";

        const SECONDS_PER_HOUR: i32 = 60 * 60;
        let offset = self.offset.abs();
        let hours = offset / SECONDS_PER_HOUR;
        assert!(hours < 25, "offset is more than a day: {}", hours);
        let minutes = (offset - (hours * SECONDS_PER_HOUR)) / 60;

        if hours < 10 {
            out.write_all(ZERO)?;
        }
        itoa::write(&mut out, hours)?;

        if minutes < 10 {
            out.write_all(ZERO)?;
        }
        itoa::write(&mut out, minutes).map(|_| ())
    }
}

pub const SHA1_SIZE: usize = 20;

/// A SHA1 identifying objects
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
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

    pub fn from_40_bytes_in_hex(buf: &[u8]) -> Result<Id, hex::FromHexError> {
        use hex::FromHex;
        Ok(Id(<[u8; 20]>::from_hex(buf)?))
    }

    pub fn null() -> Id {
        Id([0u8; 20])
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Deref for Id {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0[..]
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = [0u8; 40];
        self.encode_to_40_bytes_slice(&mut buf).unwrap();
        write!(f, "{}", &buf.as_bstr())
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Kind {
    Tree,
    Blob,
    Commit,
    Tag,
}
quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidObjectKind(kind: crate::BString) {
            display("Unknown object kind: {:?}", std::str::from_utf8(&kind))
        }
    }
}

impl Kind {
    pub fn from_bytes(s: &[u8]) -> Result<Kind, Error> {
        Ok(match s {
            b"tree" => Kind::Tree,
            b"blob" => Kind::Blob,
            b"commit" => Kind::Commit,
            b"tag" => Kind::Tag,
            _ => return Err(Error::InvalidObjectKind(s.into())),
        })
    }

    pub fn to_bytes(&self) -> &[u8] {
        match self {
            Kind::Tree => b"tree",
            Kind::Commit => b"commit",
            Kind::Blob => b"blob",
            Kind::Tag => b"tag",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(std::str::from_utf8(self.to_bytes()).expect("valid utf8 in kind name"))
    }
}
