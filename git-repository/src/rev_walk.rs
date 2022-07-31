use git_odb::FindExt;

use crate::{ext::ObjectIdExt, Id, RevWalk};

/// Create-time builder methods
impl<'repo> RevWalk<'repo> {
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
}

/// Produce the iterator
impl<'repo> RevWalk<'repo> {
    /// Return an iterator to traverse all commits in the history of the commit the parent [Id] is pointing to.
    pub fn all(self) -> Result<Iter<'repo>, git_traverse::commit::ancestors::Error> {
        let RevWalk {
            repo,
            tips,
            sorting,
            parents,
        } = self;
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

/// The iterator returned by [`RevWalk::all()`].
pub struct Iter<'repo> {
    repo: &'repo crate::Repository,
    inner: Box<dyn Iterator<Item = Result<git_hash::ObjectId, git_traverse::commit::ancestors::Error>> + 'repo>,
    error_on_missing_commit: bool,
    // TODO: tests
    /// After iteration this flag is true if the iteration was stopped prematurely due to missing parent commits.
    /// Note that this flag won't be `Some` if any iteration error occurs, which is the case if
    /// [`error_on_missing_commit()`][Iter::error_on_missing_commit()] was called.
    ///
    /// This happens if a repository is a shallow clone.
    /// Note that this value is `None` as long as the iteration isn't complete.
    pub is_shallow: Option<bool>,
}

impl<'repo> Iter<'repo> {
    // TODO: tests
    /// Once invoked, the iteration will return an error if a commit cannot be found in the object database. This typically happens
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
