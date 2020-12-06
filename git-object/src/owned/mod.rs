//! Owned objects are Commits, Trees, Blobs and Tags that can be mutated and serialized.

pub(crate) const NL: &[u8; 1] = b"\n";
pub(crate) const SPACE: &[u8; 1] = b" ";

mod convert;
mod ser;

mod id;
pub use id::{Error, Id};

mod tag;
pub use tag::Tag;

///
pub mod tree;
pub use tree::Tree;

mod commit;
pub use commit::Commit;

mod blob {
    use std::io;

    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Blob {
        pub data: Vec<u8>,
    }

    impl Blob {
        pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
            out.write_all(&self.data)
        }
    }
}
pub use blob::Blob;

pub mod signature;
#[doc(inline)]
pub use signature::Signature;

mod object;
pub use object::Object;
