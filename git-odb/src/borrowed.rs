//! Contains a borrowed Object bound to a buffer holding its decompressed data.

/// A borrowed object using a borrowed slice as backing buffer.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Object<'a> {
    /// kind of object
    pub kind: git_object::Kind,
    /// decoded, decompressed data, owned by a backing store.
    pub data: &'a [u8],
}

impl<'a> Object<'a> {
    /// Decodes the data in the backing slice into a [`crate::borrowed::Object`], allowing to access all of its data
    /// conveniently. The cost of parsing an object is negligible.
    ///
    /// **Note** that [owned, decoded objects][git_object::owned::Object] can be created from a [`crate::borrowed::Object`]
    /// using [`crate::borrowed::Object::into_owned()`].
    pub fn decode(&self) -> Result<git_object::borrowed::Object<'_>, git_object::borrowed::Error> {
        Ok(match self.kind {
            git_object::Kind::Tree => {
                git_object::borrowed::Object::Tree(git_object::borrowed::Tree::from_bytes(self.data)?)
            }
            git_object::Kind::Blob => {
                git_object::borrowed::Object::Blob(git_object::borrowed::Blob { data: self.data })
            }
            git_object::Kind::Commit => {
                git_object::borrowed::Object::Commit(git_object::borrowed::Commit::from_bytes(self.data)?)
            }
            git_object::Kind::Tag => {
                git_object::borrowed::Object::Tag(git_object::borrowed::Tag::from_bytes(self.data)?)
            }
        })
    }
}

/// Types supporting object hash verification
pub mod verify {
    use crate::{hash, loose};
    use std::io;

    /// Returned by [`crate::borrowed::Object::verify_checksum()`]
    #[derive(thiserror::Error, Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Object expected to have id {desired}, but actual id was {actual}")]
        ChecksumMismatch {
            desired: git_hash::ObjectId,
            actual: git_hash::ObjectId,
        },
    }

    impl crate::borrowed::Object<'_> {
        /// Compute the checksum of `self` and compare it with the `desired` hash.
        /// If the hashes do not match, an [`Error`] is returned, containing the actual
        /// hash of `self`.
        pub fn verify_checksum(&self, desired: impl AsRef<git_hash::oid>) -> Result<(), Error> {
            let desired = desired.as_ref();
            let mut sink = hash::Write::new(io::sink(), desired.kind());

            loose::object::header::encode(self.kind, self.data.len() as u64, &mut sink).expect("hash to always work");
            sink.hash.update(&self.data);

            let actual_id = git_hash::ObjectId::from(sink.hash.digest());
            if desired != actual_id {
                return Err(Error::ChecksumMismatch {
                    desired: desired.into(),
                    actual: actual_id,
                });
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_object() {
        assert_eq!(std::mem::size_of::<Object<'_>>(), 24, "this shouldn't change unnoticed");
    }
}
