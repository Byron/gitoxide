//! # `gix_config`
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
//! # Known differences to the `git config` specification
//!
//! - Legacy headers like `[section.subsection]` are supposed to be turned into to lower case and compared
//!   case-sensitively. We keep its case and compare case-insensitively.
//!
//! [^1]: When read values do not need normalization and it wasn't parsed in 'owned' mode.
//!
//! [`git-config` files]: https://git-scm.com/docs/git-config#_configuration_file
//! [`File`]: crate::File
//! [`parse::State`]: crate::parse::Events
//! [`nom`]: https://github.com/Geal/nom
//!
//! ## Feature Flags
#![cfg_attr(
    all(doc, feature = "document-features"),
    doc = ::document_features::document_features!()
)]
#![cfg_attr(all(doc, feature = "document-features"), feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

pub mod file;

///
#[allow(clippy::empty_docs)]
pub mod lookup;
pub mod parse;
///
#[allow(clippy::empty_docs)]
pub mod value;
pub use gix_config_value::{color, integer, path, Boolean, Color, Integer, Path};

mod key;
pub use key::{AsKey, KeyRef};
mod types;
pub use types::{File, Source};
///
#[allow(clippy::empty_docs)]
pub mod source;
