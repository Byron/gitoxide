//! Borrowed objects are read-only structures referencing most data in a byte based backing store.
//!
//! Borrowed objects are expected to be deserialized from bytes that acts as backing store, and they
//! cannot mutated or serialized. Instead, one will convert them into their `owned` counterparts,
//! which support mutation and serialization.
mod commit;
pub use commit::Commit;

mod id;
pub use id::Id;

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
