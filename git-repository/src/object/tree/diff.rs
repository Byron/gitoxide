use crate::bstr::{BStr, BString, ByteSlice, ByteVec};
use crate::ext::ObjectIdExt;
use crate::{Repository, Tree};
use git_object::TreeRefIter;
use git_odb::FindExt;
use std::collections::VecDeque;

/// The error return by methods on the [diff platform][Platform].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Diff(#[from] git_diff::tree::changes::Error),
    #[error("The user-provided callback failed")]
    ForEach(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
}

/// Returned by the `for_each` function to control flow.
#[derive(Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum Action {
    /// Continue the traversal of changes.
    Continue,
    /// Stop the traversal of changes and stop calling this function.
    Cancel,
}

impl Default for Action {
    fn default() -> Self {
        Action::Continue
    }
}

/// Represents any possible change in order to turn one tree into another.
#[derive(Debug, Clone, Copy)]
pub struct Change<'a, 'repo, 'other_repo> {
    /// The location of the file or directory described by `event`, if tracking was enabled.
    ///
    /// Otherwise this value is always an empty path.
    pub location: &'a BStr,
    /// The diff event itself to provide information about what would need to change.
    pub event: change::Event<'repo, 'other_repo>,
}

///
pub mod change {
    use crate::Id;

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
}

/// Diffing
impl<'repo> Tree<'repo> {
    /// Return a platform to see the changes needed to create other trees, for instance.
    ///
    /// # Performance
    ///
    /// It's highly recommended to set an object cache to avoid extracting the same object multiple times.
    pub fn changes<'a>(&'a self) -> Platform<'a, 'repo> {
        Platform {
            state: Default::default(),
            lhs: self,
            tracking: None,
        }
    }
}

/// The diffing platform returned by [`Tree::changes()`].
#[derive(Clone)]
pub struct Platform<'a, 'repo> {
    state: git_diff::tree::State,
    lhs: &'a Tree<'repo>,
    tracking: Option<Tracking>,
}

#[derive(Clone, Copy)]
enum Tracking {
    FileName,
    Path,
}

/// Configuration
impl<'a, 'repo> Platform<'a, 'repo> {
    /// Keep track of file-names, which makes the [`location`][Change::location] field usable with the filename of the changed item.
    pub fn track_filename(&mut self) -> &mut Self {
        self.tracking = Some(Tracking::FileName);
        self
    }

    /// Keep track of the entire path of a change, relative to the repository.
    ///
    /// This makes the [`location`][Change::location] field usable.
    pub fn track_path(&mut self) -> &mut Self {
        self.tracking = Some(Tracking::Path);
        self
    }
}

/// Add the item to compare to.
impl<'a, 'repo> Platform<'a, 'repo> {
    /// Call `for_each` repeatedly with all changes that are needed to convert the source of the diff to the tree to `other`.
    pub fn for_each_to_obtain_tree<'other_repo, E>(
        &mut self,
        other: &Tree<'other_repo>,
        for_each: impl FnMut(Change<'_, 'repo, 'other_repo>) -> Result<Action, E>,
    ) -> Result<(), Error>
    where
        E: std::error::Error + Sync + Send + 'static,
    {
        let repo = self.lhs.repo;
        let mut delegate = Delegate {
            repo: self.lhs.repo,
            other_repo: other.repo,
            tracking: self.tracking,
            location: BString::default(),
            path_deque: Default::default(),
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
    tracking: Option<Tracking>,
    location: BString,
    path_deque: VecDeque<BString>,
    visit: VisitFn,
    err: Option<E>,
}

impl<A, B> Delegate<'_, '_, A, B> {
    fn pop_element(&mut self) {
        if let Some(pos) = self.location.rfind_byte(b'/') {
            self.location.resize(pos, 0);
        } else {
            self.location.clear();
        }
    }

    fn push_element(&mut self, name: &BStr) {
        if !self.location.is_empty() {
            self.location.push(b'/');
        }
        self.location.push_str(name);
    }
}

impl<'repo, 'other_repo, VisitFn, E> git_diff::tree::Visit for Delegate<'repo, 'other_repo, VisitFn, E>
where
    VisitFn: for<'delegate> FnMut(Change<'delegate, 'repo, 'other_repo>) -> Result<Action, E>,
    E: std::error::Error + Sync + Send + 'static,
{
    fn pop_front_tracked_path_and_set_current(&mut self) {
        if let Some(Tracking::Path) = self.tracking {
            self.location = self
                .path_deque
                .pop_front()
                .expect("every call is matched with push_tracked_path_component");
        }
    }

    fn push_back_tracked_path_component(&mut self, component: &BStr) {
        if let Some(Tracking::Path) = self.tracking {
            self.push_element(component);
            self.path_deque.push_back(self.location.clone());
        }
    }

    fn push_path_component(&mut self, component: &BStr) {
        match self.tracking {
            Some(Tracking::FileName) => {
                self.location.clear();
                self.location.push_str(component);
            }
            Some(Tracking::Path) => {
                self.push_element(component);
            }
            None => {}
        }
    }

    fn pop_path_component(&mut self) {
        if let Some(Tracking::Path) = self.tracking {
            self.pop_element();
        }
    }

    fn visit(&mut self, change: git_diff::tree::visit::Change) -> git_diff::tree::visit::Action {
        use git_diff::tree::visit::Change::*;
        let event = match change {
            Addition { entry_mode, oid } => change::Event::Addition {
                entry_mode,
                id: oid.attach(self.other_repo),
            },
            Deletion { entry_mode, oid } => change::Event::Deletion {
                entry_mode,
                id: oid.attach(self.repo),
            },
            Modification {
                previous_entry_mode,
                previous_oid,
                entry_mode,
                oid,
            } => change::Event::Modification {
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
            Ok(Action::Cancel) => git_diff::tree::visit::Action::Cancel,
            Ok(Action::Continue) => git_diff::tree::visit::Action::Continue,
            Err(err) => {
                self.err = Some(err);
                git_diff::tree::visit::Action::Cancel
            }
        }
    }
}
