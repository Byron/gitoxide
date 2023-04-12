//!
use std::ops::Deref;

use gix_hash::{oid, ObjectId};

use crate::{object::find, revision, Id, Object};

/// An [object id][ObjectId] infused with `Easy`.
impl<'repo> Id<'repo> {
    /// Find the [`Object`] associated with this object id, and consider it an error if it doesn't exist.
    ///
    /// # Note
    ///
    /// There can only be one `ObjectRef` per `Easy`. To increase that limit, clone the `Easy`.
    pub fn object(&self) -> Result<Object<'repo>, find::existing::Error> {
        self.repo.find_object(self.inner)
    }

    /// Try to find the [`Object`] associated with this object id, and return `None` if it's not available locally.
    ///
    /// # Note
    ///
    /// There can only be one `ObjectRef` per `Easy`. To increase that limit, clone the `Easy`.
    pub fn try_object(&self) -> Result<Option<Object<'repo>>, find::Error> {
        self.repo.try_find_object(self.inner)
    }

    /// Turn this object id into a shortened id with a length in hex as configured by `core.abbrev`.
    pub fn shorten(&self) -> Result<gix_hash::Prefix, shorten::Error> {
        let hex_len = self.repo.config.hex_len.map_or_else(
            || self.repo.objects.packed_object_count().map(calculate_auto_hex_len),
            Ok,
        )?;

        let prefix = gix_odb::store::prefix::disambiguate::Candidate::new(self.inner, hex_len)
            .expect("BUG: internal hex-len must always be valid");
        self.repo
            .objects
            .disambiguate_prefix(prefix)?
            .ok_or(shorten::Error::NotFound { oid: self.inner })
    }

    /// Turn this object id into a shortened id with a length in hex as configured by `core.abbrev`, or default
    /// to a prefix which equals our id in the unlikely error case.
    pub fn shorten_or_id(&self) -> gix_hash::Prefix {
        self.shorten().unwrap_or_else(|_| self.inner.into())
    }
}

fn calculate_auto_hex_len(num_packed_objects: u64) -> usize {
    let mut len = 64 - num_packed_objects.leading_zeros();
    len = (len + 1) / 2;
    len.max(7) as usize
}

///
pub mod shorten {
    /// Returned by [`Id::prefix()`][super::Id::shorten()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        PackedObjectsCount(#[from] gix_odb::store::load_index::Error),
        #[error(transparent)]
        DisambiguatePrefix(#[from] gix_odb::store::prefix::disambiguate::Error),
        #[error("Id could not be shortened as the object with id {} could not be found", .oid)]
        NotFound { oid: gix_hash::ObjectId },
    }
}

impl<'repo> Deref for Id<'repo> {
    type Target = oid;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'repo> Id<'repo> {
    pub(crate) fn from_id(id: impl Into<ObjectId>, repo: &'repo crate::Repository) -> Self {
        Id { inner: id.into(), repo }
    }

    /// Turn this instance into its bare [ObjectId].
    pub fn detach(self) -> ObjectId {
        self.inner
    }
}

impl<'repo> Id<'repo> {
    /// Obtain a platform for traversing ancestors of this commit.
    ///
    pub fn ancestors(&self) -> revision::walk::Platform<'repo> {
        revision::walk::Platform::new(Some(self.inner), self.repo)
    }
}

mod impls {
    use std::{cmp::Ordering, hash::Hasher};

    use gix_hash::{oid, ObjectId};

    use crate::{Id, Object, ObjectDetached};

    // Eq, Hash, Ord, PartialOrd,

    impl<'a> std::hash::Hash for Id<'a> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.inner.hash(state)
        }
    }

    impl<'a> PartialOrd<Id<'a>> for Id<'a> {
        fn partial_cmp(&self, other: &Id<'a>) -> Option<Ordering> {
            self.inner.partial_cmp(&other.inner)
        }
    }

    impl<'repo> PartialEq<Id<'repo>> for Id<'repo> {
        fn eq(&self, other: &Id<'repo>) -> bool {
            self.inner == other.inner
        }
    }

    impl<'repo> PartialEq<ObjectId> for Id<'repo> {
        fn eq(&self, other: &ObjectId) -> bool {
            &self.inner == other
        }
    }

    impl<'repo> PartialEq<Id<'repo>> for ObjectId {
        fn eq(&self, other: &Id<'repo>) -> bool {
            self == &other.inner
        }
    }

    impl<'repo> PartialEq<oid> for Id<'repo> {
        fn eq(&self, other: &oid) -> bool {
            self.inner == other
        }
    }

    impl<'repo> PartialEq<Object<'repo>> for Id<'repo> {
        fn eq(&self, other: &Object<'repo>) -> bool {
            self.inner == other.id
        }
    }

    impl<'repo> PartialEq<ObjectDetached> for Id<'repo> {
        fn eq(&self, other: &ObjectDetached) -> bool {
            self.inner == other.id
        }
    }

    impl<'repo> std::fmt::Debug for Id<'repo> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.inner.fmt(f)
        }
    }

    impl<'repo> std::fmt::Display for Id<'repo> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.inner.fmt(f)
        }
    }

    impl<'repo> AsRef<oid> for Id<'repo> {
        fn as_ref(&self) -> &oid {
            &self.inner
        }
    }

    impl<'repo> From<Id<'repo>> for ObjectId {
        fn from(v: Id<'repo>) -> Self {
            v.inner
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_oid() {
        let actual = std::mem::size_of::<Id<'_>>();
        let ceiling = 32;
        assert!(
            actual <= ceiling,
            "size of oid shouldn't change without notice: {actual} <= {ceiling}"
        )
    }
}
