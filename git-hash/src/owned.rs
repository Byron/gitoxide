use crate::{borrowed, SIZE_OF_SHA1_DIGEST};
use bstr::ByteSlice;
use quick_error::quick_error;
use std::{fmt, io, ops::Deref};

quick_error! {
    /// An error returned by [`Digest::from_40_bytes_in_hex()`]
    #[derive(Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        HexDecode(err: String) {
            display("Failed to hex hash: {}", err)
        }
    }
}

/// An owned hash identifying objects, most commonly Sha1
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Digest([u8; SIZE_OF_SHA1_DIGEST]);

/// Access and conversion
impl Digest {
    /// Returns the kind of hash used in this `Id`
    pub fn kind(&self) -> crate::Kind {
        crate::Kind::Sha1
    }
    /// Return a borrowed version of this instance
    pub fn to_borrowed(&self) -> borrowed::Digest<'_> {
        borrowed::Digest::from(&self.0)
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
impl Digest {
    /// Create an instance from a `buffer` of 40 bytes encoded with hexadecimal notation.
    ///
    /// Such a buffer can be obtained using [`write_hex_to(buffer)`][Digest::write_hex_to()]
    pub fn from_40_bytes_in_hex(buffer: &[u8]) -> Result<Digest, Error> {
        use hex::FromHex;
        Ok(Digest(
            <[u8; 20]>::from_hex(buffer).map_err(|err| Error::HexDecode(err.to_string()))?,
        ))
    }
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

    /// Instantiate an Id from 20 bytes of a Sha1 digest.
    pub fn new_sha1(id: [u8; SIZE_OF_SHA1_DIGEST]) -> Self {
        Digest(id)
    }

    /// Instantiate an Id from a slice 20 borrowed bytes of a Sha1 digest.
    ///
    /// Panics of the slice doesn't have a length of 20.
    pub fn from_20_bytes(b: &[u8]) -> Digest {
        let mut id = [0; SIZE_OF_SHA1_DIGEST];
        id.copy_from_slice(b);
        Digest(id)
    }

    /// Instantiate an Id from a borrowed array of 20 bytes of a Sha1 digest.
    pub fn from_borrowed_sha1(b: &[u8; SIZE_OF_SHA1_DIGEST]) -> Digest {
        let mut id = [0; SIZE_OF_SHA1_DIGEST];
        id.copy_from_slice(&b[..]);
        Digest(id)
    }

    /// Returns an Id representing a Sha1 with whose memory is zeroed.
    pub fn null_sha1() -> Digest {
        Digest([0u8; 20])
    }
}

impl From<[u8; SIZE_OF_SHA1_DIGEST]> for Digest {
    fn from(v: [u8; 20]) -> Self {
        Self::new_sha1(v)
    }
}

impl Deref for Digest {
    type Target = [u8; SIZE_OF_SHA1_DIGEST];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.to_sha1_hex().as_bstr())
    }
}
