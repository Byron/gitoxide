//! Mutable objects with each field being separately allocated and mutable.
//!
//! Owned objects are Commits, Trees, Blobs and Tags that can be mutated and serialized.
//! They either created using object [construction][Object] or by [deserializing existing objects][crate::borrowed::Object::from_bytes()]
//! and converting these [into owned copies][crate::borrowed::Object::into_owned()] for mutation.

pub(crate) const NL: &[u8; 1] = b"\n";
pub(crate) const SPACE: &[u8; 1] = b" ";

mod convert;
mod ser;

pub use git_hash::owned::Id;

mod tag;
pub use tag::Tag;

///
pub mod tree;
pub use tree::Tree;

mod commit;
pub use commit::Commit;

mod blob {
    use std::io;

    /// A mutable chunk of any [`data`][Blob::data].
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Blob {
        /// The data itself.
        pub data: Vec<u8>,
    }

    impl Blob {
        /// Write the blobs data to `out` verbatim.
        pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
            out.write_all(&self.data)
        }
    }
}
pub use blob::Blob;

///
pub mod signature;
#[doc(inline)]
pub use signature::Signature;

mod object;
pub use object::Object;
