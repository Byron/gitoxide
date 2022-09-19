use git_hash::ObjectId;
use git_object::{bstr::BStr, TreeRefIter};

use crate::{object::find, Id, Tree};

/// Initialization
impl<'repo> Tree<'repo> {
    /// Obtain a tree instance by handing in all components that it is made up of.
    pub fn from_data(id: impl Into<ObjectId>, data: Vec<u8>, repo: &'repo crate::Repository) -> Self {
        Tree {
            id: id.into(),
            data,
            repo,
        }
    }
}

/// Access
impl<'repo> Tree<'repo> {
    /// Return this tree's identifier.
    pub fn id(&self) -> Id<'repo> {
        Id::from_id(self.id, self.repo)
    }

    // TODO: tests.
    /// Follow a sequence of `path` components starting from this instance, and look them up one by one until the last component
    /// is looked up and its tree entry is returned.
    ///
    /// # Performance Notes
    ///
    /// Searching tree entries is currently done in sequence, which allows to the search to be allocation free. It would be possible
    /// to re-use a vector and use a binary search instead, which might be able to improve performance over all.
    /// However, a benchmark should be created first to have some data and see which trade-off to choose here.
    pub fn lookup_path<I, P>(mut self, path: I) -> Result<Option<git_object::tree::Entry>, find::existing::Error>
    where
        I: IntoIterator<Item = P>,
        P: PartialEq<BStr>,
    {
        let mut path = path.into_iter().peekable();
        while let Some(component) = path.next() {
            match TreeRefIter::from_bytes(&self.data)
                .filter_map(Result::ok)
                .find(|entry| component.eq(entry.filename))
            {
                Some(entry) => {
                    if path.peek().is_none() {
                        return Ok(Some(entry.into()));
                    } else {
                        let next_id = entry.oid.to_owned();
                        let repo = self.repo;
                        drop(self);
                        self = match repo.find_object(next_id)?.try_into_tree() {
                            Ok(tree) => tree,
                            Err(_) => return Ok(None),
                        };
                    }
                }
                None => return Ok(None),
            }
        }
        Ok(None)
    }
}

///
pub mod diff;

///
pub mod traverse;

///
mod iter;
pub use iter::EntryRef;

impl<'r> std::fmt::Debug for Tree<'r> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tree({})", self.id)
    }
}
