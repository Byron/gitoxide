//! This crate provides types for [read-only git objects][immutable::Object] backed by bytes provided in git's serialization format
//! as well as [mutable versions][mutable::Object] of these. The latter can be serialized into git's serialization format for objects.
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]

use bstr::{BStr, BString, ByteSlice};

/// For convenience to allow using `bstr` without adding it to own cargo manifest.
pub use bstr;

pub mod immutable;
pub mod mutable;

mod types;
pub use types::{tree, Error, Kind, Sign, Time};

///
pub mod commit;
