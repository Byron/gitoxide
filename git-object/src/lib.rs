#![forbid(unsafe_code)]

pub type BytesOwned = bstr::BString;
pub type Bytes = bstr::BStr;
pub use bstr::ByteSlice;

pub mod borrowed;
mod types;

pub use types::*;
