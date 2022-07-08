#![deny(missing_docs, unsafe_code, rust_2018_idioms)]

//! # `git_config`
//!
//! This crate is a high performance `git-config` file reader and writer. It
//! exposes a high level API to parse, read, and write [`git-config` files].
//!
//! This crate has a few primary offerings and various accessory functions. The
//! table below gives a brief explanation of all offerings, loosely in order
//! from the highest to lowest abstraction.
//!
//! | Offering      | Description                                         | Zero-copy?        |
//! | ------------- | --------------------------------------------------- | ----------------- |
//! | [`File`] | Accelerated wrapper for reading and writing values. | On some reads[^1] |
//! | [`parse::State`]    | Syntactic events for `git-config` files.     | Yes               |
//! | value wrappers | Wrappers for `git-config` value types.            | Yes               |
//!
//! This crate also exposes efficient value normalization which unescapes
//! characters and removes quotes through the `normalize_*` family of functions,
//! located in the [`value`] module.
//!
//! [^1]: When read values do not need normalization.
//!
//! [`git-config` files]: https://git-scm.com/docs/git-config#_configuration_file
//! [`File`]: crate::File
//! [`parse::State`]: crate::parse::Events
//! [`nom`]: https://github.com/Geal/nom
//!
//! ## Feature Flags
#![cfg_attr(
feature = "document-features",
cfg_attr(doc, doc = ::document_features::document_features!())
)]

pub mod file;
///
pub mod fs;
///
pub mod lookup;
pub mod parse;
///
pub mod value;
mod values;
pub use values::*;

mod types;
pub use types::File;

mod permissions;
pub use permissions::Permissions;
