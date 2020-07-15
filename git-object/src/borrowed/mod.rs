//! Borrowed objects are expected to be deserialized from bytes that acts as backing store, and they
//! can not be serialized directly. Instead, one will convert them into their `owned` counterparts,
//! which support serialization.
mod commit;
pub use commit::Commit;

mod tag;
pub use tag::Tag;

pub mod tree;
pub use tree::Tree;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Blob<'a> {
    pub data: &'a [u8],
}

mod object;
pub use object::*;

mod parse;
