#![forbid(unsafe_code)]
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
//! | [`GitConfig`] | Accelerated wrapper for reading and writing values. | On some reads[^1] |
//! | [`Parser`]    | Syntactic event emitter for `git-config` files.     | Yes               |
//! | [`Value`]     | Wrappers for `git-config` value types.              | Yes               |
//!
//! This crate also exposes efficient value normalization which unescapes
//! characters and removes quotes through the `normalize_*` family of functions,
//! located in the [`values`] module.
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
//! [`GitConfig`]: crate::file::GitConfig
//! [`Parser`]: crate::parser::Parser
//! [`Value`]: crate::values::Value
//! [`values`]: crate::values
//! [`nom`]: https://github.com/Geal/nom

// Cargo.toml cannot have self-referential dependencies, so you can't just
// specify the actual serde crate when you define a feature called serde. We
// instead call the serde crate as serde_crate and then rename the crate to
// serde, to get around this in an intuitive manner.
#[cfg(feature = "serde")]
extern crate serde_crate as serde;

pub mod file;
pub mod fs;
pub mod parser;
pub mod values;

// mod de;
// mod ser;
// mod error;
// pub use de::{from_str, Deserializer};
// pub use error::{Error, Result};
// pub use ser::{to_string, Serializer};

#[cfg(test)]
pub mod test_util;
