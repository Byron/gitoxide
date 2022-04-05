//!
use std::ops::Deref;

use git_hash::{oid, ObjectId};

use crate::{object::find, Id, Object};

/// An [object id][ObjectId] infused with `Easy`.
impl<'repo> Id<'repo> {
    /// Find the [`Object`] associated with this object id, and consider it an error if it doesn't exist.
    ///
    /// # Note
    ///
    /// There can only be one `ObjectRef` per `Easy`. To increase that limit, clone the `Easy`.
    pub fn object(&self) -> Result<Object<'repo>, find::existing::OdbError> {
        self.repo.find_object(self.inner)
    }

    /// Try to find the [`Object`] associated with this object id, and return `None` if it's not available locally.
    ///
    /// # Note
    ///
    /// There can only be one `ObjectRef` per `Easy`. To increase that limit, clone the `Easy`.
    pub fn try_object(&self) -> Result<Option<Object<'repo>>, find::OdbError> {
        self.repo.try_find_object(self.inner)
    }

    /// Turn this object id into a shortened id with a length in hex as configured by `core.abbrev`.
    pub fn shorten(&self) -> Result<git_hash::Prefix, shorten::Error> {
        let hex_len = self.repo.config.hex_len.unwrap_or(
            // TODO: obtain calculated value
            7,
        );
        // NOTE: this error shouldn't be possible
        let prefix = git_odb::find::PotentialPrefix::new(self.inner, hex_len)
            .expect("BUG: internal hex-len must always be valid");
        Ok(self
            .repo
            .objects
            .disambiguate_prefix(prefix)
            .map_err(crate::object::find::existing::OdbError::Find)?
            .ok_or(crate::object::find::existing::OdbError::NotFound { oid: self.inner })?)
    }
}

///
pub mod shorten {
    /// Returned by [`Id::prefix()`][super::Id::shorten()].
    pub type Error = crate::object::find::existing::OdbError;
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

/// A platform to traverse commit ancestors, also referred to as commit history.
pub struct Ancestors<'repo> {
    repo: &'repo crate::Repository,
    tips: Box<dyn Iterator<Item = ObjectId>>,
    sorting: git_traverse::commit::Sorting,
    parents: git_traverse::commit::Parents,
}

///
pub mod ancestors {
    use git_odb::FindExt;

    use crate::{ext::ObjectIdExt, id::Ancestors, Id};

    impl<'repo> Id<'repo> {
        /// Obtain a platform for traversing ancestors of this commit.
        pub fn ancestors(&self) -> Ancestors<'repo> {
            Ancestors {
                repo: self.repo,
                tips: Box::new(Some(self.inner).into_iter()),
                sorting: Default::default(),
                parents: Default::default(),
            }
        }
    }

    impl<'repo> Ancestors<'repo> {
        /// Set the sort mode for commits to the given value. The default is to order by topology.
        pub fn sorting(mut self, sorting: git_traverse::commit::Sorting) -> Self {
            self.sorting = sorting;
            self
        }

        /// Only traverse the first parent of the commit graph.
        pub fn first_parent_only(mut self) -> Self {
            self.parents = git_traverse::commit::Parents::First;
            self
        }

        /// Return an iterator to traverse all commits in the history of the commit the parent [Id] is pointing to.
        pub fn all(&mut self) -> Result<Iter<'repo>, git_traverse::commit::ancestors::Error> {
            let tips = std::mem::replace(&mut self.tips, Box::new(None.into_iter()));
            let parents = self.parents;
            let sorting = self.sorting;
            let repo = self.repo;
            Ok(Iter {
                repo,
                inner: Box::new(
                    git_traverse::commit::Ancestors::new(
                        tips,
                        git_traverse::commit::ancestors::State::default(),
                        move |oid, buf| repo.objects.find_commit_iter(oid, buf),
                    )
                    .sorting(sorting)?
                    .parents(parents),
                ),
                is_shallow: None,
                error_on_missing_commit: false,
            })
        }
    }

    /// The iterator returned by [`Ancestors::all()`].
    pub struct Iter<'repo> {
        repo: &'repo crate::Repository,
        inner: Box<dyn Iterator<Item = Result<git_hash::ObjectId, git_traverse::commit::ancestors::Error>> + 'repo>,
        error_on_missing_commit: bool,
        // TODO: tests
        /// After iteration this flag is true if the iteration was stopped prematurely due to missing parent commits.
        /// Note that this flag won't be `Some` if any iteration error occours, which is the case if
        /// [`error_on_missing_commit()`][Iter::error_on_missing_commit()] was called.
        ///
        /// This happens if a repository is a shallow clone.
        /// Note that this value is `None` as long as the iteration isn't complete.
        pub is_shallow: Option<bool>,
    }

    impl<'repo> Iter<'repo> {
        // TODO: tests
        /// Once invoked, the iteration will return an error if a commit cannot be found in the object database. This typicall happens
        /// when operating on a shallow clone and thus is non-critical by default.
        ///
        /// Check the [`is_shallow`][Iter::is_shallow] field once the iteration ended otherwise to learn if a shallow commit graph
        /// was encountered.
        pub fn error_on_missing_commit(mut self) -> Self {
            self.error_on_missing_commit = true;
            self
        }
    }

    impl<'repo> Iterator for Iter<'repo> {
        type Item = Result<Id<'repo>, git_traverse::commit::ancestors::Error>;

        fn next(&mut self) -> Option<Self::Item> {
            match self.inner.next() {
                None => {
                    self.is_shallow = Some(false);
                    None
                }
                Some(Ok(oid)) => Some(Ok(oid.attach(self.repo))),
                Some(Err(err @ git_traverse::commit::ancestors::Error::FindExisting { .. })) => {
                    if self.error_on_missing_commit {
                        Some(Err(err))
                    } else {
                        self.is_shallow = Some(true);
                        None
                    }
                }
                Some(Err(err)) => Some(Err(err)),
            }
        }
    }
}

mod impls {
    use std::{cmp::Ordering, hash::Hasher};

    use git_hash::{oid, ObjectId};

    use crate::{DetachedObject, Id, Object};

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

    impl<'repo> PartialEq<DetachedObject> for Id<'repo> {
        fn eq(&self, other: &DetachedObject) -> bool {
            self.inner == other.id
        }
    }

    impl<'repo> std::fmt::Debug for Id<'repo> {
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
        assert_eq!(
            std::mem::size_of::<Id<'_>>(),
            32,
            "size of oid shouldn't change without notice"
        )
    }
}
