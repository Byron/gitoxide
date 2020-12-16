//! This crate provides types for identifying git objects using a hash digest.
//!
//! These are provided in borrowed versions as well as owned ones.
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]

/// For convenience to allow using `bstr` without adding it to own cargo manifest
pub use bstr;

#[allow(missing_docs)]
pub mod borrowed;
#[allow(missing_docs)]
pub mod owned;

/// The size of a SHA1 hash digest in bytes
pub const SIZE_OF_SHA1_DIGEST: usize = 20;

/// Denotes the kind of function to produce a `Digest`
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

    impl<'a> From<borrowed::Digest<'a>> for owned::Digest {
        fn from(v: borrowed::Digest<'a>) -> Self {
            owned::Digest::from_borrowed_sha1(v.sha1())
        }
    }
}
