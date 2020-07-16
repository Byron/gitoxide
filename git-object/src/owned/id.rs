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
    pub fn borrowed(&self) -> borrowed::Id {
        borrowed::Id::from(&self.0)
    }

    pub fn from_40_bytes_in_hex(buf: &[u8]) -> Result<Id, hex::FromHexError> {
        use hex::FromHex;
        Ok(Id(<[u8; 20]>::from_hex(buf)?))
    }

    pub fn sha1(&self) -> &[u8; SHA1_SIZE] {
        &self.0
    }
    pub fn encode_to_40_bytes_slice(&self, out: &mut [u8]) -> Result<(), hex::FromHexError> {
        hex::encode_to_slice(self.0, out)
    }

    pub fn null_sha1() -> Id {
        Id([0u8; 20])
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_ref()
    }

    pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
        let mut hex_buf: [u8; 40] = [0; 40];
        self.encode_to_40_bytes_slice(&mut hex_buf[..])
            .expect("20 to 40 bytes hex encoding to always work");
        out.write_all(&hex_buf)
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
        let mut buf = [0u8; 40];
        self.encode_to_40_bytes_slice(&mut buf).unwrap();
        write!(f, "{}", &buf.as_bstr())
    }
}
