#![forbid(unsafe_code)]

use bstr::{BStr, BString, ByteSlice};

/// For convenience
pub use bstr;

pub mod borrowed;
pub mod owned;

mod types;
pub use types::*;
