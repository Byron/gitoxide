use crate::{borrowed, SHA1_SIZE};
use bstr::ByteSlice;
use std::{fmt, io, ops::Deref};

/// An owned SHA1 identifying objects
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Id([u8; SHA1_SIZE]);

impl Id {
    pub fn new_sha1(id: [u8; SHA1_SIZE]) -> Self {
        Id(id)
    }
    pub fn from_20_bytes(b: &[u8]) -> Id {
        let mut id = [0; SHA1_SIZE];
        id.copy_from_slice(b);
        Id(id)
    }
    pub fn from_borrowed_sha1(b: &[u8; SHA1_SIZE]) -> Id {
        let mut id = [0; SHA1_SIZE];
        id.copy_from_slice(&b[..]);
        Id(id)
    }
    pub fn null_sha1() -> Id {
        Id([0u8; 20])
    }

    pub fn from_40_bytes_in_hex(buf: &[u8]) -> Result<Id, hex::FromHexError> {
        use hex::FromHex;
        Ok(Id(<[u8; 20]>::from_hex(buf)?))
    }

    pub fn to_borrowed(&self) -> borrowed::Id {
        borrowed::Id::from(&self.0)
    }
    pub fn sha1(&self) -> &[u8; SHA1_SIZE] {
        &self.0
    }
    pub fn to_hex(&self) -> [u8; SHA1_SIZE * 2] {
        let mut hex_buf = [0u8; 40];
        hex::encode_to_slice(self.0, &mut hex_buf).expect("we can count");
        hex_buf
    }
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_ref()
    }
    pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
        out.write_all(&self.to_hex())
    }
}

impl Deref for Id {
    type Target = [u8; SHA1_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.to_hex().as_bstr())
    }
}
