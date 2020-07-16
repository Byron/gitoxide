//! Owned objects for use with serialization.

pub(crate) const NL: &[u8; 1] = b"\n";
pub(crate) const SPACE: &[u8; 1] = b" ";

mod convert;
mod ser;

mod id;
pub use id::*;
mod tag;
pub use tag::Tag;

mod object;
pub use object::*;
