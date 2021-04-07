//! Borrowed objects are read-only structures referencing most data from [a byte slice][Object::from_bytes()].
//!
//! Borrowed objects are expected to be deserialized from bytes that acts as backing store, and they
//! cannot be mutated or serialized. Instead, one will [convert][Object::into_owned()] them into their [`owned`][crate::owned] counterparts
//! which support mutation and serialization.
mod commit;
pub use commit::Commit;

mod tag;
pub use tag::Tag;

///
pub mod tree;
pub use tree::Tree;

mod blob;
pub use blob::Blob;

mod object;
pub use object::{Error, Object, Signature};

mod parse;
