//! This crate contains an assortment of utilities to deal with paths and their conversions.
//!
//! Git treats paths as bytes, using UTF-8 for native path conversion on Windows. Paths in Git files use
//! slashes as separators, with conversions to native separators performed on Windows as needed.
//!
//! ## Examples
//!
//! ```
//! use bstr::ByteSlice;
//!
//! use std::path::Path;
//!
//! let normalized = gix_path::normalize(Path::new("a/./b/..").into(), Path::new("/cwd")).unwrap();
//! assert_eq!(normalized.as_ref(), Path::new("a"));
//!
//! let unix = gix_path::to_unix_separators(b"dir\\subdir\\file".as_bstr());
//! assert_eq!(unix.as_ref(), b"dir/subdir/file".as_bstr());
//! ```
//!
//! ## Path encodings
//!
//! Unix paths contain arbitrary bytes, which these conversions preserve without UTF-8 validation.
//! Windows paths use potentially ill-formed UTF-16: even modern Windows permits unpaired surrogate
//! code units. Such paths cannot be represented losslessly as UTF-8, so conversion to Git path bytes
//! returns an error. Likewise, converting Git path bytes to a native path requires valid UTF-8 on
//! non-Unix platforms. Valid surrogate pairs, including those used for emoji, convert normally.
//!
//! Conversions preserve borrowed and owned inputs and report encoding failures through
//! [`gix_error::Result`], retaining the concrete encoding error as a source.
#![deny(missing_docs)]
#![cfg_attr(not(test), deny(unsafe_code))]

/// A dummy type to represent path specs and help finding all spots that take path specs once it is implemented.
mod convert;
pub use convert::*;

mod util;
pub use util::{is_absolute, windows_drive_prefix};

///
pub mod realpath;
pub use realpath::function::{realpath, realpath_opts};

/// Information about the environment in terms of locations of resources.
pub mod env;

///
pub mod relative_path;
pub use relative_path::types::RelativePath;
