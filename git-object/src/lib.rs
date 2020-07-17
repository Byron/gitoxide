#![forbid(unsafe_code)]

use bstr::{BStr, BString, ByteSlice};

/// For convenience to allow using `bstr` without adding it to own cargo manifest
pub use bstr;

pub mod borrowed;
pub mod owned;

mod types;
pub use types::*;
