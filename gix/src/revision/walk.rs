use gix_hash::ObjectId;
use gix_odb::FindExt;

use crate::{revision, Repository};

/// A platform to traverse the revision graph by adding starting points as well as points which shouldn't be crossed,
/// returned by [`Repository::rev_walk()`].
pub struct Platform<'repo> {
    pub(crate) repo: &'repo Repository,
    pub(crate) tips: Vec<ObjectId>,
    pub(crate) sorting: gix_traverse::commit::Sorting,
    pub(crate) parents: gix_traverse::commit::Parents,
}

impl<'repo> Platform<'repo> {
    pub(crate) fn new(tips: impl IntoIterator<Item = impl Into<ObjectId>>, repo: &'repo Repository) -> Self {
        revision::walk::Platform {
            repo,
            tips: tips.into_iter().map(Into::into).collect(),
            sorting: Default::default(),
            parents: Default::default(),
        }
    }
}

/// Create-time builder methods
impl<'repo> Platform<'repo> {
    /// Set the sort mode for commits to the given value. The default is to order by topology.
    pub fn sorting(mut self, sorting: gix_traverse::commit::Sorting) -> Self {
        self.sorting = sorting;
        self
    }

    /// Only traverse the first parent of the commit graph.
    pub fn first_parent_only(mut self) -> Self {
        self.parents = gix_traverse::commit::Parents::First;
        self
    }
}

/// Produce the iterator
impl<'repo> Platform<'repo> {
    /// Return an iterator to traverse all commits reachable as configured by the [Platform].
    ///
    /// # Performance
    ///
    /// It's highly recommended to set an [`object cache`][Repository::object_cache_size()] on the parent repo
    /// to greatly speed up performance if the returned id is supposed to be looked up right after.
    pub fn all(self) -> Result<revision::Walk<'repo>, gix_traverse::commit::ancestors::Error> {
        let Platform {
            repo,
            tips,
            sorting,
            parents,
        } = self;
        Ok(revision::Walk {
            repo,
            inner: Box::new(
                gix_traverse::commit::Ancestors::new(
                    tips,
                    gix_traverse::commit::ancestors::State::default(),
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

pub(crate) mod iter {
    use crate::{ext::ObjectIdExt, Id};

    /// The iterator returned by [`crate::revision::walk::Platform::all()`].
    pub struct Walk<'repo> {
        pub(crate) repo: &'repo crate::Repository,
        pub(crate) inner:
            Box<dyn Iterator<Item = Result<gix_hash::ObjectId, gix_traverse::commit::ancestors::Error>> + 'repo>,
        pub(crate) error_on_missing_commit: bool,
        // TODO: tests
        /// After iteration this flag is true if the iteration was stopped prematurely due to missing parent commits.
        /// Note that this flag won't be `Some` if any iteration error occurs, which is the case if
        /// [`error_on_missing_commit()`][Walk::error_on_missing_commit()] was called.
        ///
        /// This happens if a repository is a shallow clone.
        /// Note that this value is `None` as long as the iteration isn't complete.
        pub is_shallow: Option<bool>,
    }

    impl<'repo> Walk<'repo> {
        // TODO: tests
        /// Once invoked, the iteration will return an error if a commit cannot be found in the object database. This typically happens
        /// when operating on a shallow clone and thus is non-critical by default.
        ///
        /// Check the [`is_shallow`][Walk::is_shallow] field once the iteration ended otherwise to learn if a shallow commit graph
        /// was encountered.
        pub fn error_on_missing_commit(mut self) -> Self {
            self.error_on_missing_commit = true;
            self
        }
    }

    impl<'repo> Iterator for Walk<'repo> {
        type Item = Result<Id<'repo>, gix_traverse::commit::ancestors::Error>;

        fn next(&mut self) -> Option<Self::Item> {
            match self.inner.next() {
                None => {
                    self.is_shallow = Some(false);
                    None
                }
                Some(Ok(oid)) => Some(Ok(oid.attach(self.repo))),
                Some(Err(err @ gix_traverse::commit::ancestors::Error::FindExisting { .. })) => {
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
