//! This crate provides types for identifying git objects using a hash digest.
//!
//! These are provided in borrowed versions as well as owned ones.
#![deny(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]

/// For convenience to allow using `bstr` without adding it to own cargo manifest
pub use bstr;

#[allow(missing_docs)]
pub mod borrowed;
pub use borrowed::oid;

#[allow(missing_docs)]
mod owned;
pub use owned::ObjectId;

#[allow(missing_docs)]
pub mod decode {
    use crate::owned::ObjectId;
    use quick_error::quick_error;
    use std::str::FromStr;

    quick_error! {
        /// An error returned by [`Id::from_40_bytes_in_hex()`]
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
        /// Such a buffer can be obtained using [`write_hex_to(buffer)`][Id::write_hex_to()]
        pub fn from_hex(buffer: &[u8]) -> Result<ObjectId, Error> {
            use hex::FromHex;
            match buffer.len() {
                40 => Ok(ObjectId(
                    <[u8; 20]>::from_hex(buffer).expect("our length check is correct thus we can decode hex"),
                )),
                len => Err(Error::InvalidHexEncodingLength(len)),
            }
        }
    }

    impl FromStr for ObjectId {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            use hex::FromHex;
            match s.len() {
                40 => Ok(ObjectId(
                    <[u8; 20]>::from_hex(s).expect("our length check is correct thus we can decode hex"),
                )),
                len => Err(Error::InvalidHexEncodingLength(len)),
            }
        }
    }
}

/// The size of a SHA1 hash digest in bytes
pub const SIZE_OF_SHA1_DIGEST: usize = 20;

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

mod convert {
    use crate::{borrowed, owned};

    impl<'a> From<borrowed::Id<'a>> for owned::ObjectId {
        fn from(v: borrowed::Id<'a>) -> Self {
            owned::ObjectId::from_borrowed_sha1(v.sha1())
        }
    }
}
