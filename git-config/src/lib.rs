#![forbid(unsafe_code)]

// mod de;
pub mod config;
mod error;
// mod ser;
pub mod parser;
pub mod values;

// pub use de::{from_str, Deserializer};
pub use error::{Error, Result};
// pub use ser::{to_string, Serializer};
