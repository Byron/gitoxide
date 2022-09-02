//! This crate provides types for identifying git objects using a hash digest.
//!
//! These are provided in borrowed versions as well as owned ones.
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

#[path = "oid.rs"]
mod borrowed;
pub use borrowed::oid;

mod object_id;
pub use object_id::ObjectId;

///
pub mod prefix;

/// An partial owned hash possibly identifying an object uniquely,
/// whose non-prefix bytes are zeroed.
#[derive(PartialEq, Eq, Hash, Ord, PartialOrd, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Prefix {
    bytes: ObjectId,
    hex_len: usize,
}

#[allow(missing_docs)]
pub mod decode {
    use std::str::FromStr;

    use quick_error::quick_error;

    use crate::object_id::ObjectId;

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

mod kind;
