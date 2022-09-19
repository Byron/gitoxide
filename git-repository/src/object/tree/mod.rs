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

#[allow(missing_docs)]
///
pub mod diff {
    use crate::bstr::{BStr, BString};
    use crate::ext::ObjectIdExt;
    use crate::{Id, Repository, Tree};
    use git_object::TreeRefIter;
    use git_odb::FindExt;

    /// The error return by methods on the [diff platform][super::Platform].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Diff(#[from] git_diff::tree::changes::Error),
        #[error("The user-provided callback failed")]
        ForEach(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
    }

    /// Represents any possible change in order to turn one tree into another.
    #[derive(Debug, Clone, Copy)]
    pub struct Change<'a, 'repo, 'other_repo> {
        /// The location of the file or directory described by `event`, if tracking was enabled.
        ///
        /// Otherwise this value is always an empty path.
        pub location: &'a BStr,
        /// The diff event itself to provide information about what would need to change.
        pub event: Event<'repo, 'other_repo>,
    }

    /// An event emitted when finding differences between two trees.
    #[derive(Debug, Clone, Copy)]
    pub enum Event<'repo, 'other_repo> {
        /// An entry was added, like the addition of a file or directory.
        Addition {
            /// The mode of the added entry.
            entry_mode: git_object::tree::EntryMode,
            /// The object id of the added entry.
            id: Id<'other_repo>,
        },
        /// An entry was deleted, like the deletion of a file or directory.
        Deletion {
            /// The mode of the deleted entry.
            entry_mode: git_object::tree::EntryMode,
            /// The object id of the deleted entry.
            id: Id<'repo>,
        },
        /// An entry was modified, e.g. changing the contents of a file adjusts its object id and turning
        /// a file into a symbolic link adjusts its mode.
        Modification {
            /// The mode of the entry before the modification.
            previous_entry_mode: git_object::tree::EntryMode,
            /// The object id of the entry before the modification.
            previous_id: Id<'repo>,

            /// The mode of the entry after the modification.
            entry_mode: git_object::tree::EntryMode,
            /// The object id after the modification.
            id: Id<'other_repo>,
        },
    }

    /// Diffing
    impl<'repo> Tree<'repo> {
        /// Return a platform to see the changes needed to create other trees, for instance.
        pub fn changes<'other_repo, 'a>(&'a self) -> Platform<'a, 'repo> {
            Platform {
                state: Default::default(),
                lhs: self,
            }
        }
    }

    pub struct Platform<'a, 'repo> {
        state: git_diff::tree::State,
        lhs: &'a Tree<'repo>,
    }

    /// Add the item to compare to.
    impl<'a, 'repo> Platform<'a, 'repo> {
        /// Call `for_each` repeatedly with all changes that are needed to convert the source of the diff to the tree to `other`.
        pub fn for_each_to_obtain_tree<'other_repo, E>(
            &mut self,
            other: &Tree<'other_repo>,
            for_each: impl FnMut(Change<'_, 'repo, 'other_repo>) -> Result<git_diff::tree::visit::Action, E>,
        ) -> Result<(), Error>
        where
            E: std::error::Error + Sync + Send + 'static,
        {
            let repo = self.lhs.repo;
            let mut delegate = Delegate {
                repo: self.lhs.repo,
                other_repo: other.repo,
                location: BString::default(),
                visit: for_each,
                err: None,
            };
            git_diff::tree::Changes::from(TreeRefIter::from_bytes(&self.lhs.data)).needed_to_obtain(
                TreeRefIter::from_bytes(&other.data),
                &mut self.state,
                |oid, buf| repo.objects.find_tree_iter(oid, buf),
                &mut delegate,
            )?;
            match delegate.err {
                Some(err) => Err(Error::ForEach(Box::new(err))),
                None => Ok(()),
            }
        }
    }

    struct Delegate<'repo, 'other_repo, VisitFn, E> {
        repo: &'repo Repository,
        other_repo: &'other_repo Repository,
        location: BString,
        visit: VisitFn,
        err: Option<E>,
    }

    impl<'repo, 'other_repo, VisitFn, E> git_diff::tree::Visit for Delegate<'repo, 'other_repo, VisitFn, E>
    where
        VisitFn:
            for<'delegate> FnMut(Change<'delegate, 'repo, 'other_repo>) -> Result<git_diff::tree::visit::Action, E>,
        E: std::error::Error + Sync + Send + 'static,
    {
        fn pop_front_tracked_path_and_set_current(&mut self) {}

        fn push_back_tracked_path_component(&mut self, _component: &BStr) {
            {}
        }

        fn push_path_component(&mut self, _component: &BStr) {}

        fn pop_path_component(&mut self) {}

        fn visit(&mut self, change: git_diff::tree::visit::Change) -> git_diff::tree::visit::Action {
            use git_diff::tree::visit::Change::*;
            let event = match change {
                Addition { entry_mode, oid } => Event::Addition {
                    entry_mode,
                    id: oid.attach(self.other_repo),
                },
                Deletion { entry_mode, oid } => Event::Deletion {
                    entry_mode,
                    id: oid.attach(self.repo),
                },
                Modification {
                    previous_entry_mode,
                    previous_oid,
                    entry_mode,
                    oid,
                } => Event::Modification {
                    previous_entry_mode,
                    entry_mode,
                    previous_id: previous_oid.attach(self.repo),
                    id: oid.attach(self.other_repo),
                },
            };
            match (self.visit)(Change {
                event,
                location: self.location.as_ref(),
            }) {
                Ok(action) => action,
                Err(err) => {
                    self.err = Some(err);
                    git_diff::tree::visit::Action::Cancel
                }
            }
        }
    }
}

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
