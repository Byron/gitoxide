#![forbid(unsafe_code)]

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
