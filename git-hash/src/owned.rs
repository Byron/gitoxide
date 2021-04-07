use crate::{borrowed, SIZE_OF_SHA1_DIGEST};
use bstr::ByteSlice;
use std::{fmt, io, ops::Deref};

/// An owned hash identifying objects, most commonly Sha1
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Id(pub(crate) [u8; SIZE_OF_SHA1_DIGEST]);

/// Access and conversion
impl Id {
    /// Returns the kind of hash used in this `Id`
    pub fn kind(&self) -> crate::Kind {
        crate::Kind::Sha1
    }
    /// Return a borrowed version of this instance
    pub fn to_borrowed(&self) -> borrowed::Id<'_> {
        borrowed::Id::from(&self.0)
    }
    /// Return the raw byte slice representing this hash
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_ref()
    }
    /// Return the raw mutable byte slice representing this hash
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }

    /// Write ourselves to `out` in hexadecimal notation
    pub fn write_hex_to(&self, mut out: impl io::Write) -> io::Result<()> {
        out.write_all(&self.to_sha1_hex())
    }
}

/// Sha1 hash specific methods
impl Id {
    /// Returns ourselves as slice of 20 bytes.
    ///
    /// Panics if this instance is not a sha1 hash.
    pub fn sha1(&self) -> &[u8; SIZE_OF_SHA1_DIGEST] {
        &self.0
    }

    /// Return ourselves as array of 40 hexadecimal bytes.
    ///
    /// Panics if this instance is not a sha1 hash.
    pub fn to_sha1_hex(&self) -> [u8; SIZE_OF_SHA1_DIGEST * 2] {
        let mut hex_buf = [0u8; 40];
        hex::encode_to_slice(self.0, &mut hex_buf).expect("we can count");
        hex_buf
    }

    /// Return ourselves as hexadecimal string with a length of 40 bytes.
    ///
    /// Panics if this instance is not a sha1 hash.
    pub fn to_sha1_hex_string(&self) -> String {
        let buf = self.to_sha1_hex();
        std::str::from_utf8(&buf).expect("hex is valid UTF-8").to_string()
    }

    /// Instantiate an Digest from 20 bytes of a Sha1 digest.
    pub fn new_sha1(id: [u8; SIZE_OF_SHA1_DIGEST]) -> Self {
        Id(id)
    }

    /// Instantiate an Digest from a slice 20 borrowed bytes of a Sha1 digest.
    ///
    /// Panics of the slice doesn't have a length of 20.
    pub fn from_20_bytes(b: &[u8]) -> Id {
        let mut id = [0; SIZE_OF_SHA1_DIGEST];
        id.copy_from_slice(b);
        Id(id)
    }

    /// Instantiate an Digest from a borrowed array of 20 bytes of a Sha1 digest.
    pub fn from_borrowed_sha1(b: &[u8; SIZE_OF_SHA1_DIGEST]) -> Id {
        let mut id = [0; SIZE_OF_SHA1_DIGEST];
        id.copy_from_slice(&b[..]);
        Id(id)
    }

    /// Returns an Digest representing a Sha1 with whose memory is zeroed.
    pub fn null_sha1() -> Id {
        Id([0u8; 20])
    }
}

impl From<[u8; SIZE_OF_SHA1_DIGEST]> for Id {
    fn from(v: [u8; 20]) -> Self {
        Self::new_sha1(v)
    }
}

impl Deref for Id {
    type Target = [u8; SIZE_OF_SHA1_DIGEST];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.to_sha1_hex().as_bstr())
    }
}
