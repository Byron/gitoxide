//! Borrowed objects are read-only structures referencing most data in a byte based backing store.
//!
//! Borrowed objects are expected to be deserialized from bytes that acts as backing store, and they
//! cannot mutated or serialized. Instead, one will convert them into their `owned` counterparts,
//! which support mutation and serialization.
mod commit;
pub use commit::Commit;

mod id;
pub use id::*;

mod tag;
pub use tag::Tag;

pub mod tree;
#[doc(inline)]
pub use tree::Tree;

mod blob {
    use std::convert::Infallible;

    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Blob<'a> {
        pub data: &'a [u8],
    }

    impl<'a> Blob<'a> {
        pub fn from_bytes(input: &[u8]) -> Result<Blob<'_>, Infallible> {
            Ok(Blob { data: input })
        }
    }
}

pub use blob::Blob;

mod object;
pub use object::*;

mod parse;
