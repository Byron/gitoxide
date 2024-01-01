use gix_object::TreeRefIter;

use super::{change, Action, Change, Platform};
use crate::{
    bstr::BStr,
    diff::{rewrites, rewrites::tracker},
    ext::ObjectIdExt,
    object::tree::diff,
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
    #[error(transparent)]
    ResourceCache(#[from] crate::repository::diff::resource_cache::Error),
    #[error("Failure during rename tracking")]
    RenameTracking(#[from] tracker::emit::Error),
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
        E: Into<Box<dyn std::error::Error + Sync + Send + 'static>>,
    {
        self.for_each_to_obtain_tree_inner(other, for_each, None)
    }

    /// Like [`Self::for_each_to_obtain_tree()`], but with a reusable `resource_cache` which is used to perform
    /// diffs fast.
    ///
    /// Reusing it between multiple invocations saves a lot of IOps as it avoids the creation
    /// of a temporary `resource_cache` that triggers reading or checking for multiple gitattribute files.
    /// Note that it's recommended to call [`gix_diff::blob::Platform::clear_resource_cache()`] between the calls
    /// to avoid runaway memory usage, as the cache isn't limited.
    ///
    /// Note that to do rename tracking like `git` does, one has to configure the `resource_cache` with
    /// a conversion pipeline that uses [`gix_diff::blob::pipeline::Mode::ToGit`].
    pub fn for_each_to_obtain_tree_with_cache<'new, E>(
        &mut self,
        other: &Tree<'new>,
        resource_cache: &mut gix_diff::blob::Platform,
        for_each: impl FnMut(Change<'_, 'old, 'new>) -> Result<Action, E>,
    ) -> Result<Outcome, Error>
    where
        E: Into<Box<dyn std::error::Error + Sync + Send + 'static>>,
    {
        self.for_each_to_obtain_tree_inner(other, for_each, Some(resource_cache))
    }

    fn for_each_to_obtain_tree_inner<'new, E>(
        &mut self,
        other: &Tree<'new>,
        for_each: impl FnMut(Change<'_, 'old, 'new>) -> Result<Action, E>,
        resource_cache: Option<&mut gix_diff::blob::Platform>,
    ) -> Result<Outcome, Error>
    where
        E: Into<Box<dyn std::error::Error + Sync + Send + 'static>>,
    {
        let repo = self.lhs.repo;
        let mut delegate = Delegate {
            src_tree: self.lhs,
            other_repo: other.repo,
            recorder: gix_diff::tree::Recorder::default().track_location(self.tracking),
            visit: for_each,
            location: self.tracking,
            tracked: self.rewrites.map(rewrites::Tracker::new),
            err: None,
        };
        match gix_diff::tree::Changes::from(TreeRefIter::from_bytes(&self.lhs.data)).needed_to_obtain(
            TreeRefIter::from_bytes(&other.data),
            &mut self.state,
            &repo.objects,
            &mut delegate,
        ) {
            Ok(()) => {
                let outcome = Outcome {
                    rewrites: delegate.process_tracked_changes(resource_cache)?,
                };
                match delegate.err {
                    Some(err) => Err(Error::ForEach(err.into())),
                    None => Ok(outcome),
                }
            }
            Err(gix_diff::tree::changes::Error::Cancelled) => delegate
                .err
                .map_or(Err(Error::Diff(gix_diff::tree::changes::Error::Cancelled)), |err| {
                    Err(Error::ForEach(err.into()))
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
    tracked: Option<rewrites::Tracker<gix_diff::tree::visit::Change>>,
    location: Option<gix_diff::tree::recorder::Location>,
    err: Option<E>,
}

impl<'a, 'old, 'new, VisitFn, E> Delegate<'a, 'old, 'new, VisitFn, E>
where
    VisitFn: for<'delegate> FnMut(Change<'delegate, 'old, 'new>) -> Result<Action, E>,
    E: Into<Box<dyn std::error::Error + Sync + Send + 'static>>,
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

    fn process_tracked_changes(
        &mut self,
        diff_cache: Option<&mut gix_diff::blob::Platform>,
    ) -> Result<Option<rewrites::Outcome>, Error> {
        let tracked = match self.tracked.as_mut() {
            Some(t) => t,
            None => return Ok(None),
        };

        let repo = self.src_tree.repo;
        let mut storage;
        let diff_cache = match diff_cache {
            Some(cache) => cache,
            None => {
                storage = repo.diff_resource_cache(gix_diff::blob::pipeline::Mode::ToGit, Default::default())?;
                &mut storage
            }
        };

        let outcome = tracked.emit(
            |dest, source| match source {
                Some(source) => {
                    let (oid, mode) = dest.change.oid_and_entry_mode();
                    let change = diff::Change {
                        location: dest.location,
                        event: diff::change::Event::Rewrite {
                            source_location: source.location,
                            source_entry_mode: source.entry_mode,
                            source_id: source.id.attach(self.src_tree.repo),
                            entry_mode: mode,
                            id: oid.to_owned().attach(self.other_repo),
                            diff: source.diff,
                            copy: match source.kind {
                                tracker::visit::SourceKind::Rename => false,
                                tracker::visit::SourceKind::Copy => true,
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
            diff_cache,
            &self.src_tree.repo.objects,
            |push| {
                self.src_tree
                    .traverse()
                    .breadthfirst(&mut tree_to_changes::Delegate::new(push, self.location))
            },
        )?;
        Ok(Some(outcome))
    }
}

impl<'a, 'old, 'new, VisitFn, E> gix_diff::tree::Visit for Delegate<'a, 'old, 'new, VisitFn, E>
where
    VisitFn: for<'delegate> FnMut(Change<'delegate, 'old, 'new>) -> Result<Action, E>,
    E: Into<Box<dyn std::error::Error + Sync + Send + 'static>>,
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

mod tree_to_changes {
    use gix_diff::tree::visit::Change;
    use gix_object::tree::EntryRef;

    use crate::bstr::BStr;

    pub struct Delegate<'a> {
        push: &'a mut dyn FnMut(Change, &BStr),
        recorder: gix_traverse::tree::Recorder,
    }

    impl<'a> Delegate<'a> {
        pub fn new(
            push: &'a mut dyn FnMut(Change, &BStr),
            location: Option<gix_diff::tree::recorder::Location>,
        ) -> Self {
            let location = location.map(|t| match t {
                gix_diff::tree::recorder::Location::FileName => gix_traverse::tree::recorder::Location::FileName,
                gix_diff::tree::recorder::Location::Path => gix_traverse::tree::recorder::Location::Path,
            });
            Self {
                push,
                recorder: gix_traverse::tree::Recorder::default().track_location(location),
            }
        }
    }

    impl gix_traverse::tree::Visit for Delegate<'_> {
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
            self.recorder.pop_path_component();
        }

        fn visit_tree(&mut self, _entry: &EntryRef<'_>) -> gix_traverse::tree::visit::Action {
            gix_traverse::tree::visit::Action::Continue
        }

        fn visit_nontree(&mut self, entry: &EntryRef<'_>) -> gix_traverse::tree::visit::Action {
            if entry.mode.is_blob() {
                (self.push)(
                    Change::Modification {
                        previous_entry_mode: entry.mode,
                        previous_oid: gix_hash::ObjectId::null(entry.oid.kind()),
                        entry_mode: entry.mode,
                        oid: entry.oid.to_owned(),
                    },
                    self.recorder.path(),
                );
            }
            gix_traverse::tree::visit::Action::Continue
        }
    }
}
