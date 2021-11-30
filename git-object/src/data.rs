//! Contains a borrowed Object bound to a buffer holding its decompressed data.

use crate::{BlobRef, CommitRef, CommitRefIter, Kind, ObjectRef, TagRef, TagRefIter, TreeRef, TreeRefIter};

use crate::Data;

impl<'a> Data<'a> {
    /// Constructs a new data object from `kind` and `data`.
    pub fn new(kind: Kind, data: &'a [u8]) -> Data<'a> {
        Data { kind, data }
    }
    /// Decodes the data in the backing slice into a [`git_object::ObjectRef`], allowing to access all of its data
    /// conveniently. The cost of parsing an object is negligible.
    ///
    /// **Note** that [mutable, decoded objects][crate::Object] can be created from [`Data`]
    /// using [`crate::ObjectRef::into_owned()`].
    pub fn decode(&self) -> Result<ObjectRef<'a>, crate::decode::Error> {
        Ok(match self.kind {
            crate::Kind::Tree => ObjectRef::Tree(TreeRef::from_bytes(self.data)?),
            crate::Kind::Blob => ObjectRef::Blob(BlobRef { data: self.data }),
            crate::Kind::Commit => ObjectRef::Commit(CommitRef::from_bytes(self.data)?),
            crate::Kind::Tag => ObjectRef::Tag(TagRef::from_bytes(self.data)?),
        })
    }

    /// Returns this object as tree iterator to parse entries one at a time to avoid allocations, or
    /// `None` if this is not a tree object.
    pub fn try_into_tree_iter(self) -> Option<TreeRefIter<'a>> {
        match self.kind {
            crate::Kind::Tree => Some(TreeRefIter::from_bytes(self.data)),
            _ => None,
        }
    }

    /// Returns this object as commit iterator to parse tokens one at a time to avoid allocations, or
    /// `None` if this is not a commit object.
    pub fn try_into_commit_iter(self) -> Option<CommitRefIter<'a>> {
        match self.kind {
            crate::Kind::Commit => Some(CommitRefIter::from_bytes(self.data)),
            _ => None,
        }
    }

    /// Returns this object as tag iterator to parse tokens one at a time to avoid allocations, or
    /// `None` if this is not a tag object.
    pub fn try_into_tag_iter(self) -> Option<TagRefIter<'a>> {
        match self.kind {
            crate::Kind::Tag => Some(TagRefIter::from_bytes(self.data)),
            _ => None,
        }
    }
}

/// Types supporting object hash verification
pub mod verify {
    use quick_error::quick_error;

    quick_error! {
        /// Returned by [`crate::Data::verify_checksum()`]
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            ChecksumMismatch {desired: git_hash::ObjectId, actual: git_hash::ObjectId} {
                display("Object expected to have id {}, but actual id was {}", desired, actual)
            }
        }
    }

    impl crate::Data<'_> {
        /// Compute the checksum of `self` and compare it with the `desired` hash.
        /// If the hashes do not match, an [`Error`] is returned, containing the actual
        /// hash of `self`.
        pub fn verify_checksum(&self, desired: impl AsRef<git_hash::oid>) -> Result<(), Error> {
            let desired = desired.as_ref();
            let mut hasher = git_features::hash::hasher(desired.kind());
            hasher.update(&crate::encode::loose_header(self.kind, self.data.len()));
            hasher.update(self.data);

            let actual_id = git_hash::ObjectId::from(hasher.digest());
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
        assert_eq!(std::mem::size_of::<Data<'_>>(), 24, "this shouldn't change unnoticed");
    }
}
