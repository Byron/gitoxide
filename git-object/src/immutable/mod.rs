//! Immutable objects are read-only structures referencing most data from [a byte slice][crate::ObjectRef::from_bytes()].
//!
//! Immutable objects are expected to be deserialized from bytes that acts as backing store, and they
//! cannot be mutated or serialized. Instead, one will [convert][crate::ObjectRef::into_owned()] them into their [`mutable`][crate::mutable] counterparts
//! which support mutation and serialization.

///
pub mod object;

pub(crate) mod parse;
