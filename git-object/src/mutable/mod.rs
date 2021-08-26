//! Mutable objects with each field being separately allocated and changeable.
//!
//! Mutable objects are Commits, Trees, Blobs and Tags that can be changed and serialized.
//! They either created using object [construction][crate::Object] or by [deserializing existing objects][crate::ObjectRef::from_bytes()]
//! and converting these [into mutable copies][crate::ObjectRef::into_owned()] for adjustments.

mod convert;

mod commit;

mod object;
