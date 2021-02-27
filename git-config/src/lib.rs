#![forbid(unsafe_code)]

//! # git_config
//!
//! This crate is a high performance `git-config` file reader and writer. It
//! exposes a high level API to parse, read, and write [`git-config` files],
//! which are loosely based on the [INI file format].
//!
//! [`git-config` files]: https://git-scm.com/docs/git-config#_configuration_file
//! [INI file format]: https://en.wikipedia.org/wiki/INI_file

// Cargo.toml cannot have self-referential dependencies, so you can't just
// specify the actual serde crate when you define a feature called serde. We
// instead call the serde crate as serde_crate and then rename the crate to
// serde, to get around this in an intuitive manner.
#[cfg(feature = "serde")]
extern crate serde_crate as serde;

// mod de;
// mod ser;
pub mod config;
mod error;
pub mod parser;
pub mod values;

// pub use de::{from_str, Deserializer};
pub use error::{Error, Result};
// pub use ser::{to_string, Serializer};

#[cfg(test)]
pub mod test_util;
