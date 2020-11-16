//! Contains a borrowed Object bound to a buffer holding its decompressed data.
use git_object::borrowed;

/// A borrowed object using a borrowed slice as backing buffer.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Object<'a> {
    /// kind of object
    pub kind: git_object::Kind,
    /// decoded, decompressed data, owned by a backing store.
    pub data: &'a [u8],
}

impl<'a> Object<'a> {
    /// Decodes the data in the backing slice into a [`borrowed::Object`], allowing to access all of its data
    /// conveniently. The cost of parsing an object is negligible.
    ///
    /// **Note** that [owned, decoded objects][git_object::owned::Object] can be created from a [`borrowed::Object`]
    /// using [`borrowed::Object::into_owned()`].
    pub fn decode(&self) -> Result<borrowed::Object<'_>, borrowed::Error> {
        Ok(match self.kind {
            git_object::Kind::Tree => borrowed::Object::Tree(borrowed::Tree::from_bytes(self.data)?),
            git_object::Kind::Blob => borrowed::Object::Blob(borrowed::Blob { data: self.data }),
            git_object::Kind::Commit => borrowed::Object::Commit(borrowed::Commit::from_bytes(self.data)?),
            git_object::Kind::Tag => borrowed::Object::Tag(borrowed::Tag::from_bytes(self.data)?),
        })
    }
}

/// Types supporting object hash verification
pub mod verify {
    use crate::{hash, loose};
    use git_object::{borrowed, owned};
    use std::io;

    /// Returned by [`crate::borrowed::Object::verify_checksum()`]
    #[derive(thiserror::Error, Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Object expected to have id {desired}, but actual id was {actual}")]
        ChecksumMismatch { desired: owned::Id, actual: owned::Id },
    }

    impl crate::borrowed::Object<'_> {
        /// Compute the checksum of `self` and compare it with the `desired` hash.
        /// If the hashes do not match, an [`Error`] is returned, containing the actual
        /// hash of `self`.
        pub fn verify_checksum(&self, desired: borrowed::Id<'_>) -> Result<(), Error> {
            let mut sink = hash::Write::new(io::sink(), desired.kind());

            loose::object::header::encode(self.kind, self.data.len() as u64, &mut sink).expect("hash to always work");
            sink.hash.update(&self.data);

            let actual_id = owned::Id::from(sink.hash.digest());
            if desired != actual_id.to_borrowed() {
                return Err(Error::ChecksumMismatch {
                    desired: desired.into(),
                    actual: actual_id,
                });
            }
            Ok(())
        }
    }
}
