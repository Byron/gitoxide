//! This crate provides types for identifying git objects using a hash digest.
//!
//! These are provided in borrowed versions as well as owned ones.
#![deny(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]

mod borrowed;

use std::{convert::TryFrom, str::FromStr};

pub use borrowed::oid;

mod owned;
pub use owned::ObjectId;

#[allow(missing_docs)]
pub mod decode {
    use std::str::FromStr;

    use quick_error::quick_error;

    use crate::owned::ObjectId;

    quick_error! {
        /// An error returned by [`ObjectId::from_40_bytes_in_hex()`]
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            InvalidHexEncodingLength(length: usize) {
                display("A hash sized {} hexadecimal characters is invalid", length)
            }
        }
    }

    /// Hash decoding
    impl ObjectId {
        /// Create an instance from a `buffer` of 40 bytes encoded with hexadecimal notation.
        ///
        /// Such a buffer can be obtained using [`oid::write_hex_to(buffer)`][super::oid::write_hex_to()]
        pub fn from_hex(buffer: &[u8]) -> Result<ObjectId, Error> {
            use hex::FromHex;
            match buffer.len() {
                40 => Ok(ObjectId::Sha1(
                    <[u8; 20]>::from_hex(buffer).expect("our length check is correct thus we can decode hex"),
                )),
                len => Err(Error::InvalidHexEncodingLength(len)),
            }
        }
    }

    impl FromStr for ObjectId {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::from_hex(s.as_bytes())
        }
    }
}

/// The size of a SHA1 hash digest in bytes
const SIZE_OF_SHA1_DIGEST: usize = 20;

/// Denotes the kind of function to produce a `Id`
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Kind {
    /// The Sha1 hash with 160 bits.
    Sha1 = 1,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::Sha1
    }
}

impl TryFrom<u8> for Kind {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Kind::Sha1,
            unknown => return Err(unknown),
        })
    }
}

impl FromStr for Kind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "sha1" | "SHA1" => Kind::Sha1,
            other => return Err(other.into()),
        })
    }
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Sha1 => f.write_str("SHA1"),
        }
    }
}

impl Kind {
    /// Returns the shortest hash we support
    #[inline]
    pub const fn shortest() -> Self {
        Self::Sha1
    }

    /// Returns the longest hash we support
    #[inline]
    pub const fn longest() -> Self {
        Self::Sha1
    }

    /// Returns a buffer suitable to hold the longest possible hash in hex.
    #[inline]
    pub const fn hex_buf() -> [u8; Kind::longest().len_in_hex()] {
        [0u8; Kind::longest().len_in_hex()]
    }

    /// Returns a buffer suitable to hold the longest possible hash as raw bytes.
    #[inline]
    pub const fn buf() -> [u8; Kind::longest().len_in_bytes()] {
        [0u8; Kind::longest().len_in_bytes()]
    }

    /// Returns the amount of ascii-characters needed to encode this has in hex
    #[inline]
    pub const fn len_in_hex(&self) -> usize {
        match self {
            Kind::Sha1 => 40,
        }
    }
    /// Returns the amount of bytes taken up by the hash of the current kind
    #[inline]
    pub const fn len_in_bytes(&self) -> usize {
        match self {
            Kind::Sha1 => 20,
        }
    }

    /// Converts a size in bytes as obtained by `Kind::len_in_bytes()` into the corresponding hash kind, if possible.
    ///
    /// **Panics** if the hash length doesn't match a known hash.
    ///
    /// NOTE that this method isn't public as it shouldn't be encouraged to assume all hashes have the same length.
    /// However, if there should be such a thing, our `oid` implementation will have to become an enum and it's pretty breaking
    /// to the way it's currently being used as auto-dereffing doesn't work anymore. Let's hope it won't happen.
    // TODO: make 'const' once Rust 1.57 is more readily available in projects using 'gitoxide'.
    #[inline]
    pub(crate) fn from_len_in_bytes(bytes: usize) -> Self {
        match bytes {
            20 => Kind::Sha1,
            _ => panic!("BUG: must be called only with valid hash lengths produced by len_in_bytes()"),
        }
    }

    /// Create a null-id of our hash kind.
    #[inline]
    pub fn null_ref(&self) -> &'static oid {
        match self {
            Kind::Sha1 => oid::null_sha1(),
        }
    }

    /// Create a null-id of our hash kind.
    #[inline]
    pub const fn null(&self) -> ObjectId {
        match self {
            Kind::Sha1 => ObjectId::null_sha1(),
        }
    }
}
