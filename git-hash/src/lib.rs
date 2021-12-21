//! This crate provides types for identifying git objects using a hash digest.
//!
//! These are provided in borrowed versions as well as owned ones.
#![deny(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]

mod borrowed;
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
        /// Such a buffer can be obtained using [`write_hex_to(buffer)`][ObjectId::write_hex_to()]
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
    Sha1,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::Sha1
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
    #[inline]
    pub const fn from_len_in_bytes(bytes: usize) -> Option<Self> {
        Some(match bytes {
            20 => Kind::Sha1,
            _ => return None,
        })
    }

    /// Create a null-id of our hash kind.
    #[inline]
    pub fn null(&self) -> &'static oid {
        match self {
            Kind::Sha1 => oid::null_sha1(),
        }
    }

    /// Create a null-id of our hash kind.
    #[inline]
    pub const fn null_owned(&self) -> ObjectId {
        match self {
            Kind::Sha1 => ObjectId::null_sha1(),
        }
    }
}
