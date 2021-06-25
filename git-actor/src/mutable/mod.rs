//! Mutable objects with each field being separately allocated and changeable.
//!
//! Mutable objects are Commits, Trees, Blobs and Tags that can be changed and serialized.
//! They either created using object [construction][Object] or by [deserializing existing objects][crate::immutable::Object::from_bytes()]
//! and converting these [into mutable copies][crate::immutable::Object::into_mutable()] for adjustments.

pub(crate) const SPACE: &[u8; 1] = b" ";

mod convert {
    use crate::{immutable, mutable};

    impl From<immutable::Signature<'_>> for mutable::Signature {
        fn from(other: immutable::Signature<'_>) -> mutable::Signature {
            let immutable::Signature { name, email, time } = other;
            mutable::Signature {
                name: name.to_owned(),
                email: email.to_owned(),
                time,
            }
        }
    }
}

///
pub mod signature;
#[doc(inline)]
pub use signature::Signature;
