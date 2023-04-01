use gix_hash::ObjectId;
use gix_odb::FindExt;

use crate::{revision, Repository};

/// The error returned by [`Platform::all()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    AncestorIter(#[from] gix_traverse::commit::ancestors::Error),
    #[error(transparent)]
    ShallowCommits(#[from] crate::shallow::open::Error),
}

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
    /// For each commit, let `filter` return `true` if it and its parents should be included in the traversal, or `false`
    /// if the traversal should exclude it and its ancestry entirely.
    ///
    /// If `filter` is None, no pruning of the graph will be performed which is the default.
    pub fn selected(
        self,
        mut filter: impl FnMut(&gix_hash::oid) -> bool + 'repo,
    ) -> Result<revision::Walk<'repo>, Error> {
        let Platform {
            repo,
            tips,
            sorting,
            parents,
        } = self;
        Ok(revision::Walk {
            repo,
            inner: Box::new(
                gix_traverse::commit::Ancestors::filtered(
                    tips,
                    gix_traverse::commit::ancestors::State::default(),
                    move |oid, buf| repo.objects.find_commit_iter(oid, buf),
                    {
                        let shallow_commits = repo.shallow_commits()?;
                        let mut grafted_parents_to_skip = Vec::new();
                        let mut buf = Vec::new();
                        move |id| {
                            if !filter(id) {
                                return false;
                            }
                            match shallow_commits.as_ref() {
                                Some(commits) => {
                                    let id = id.to_owned();
                                    if let Ok(idx) = grafted_parents_to_skip.binary_search(&id) {
                                        grafted_parents_to_skip.remove(idx);
                                        return false;
                                    };
                                    if commits.binary_search(&id).is_ok() {
                                        if let Ok(commit) = repo.objects.find_commit_iter(id, &mut buf) {
                                            grafted_parents_to_skip.extend(commit.parent_ids());
                                            grafted_parents_to_skip.sort();
                                        }
                                    };
                                    true
                                }
                                None => true,
                            }
                        }
                    },
                )
                .sorting(sorting)?
                .parents(parents),
            ),
        })
    }
    /// Return an iterator to traverse all commits reachable as configured by the [Platform].
    ///
    /// # Performance
    ///
    /// It's highly recommended to set an [`object cache`][Repository::object_cache_size()] on the parent repo
    /// to greatly speed up performance if the returned id is supposed to be looked up right after.
    pub fn all(self) -> Result<revision::Walk<'repo>, Error> {
        self.selected(|_| true)
    }
}

pub(crate) mod iter {
    use crate::{ext::ObjectIdExt, Id};

    /// The iterator returned by [`crate::revision::walk::Platform::all()`].
    pub struct Walk<'repo> {
        pub(crate) repo: &'repo crate::Repository,
        pub(crate) inner:
            Box<dyn Iterator<Item = Result<gix_hash::ObjectId, gix_traverse::commit::ancestors::Error>> + 'repo>,
    }

    impl<'repo> Iterator for Walk<'repo> {
        type Item = Result<Id<'repo>, gix_traverse::commit::ancestors::Error>;

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next().map(|res| res.map(|id| id.attach(self.repo)))
        }
    }
}
