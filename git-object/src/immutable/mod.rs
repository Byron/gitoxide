//! Immutable objects are read-only structures referencing most data from [a byte slice][Object::from_bytes()].
//!
//! Immutable objects are expected to be deserialized from bytes that acts as backing store, and they
//! cannot be mutated or serialized. Instead, one will [convert][Object::into_mutable()] them into their [`mutable`][crate::mutable] counterparts
//! which support mutation and serialization.

mod blob;
pub use blob::Blob;

///
pub mod commit;
pub use commit::{iter::Iter as CommitIter, Commit};

///
pub mod object;
pub use object::Object;

///
pub mod tag;
pub use tag::{iter::Iter as TagIter, Tag};

///
pub mod tree;
pub use tree::{Tree, TreeIter};

mod parse;
