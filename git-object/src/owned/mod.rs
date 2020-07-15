//! Owned objects for use with serialization.

pub(crate) const NL: &[u8; 1] = b"\n";
pub(crate) const SPACE: &[u8; 1] = b" ";

mod convert;
mod object;
mod ser;
mod tag;

pub use object::*;
pub use tag::Tag;
