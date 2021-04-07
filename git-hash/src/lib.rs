//! This crate provides types for identifying git objects using a hash digest.
//!
//! These are provided in borrowed versions as well as owned ones.
#![deny(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]

/// For convenience to allow using `bstr` without adding it to own cargo manifest
pub use bstr;

#[allow(missing_docs)]
pub mod borrowed;

#[allow(missing_docs)]
mod owned;
pub use owned::Id;

#[allow(missing_docs)]
pub mod decode {
    use crate::owned::Id;
    use quick_error::quick_error;

    quick_error! {
        /// An error returned by [`Id::from_40_bytes_in_hex()`]
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            HexDecode(err: String) {
                display("Failed to hex hash: {}", err)
            }
        }
    }

    /// Hash decoding
    impl Id {
        /// Create an instance from a `buffer` of 40 bytes encoded with hexadecimal notation.
        ///
        /// Such a buffer can be obtained using [`write_hex_to(buffer)`][Id::write_hex_to()]
        pub fn from_40_bytes_in_hex(buffer: &[u8]) -> Result<Id, Error> {
            use hex::FromHex;
            Ok(Id(
                <[u8; 20]>::from_hex(buffer).map_err(|err| Error::HexDecode(err.to_string()))?
            ))
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

    impl<'a> From<borrowed::Id<'a>> for owned::Id {
        fn from(v: borrowed::Id<'a>) -> Self {
            owned::Id::from_borrowed_sha1(v.sha1())
        }
    }
}
