#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

use bstr::{BStr, BString, ByteSlice};

/// For convenience to allow using `bstr` without adding it to own cargo manifest
pub use bstr;

pub mod borrowed;
pub mod owned;

mod types;
pub use types::*;

pub mod commit;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum HashKind {
    Sha1,
}

impl Default for HashKind {
    fn default() -> Self {
        HashKind::Sha1
    }
}
