use crate::{borrowed, SHA1_SIZE};
use bstr::ByteSlice;
use std::{fmt, io, ops::Deref};

/// An owned SHA1 identifying objects
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Id([u8; SHA1_SIZE]);

impl Id {
    pub fn kind(&self) -> crate::HashKind {
        crate::HashKind::Sha1
    }
    pub fn to_borrowed(&self) -> borrowed::Id {
        borrowed::Id::from(&self.0)
    }
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_ref()
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
    pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
        out.write_all(&self.to_sha1_hex())
    }
}

/// Sha1 hash specific methods
impl Id {
    pub fn from_40_bytes_in_hex(buf: &[u8]) -> Result<Id, hex::FromHexError> {
        use hex::FromHex;
        Ok(Id(<[u8; 20]>::from_hex(buf)?))
    }
    pub fn sha1(&self) -> &[u8; SHA1_SIZE] {
        &self.0
    }
    pub fn to_sha1_hex(&self) -> [u8; SHA1_SIZE * 2] {
        let mut hex_buf = [0u8; 40];
        hex::encode_to_slice(self.0, &mut hex_buf).expect("we can count");
        hex_buf
    }
    pub fn to_sha1_hex_string(&self) -> String {
        let buf = self.to_sha1_hex();
        std::str::from_utf8(&buf).expect("hex is valid UTF-8").to_string()
    }
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
}

impl From<[u8; SHA1_SIZE]> for Id {
    fn from(v: [u8; 20]) -> Self {
        Self::new_sha1(v)
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
        write!(f, "{}", &self.to_sha1_hex().as_bstr())
    }
}
