//! Immutable objects are read-only structures referencing most data from [a byte slice][Object::from_bytes()].
//!
//! Immutable objects are expected to be deserialized from bytes that acts as backing store, and they
//! cannot be mutated or serialized. Instead, one will [convert][Object::into_mutable()] them into their [`mutable`][crate::mutable] counterparts
//! which support mutation and serialization.
pub use blob::Blob;
pub use commit::Commit;
pub use object::{Object, Signature};
pub use tag::Tag;
pub use tree::{Tree, TreeIter};

mod blob;
mod commit;
///
pub mod object;
mod parse;
mod tag;
///
pub mod tree;
