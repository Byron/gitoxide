use gix_object::TreeRefIter;
use gix_odb::FindExt;

use super::{change, Action, Change, Platform};
use crate::{
    bstr::BStr,
    ext::ObjectIdExt,
    object::tree::{
        diff,
        diff::{rewrites, tracked},
    },
    Repository, Tree,
};

/// The error return by methods on the [diff platform][Platform].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Diff(#[from] gix_diff::tree::changes::Error),
    #[error("The user-provided callback failed")]
    ForEach(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("Could not find blob for similarity checking")]
    FindExistingBlob(#[from] crate::object::find::existing::Error),
    #[error("Could not configure diff algorithm prior to checking similarity")]
    ConfigureDiffAlgorithm(#[from] crate::config::diff::algorithm::Error),
    #[error("Could not traverse tree to obtain possible sources for copies")]
    TraverseTreeForExhaustiveCopyDetection(#[from] gix_traverse::tree::breadthfirst::Error),
}

///
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Outcome {
    /// Available only if [rewrite-tracking was enabled][Platform::track_rewrites()].
    pub rewrites: Option<rewrites::Outcome>,
}

/// Add the item to compare to.
impl<'a, 'old> Platform<'a, 'old> {
    /// Call `for_each` repeatedly with all changes that are needed to convert the source of the diff to the tree to `other`.
    ///
    /// `other` could also be created with the [`empty_tree()`][crate::Repository::empty_tree()] method to handle the first commit
    /// in a repository - it doesn't have a parent, equivalent to compare 'nothing' to something.
    pub fn for_each_to_obtain_tree<'new, E>(
        &mut self,
        other: &Tree<'new>,
        for_each: impl FnMut(Change<'_, 'old, 'new>) -> Result<Action, E>,
    ) -> Result<Outcome, Error>
    where
        E: std::error::Error + Sync + Send + 'static,
    {
        let repo = self.lhs.repo;
        let mut delegate = Delegate {
            src_tree: self.lhs,
            other_repo: other.repo,
            recorder: gix_diff::tree::Recorder::default().track_location(self.tracking),
            visit: for_each,
            tracked: self.rewrites.map(|r| tracked::State::new(r, self.tracking)),
            err: None,
        };
        match gix_diff::tree::Changes::from(TreeRefIter::from_bytes(&self.lhs.data)).needed_to_obtain(
            TreeRefIter::from_bytes(&other.data),
            &mut self.state,
            |oid, buf| repo.objects.find_tree_iter(oid, buf),
            &mut delegate,
        ) {
            Ok(()) => {
                let outcome = Outcome {
                    rewrites: delegate.process_tracked_changes()?,
                };
                match delegate.err {
                    Some(err) => Err(Error::ForEach(Box::new(err))),
                    None => Ok(outcome),
                }
            }
            Err(gix_diff::tree::changes::Error::Cancelled) => delegate
                .err
                .map_or(Err(Error::Diff(gix_diff::tree::changes::Error::Cancelled)), |err| {
                    Err(Error::ForEach(Box::new(err)))
                }),
            Err(err) => Err(err.into()),
        }
    }
}

struct Delegate<'a, 'old, 'new, VisitFn, E> {
    src_tree: &'a Tree<'old>,
    other_repo: &'new Repository,
    recorder: gix_diff::tree::Recorder,
    visit: VisitFn,
    tracked: Option<tracked::State>,
    err: Option<E>,
}

impl<'a, 'old, 'new, VisitFn, E> Delegate<'a, 'old, 'new, VisitFn, E>
where
    VisitFn: for<'delegate> FnMut(Change<'delegate, 'old, 'new>) -> Result<Action, E>,
    E: std::error::Error + Sync + Send + 'static,
{
    /// Call `visit` on an attached version of `change`.
    fn emit_change(
        change: gix_diff::tree::visit::Change,
        location: &BStr,
        visit: &mut VisitFn,
        repo: &'old Repository,
        other_repo: &'new Repository,
        stored_err: &mut Option<E>,
    ) -> gix_diff::tree::visit::Action {
        use gix_diff::tree::visit::Change::*;
        let event = match change {
            Addition { entry_mode, oid } => change::Event::Addition {
                entry_mode,
                id: oid.attach(other_repo),
            },
            Deletion { entry_mode, oid } => change::Event::Deletion {
                entry_mode,
                id: oid.attach(repo),
            },
            Modification {
                previous_entry_mode,
                previous_oid,
                entry_mode,
                oid,
            } => change::Event::Modification {
                previous_entry_mode,
                entry_mode,
                previous_id: previous_oid.attach(repo),
                id: oid.attach(other_repo),
            },
        };
        match visit(Change { event, location }) {
            Ok(Action::Cancel) => gix_diff::tree::visit::Action::Cancel,
            Ok(Action::Continue) => gix_diff::tree::visit::Action::Continue,
            Err(err) => {
                *stored_err = Some(err);
                gix_diff::tree::visit::Action::Cancel
            }
        }
    }

    fn process_tracked_changes(&mut self) -> Result<Option<rewrites::Outcome>, Error> {
        let tracked = match self.tracked.as_mut() {
            Some(t) => t,
            None => return Ok(None),
        };

        let outcome = tracked.emit(
            |dest, source| match source {
                Some(source) => {
                    let (oid, mode) = dest.change.oid_and_entry_mode();
                    let change = diff::Change {
                        location: dest.location,
                        event: diff::change::Event::Rewrite {
                            source_location: source.location,
                            source_entry_mode: source.mode,
                            source_id: source.id.attach(self.src_tree.repo),
                            entry_mode: mode,
                            id: oid.to_owned().attach(self.other_repo),
                            diff: source.diff,
                            copy: match source.kind {
                                tracked::visit::Kind::RenameTarget => false,
                                tracked::visit::Kind::CopyDestination => true,
                            },
                        },
                    };
                    match (self.visit)(change) {
                        Ok(Action::Cancel) => gix_diff::tree::visit::Action::Cancel,
                        Ok(Action::Continue) => gix_diff::tree::visit::Action::Continue,
                        Err(err) => {
                            self.err = Some(err);
                            gix_diff::tree::visit::Action::Cancel
                        }
                    }
                }
                None => Self::emit_change(
                    dest.change,
                    dest.location,
                    &mut self.visit,
                    self.src_tree.repo,
                    self.other_repo,
                    &mut self.err,
                ),
            },
            self.src_tree,
        )?;
        Ok(Some(outcome))
    }
}

impl<'a, 'old, 'new, VisitFn, E> gix_diff::tree::Visit for Delegate<'a, 'old, 'new, VisitFn, E>
where
    VisitFn: for<'delegate> FnMut(Change<'delegate, 'old, 'new>) -> Result<Action, E>,
    E: std::error::Error + Sync + Send + 'static,
{
    fn pop_front_tracked_path_and_set_current(&mut self) {
        self.recorder.pop_front_tracked_path_and_set_current()
    }

    fn push_back_tracked_path_component(&mut self, component: &BStr) {
        self.recorder.push_back_tracked_path_component(component)
    }

    fn push_path_component(&mut self, component: &BStr) {
        self.recorder.push_path_component(component)
    }

    fn pop_path_component(&mut self) {
        self.recorder.pop_path_component()
    }

    fn visit(&mut self, change: gix_diff::tree::visit::Change) -> gix_diff::tree::visit::Action {
        match self.tracked.as_mut() {
            Some(tracked) => tracked.try_push_change(change, self.recorder.path()).map_or(
                gix_diff::tree::visit::Action::Continue,
                |change| {
                    Self::emit_change(
                        change,
                        self.recorder.path(),
                        &mut self.visit,
                        self.src_tree.repo,
                        self.other_repo,
                        &mut self.err,
                    )
                },
            ),
            None => Self::emit_change(
                change,
                self.recorder.path(),
                &mut self.visit,
                self.src_tree.repo,
                self.other_repo,
                &mut self.err,
            ),
        }
    }
}
