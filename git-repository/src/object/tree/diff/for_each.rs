use super::{change, Action, Change, Platform, Tracking};
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

/// Add the item to compare to.
impl<'a, 'old> Platform<'a, 'old> {
    /// Call `for_each` repeatedly with all changes that are needed to convert the source of the diff to the tree to `other`.
    pub fn for_each_to_obtain_tree<'new, E>(
        &mut self,
        other: &Tree<'new>,
        for_each: impl FnMut(Change<'_, 'old, 'new>) -> Result<Action, E>,
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

struct Delegate<'old, 'new, VisitFn, E> {
    repo: &'old Repository,
    other_repo: &'new Repository,
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

impl<'old, 'new, VisitFn, E> git_diff::tree::Visit for Delegate<'old, 'new, VisitFn, E>
where
    VisitFn: for<'delegate> FnMut(Change<'delegate, 'old, 'new>) -> Result<Action, E>,
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
