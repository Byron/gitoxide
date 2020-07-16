//! Owned objects for use with serialization.

pub(crate) const NL: &[u8; 1] = b"\n";
pub(crate) const SPACE: &[u8; 1] = b" ";

mod convert;
mod ser;

mod id;
pub use id::*;

mod tag;
pub use tag::Tag;

pub mod tree;
pub use tree::Tree;

mod commit;
pub use commit::Commit;

mod object;
pub use object::*;
