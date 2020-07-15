#![forbid(unsafe_code)]

use bstr::{BStr, BString, ByteSlice};

pub use bstr;

pub mod borrowed;
mod types;

pub use types::*;

pub mod owned;
