#![deny(unsafe_code, rust_2018_idioms)]
// #![warn(missing_docs)]
// #![warn(clippy::pedantic, clippy::nursery)]

//! # `git_config`
//!
//! This crate is a high performance `git-config` file reader and writer. It
//! exposes a high level API to parse, read, and write [`git-config` files],
//! which are loosely based on the [INI file format].
//!
//! This crate has a few primary offerings and various accessory functions. The
//! table below gives a brief explanation of all offerings, loosely in order
//! from the highest to lowest abstraction.
//!
//! | Offering      | Description                                         | Zero-copy?        |
//! | ------------- | --------------------------------------------------- | ----------------- |
//! | [`File`] | Accelerated wrapper for reading and writing values. | On some reads[^1] |
//! | [`parse::State`]    | Syntactic events for `git-config` files.     | Yes               |
//! | [`value`]    | Wrappers for `git-config` value types.              | Yes               |
//!
//! This crate also exposes efficient value normalization which unescapes
//! characters and removes quotes through the `normalize_*` family of functions,
//! located in the [`value`] module.
//!
//! # Zero-copy versus zero-alloc
//!
//! We follow [`nom`]'s definition of "zero-copy":
//!
//! > If a parser returns a subset of its input data, it will return a slice of
//! > that input, without copying.
//!
//! Due to the syntax of `git-config`, we must allocate at the parsing level
//! (and thus higher level abstractions must allocate as well) in order to
//! provide a meaningful event stream. That being said, all operations with the
//! parser is still zero-copy. Higher level abstractions may have operations
//! that are zero-copy, but are not guaranteed to do so.
//!
//! However, we intend to be performant as possible, so allocations are
//! limited restricted and we attempt to avoid copying whenever possible.
//!
//! [^1]: When read values do not need normalization.
//!
//! [`git-config` files]: https://git-scm.com/docs/git-config#_configuration_file
//! [INI file format]: https://en.wikipedia.org/wiki/INI_file
//! [`File`]: crate::File
//! [`parse::State`]: crate::parse::Events
//! [`value`]: crate::value
//! [`nom`]: https://github.com/Geal/nom
//!
//! ## Feature Flags
#![cfg_attr(
feature = "document-features",
cfg_attr(doc, doc = ::document_features::document_features!())
)]

pub mod file;
pub mod fs;
pub mod lookup;
pub mod parse;
pub mod value;
mod values;
pub use values::*;

mod types;
pub use types::File;

mod permissions;
pub use permissions::Permissions;
