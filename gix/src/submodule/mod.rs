#![allow(clippy::result_large_err)]
//! Submodule plumbing and abstractions
//!
use std::{
    borrow::Cow,
    cell::{Ref, RefCell, RefMut},
    path::PathBuf,
};

use gix_odb::FindExt;
pub use gix_submodule::*;

use crate::{bstr::BStr, repository::IndexPersistedOrInMemory, Repository, Submodule};

pub(crate) type ModulesFileStorage = gix_features::threading::OwnShared<gix_fs::SharedFileSnapshotMut<File>>;
/// A lazily loaded and auto-updated worktree index.
pub type ModulesSnapshot = gix_fs::SharedFileSnapshot<File>;

/// The name of the file containing (sub) module information.
pub(crate) const MODULES_FILE: &str = ".gitmodules";

mod errors;
pub use errors::*;

/// A platform maintaining state needed to interact with submodules, created by [`Repository::submodules()].
pub(crate) struct SharedState<'repo> {
    pub(crate) repo: &'repo Repository,
    pub(crate) modules: ModulesSnapshot,
    is_active: RefCell<Option<IsActiveState>>,
    index: RefCell<Option<IndexPersistedOrInMemory>>,
}

impl<'repo> SharedState<'repo> {
    pub(crate) fn new(repo: &'repo Repository, modules: ModulesSnapshot) -> Self {
        SharedState {
            repo,
            modules,
            is_active: RefCell::new(None),
            index: RefCell::new(None),
        }
    }

    fn index(&self) -> Result<Ref<'_, IndexPersistedOrInMemory>, crate::repository::index_or_load_from_head::Error> {
        {
            let mut state = self.index.borrow_mut();
            if state.is_none() {
                *state = self.repo.index_or_load_from_head()?.into();
            }
        }
        Ok(Ref::map(self.index.borrow(), |opt| {
            opt.as_ref().expect("just initialized")
        }))
    }

    fn active_state_mut(
        &self,
    ) -> Result<(RefMut<'_, IsActivePlatform>, RefMut<'_, gix_worktree::Stack>), is_active::Error> {
        let mut state = self.is_active.borrow_mut();
        if state.is_none() {
            let platform = self
                .modules
                .is_active_platform(&self.repo.config.resolved, self.repo.config.pathspec_defaults()?)?;
            let index = self.index()?;
            let attributes = self
                .repo
                .attributes_only(
                    &index,
                    gix_worktree::stack::state::attributes::Source::WorktreeThenIdMapping
                        .adjust_for_bare(self.repo.is_bare()),
                )?
                .detach();
            *state = Some(IsActiveState { platform, attributes });
        }
        Ok(RefMut::map_split(state, |opt| {
            let state = opt.as_mut().expect("populated above");
            (&mut state.platform, &mut state.attributes)
        }))
    }
}

struct IsActiveState {
    platform: IsActivePlatform,
    attributes: gix_worktree::Stack,
}

///Access
impl<'repo> Submodule<'repo> {
    /// Return the submodule's name.
    pub fn name(&self) -> &BStr {
        self.name.as_ref()
    }
    /// Return the path at which the submodule can be found, relative to the repository.
    ///
    /// For details, see [gix_submodule::File::path()].
    pub fn path(&self) -> Result<Cow<'_, BStr>, config::path::Error> {
        self.state.modules.path(self.name())
    }

    /// Return the url from which to clone or update the submodule.
    pub fn url(&self) -> Result<gix_url::Url, config::url::Error> {
        self.state.modules.url(self.name())
    }

    /// Return the `update` field from this submodule's configuration, if present, or `None`.
    pub fn update(&self) -> Result<Option<config::Update>, config::update::Error> {
        self.state.modules.update(self.name())
    }

    /// Return the `branch` field from this submodule's configuration, if present, or `None`.
    pub fn branch(&self) -> Result<Option<config::Branch>, config::branch::Error> {
        self.state.modules.branch(self.name())
    }

    /// Return the `fetchRecurseSubmodules` field from this submodule's configuration, or retrieve the value from `fetch.recurseSubmodules` if unset.
    pub fn fetch_recurse(&self) -> Result<Option<config::FetchRecurse>, fetch_recurse::Error> {
        Ok(match self.state.modules.fetch_recurse(self.name())? {
            Some(val) => Some(val),
            None => self
                .state
                .repo
                .config
                .resolved
                .boolean_by_key("fetch.recurseSubmodules")
                .map(|res| crate::config::tree::Fetch::RECURSE_SUBMODULES.try_into_recurse_submodules(res))
                .transpose()?,
        })
    }

    /// Return the `ignore` field from this submodule's configuration, if present, or `None`.
    pub fn ignore(&self) -> Result<Option<config::Ignore>, config::Error> {
        self.state.modules.ignore(self.name())
    }

    /// Return the `shallow` field from this submodule's configuration, if present, or `None`.
    ///
    /// If `true`, the submodule will be checked out with `depth = 1`. If unset, `false` is assumed.
    pub fn shallow(&self) -> Result<Option<bool>, gix_config::value::Error> {
        self.state.modules.shallow(self.name())
    }

    /// Returns true if this submodule is considered active and can thus participate in an operation.
    ///
    /// Please see the [plumbing crate documentation](gix_submodule::IsActivePlatform::is_active()) for details.
    pub fn is_active(&self) -> Result<bool, is_active::Error> {
        let (mut platform, mut attributes) = self.state.active_state_mut()?;
        let is_active = platform.is_active(&self.state.repo.config.resolved, self.name.as_ref(), {
            &mut |relative_path, case, is_dir, out| {
                attributes
                    .set_case(case)
                    .at_entry(relative_path, Some(is_dir), |id, buf| {
                        self.state.repo.objects.find_blob(id, buf)
                    })
                    .map_or(false, |platform| platform.matching_attributes(out))
            }
        })?;
        Ok(is_active)
    }

    /// Return the object id of the submodule as stored in the index of the superproject,
    /// or `None` if it was deleted from the index.
    ///
    /// If `None`, but `Some()` when calling [`Self::head_id()`], then the submodule was just deleted but the change
    /// wasn't yet committed. Note that `None` is also returned if the entry at the submodule path isn't a submodule.
    /// If `Some()`, but `None` when calling [`Self::head_id()`], then the submodule was just added without having committed the change.
    pub fn index_id(&self) -> Result<Option<gix_hash::ObjectId>, index_id::Error> {
        let path = self.path()?;
        Ok(self
            .state
            .index()?
            .entry_by_path(&path)
            .and_then(|entry| (entry.mode == gix_index::entry::Mode::COMMIT).then_some(entry.id)))
    }

    /// Return the object id of the submodule as stored in `HEAD^{tree}` of the superproject, or `None` if it wasn't yet committed.
    ///
    /// If `Some()`, but `None` when calling [`Self::index_id()`], then the submodule was just deleted but the change
    /// wasn't yet committed. Note that `None` is also returned if the entry at the submodule path isn't a submodule.
    /// If `None`, but `Some()` when calling [`Self::index_id()`], then the submodule was just added without having committed the change.
    pub fn head_id(&self) -> Result<Option<gix_hash::ObjectId>, head_id::Error> {
        let path = self.path()?;
        Ok(self
            .state
            .repo
            .head_commit()?
            .tree()?
            .peel_to_entry_by_path(gix_path::from_bstr(path.as_ref()))?
            .and_then(|entry| (entry.mode() == gix_object::tree::EntryMode::Commit).then_some(entry.inner.oid)))
    }

    /// Return the path at which the repository of the submodule should be located.
    ///
    /// The directory might not exist yet.
    pub fn git_dir(&self) -> PathBuf {
        self.state
            .repo
            .common_dir()
            .join("modules")
            .join(gix_path::from_bstr(self.name()))
    }

    /// Return the path to the location at which the workdir would be checked out.
    ///
    /// Note that it may be a path relative to the repository if, for some reason, the parent directory
    /// doesn't have a working dir set.
    pub fn work_dir(&self) -> Result<PathBuf, config::path::Error> {
        let worktree_git = gix_path::from_bstr(self.path()?);
        Ok(match self.state.repo.work_dir() {
            None => worktree_git.into_owned(),
            Some(prefix) => prefix.join(worktree_git),
        })
    }

    /// Return the path at which the repository of the submodule should be located, or the path inside of
    /// the superproject's worktree where it actually *is* located if the submodule in the 'old-form', thus is a directory
    /// inside of the superproject's work-tree.
    ///
    /// Note that 'old-form' paths returned aren't verified, i.e. the `.git` repository might be corrupt or otherwise
    /// invalid - it's left to the caller to try to open it.
    ///
    /// Also note that the returned path may not actually exist.
    pub fn git_dir_try_old_form(&self) -> Result<PathBuf, config::path::Error> {
        let worktree_git = self.work_dir()?.join(gix_discover::DOT_GIT_DIR);
        Ok(if worktree_git.is_dir() {
            worktree_git
        } else {
            self.git_dir()
        })
    }

    /// Query various parts of the submodule and assemble it into state information.
    #[doc(alias = "status", alias = "git2")]
    pub fn state(&self) -> Result<State, config::path::Error> {
        let maybe_old_path = self.git_dir_try_old_form()?;
        let git_dir = self.git_dir();
        let worktree_git = self.work_dir()?.join(gix_discover::DOT_GIT_DIR);
        let superproject_configuration = self
            .state
            .repo
            .config
            .resolved
            .sections_by_name("submodule")
            .into_iter()
            .flatten()
            .any(|section| section.header().subsection_name() == Some(self.name.as_ref()));
        Ok(State {
            repository_exists: maybe_old_path.is_dir(),
            is_old_form: maybe_old_path != git_dir,
            worktree_checkout: worktree_git.exists(),
            superproject_configuration,
        })
    }

    /// Open the submodule as repository, or `None` if the submodule wasn't initialized yet.
    ///
    /// More states can be derived here:
    ///
    /// * *initialized* - a repository exists, i.e. `Some(repo)` and the working tree is present.
    /// * *uninitialized* - a repository does not exist, i.e. `None`
    /// * *deinitialized* - a repository does exist, i.e. `Some(repo)`, but its working tree is empty.
    ///
    /// Also see the [state()](Self::state()) method for learning about the submodule.
    /// The repository can also be used to learn about the submodule `HEAD`, i.e. where its working tree is at,
    /// which may differ compared to the superproject's index or `HEAD` commit.
    pub fn open(&self) -> Result<Option<Repository>, open::Error> {
        match crate::open_opts(self.git_dir_try_old_form()?, self.state.repo.options.clone()) {
            Ok(repo) => Ok(Some(repo)),
            Err(crate::open::Error::NotARepository { .. }) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}

/// A summary of the state of all parts forming a submodule, which allows to answer various questions about it.
///
/// Note that expensive questions about its presence in the `HEAD` or the `index` are left to the caller.
#[derive(Default, Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct State {
    /// if the submodule repository has been cloned.
    pub repository_exists: bool,
    /// if the submodule repository is located directly in the worktree of the superproject.
    pub is_old_form: bool,
    /// if the worktree is checked out.
    pub worktree_checkout: bool,
    /// If submodule configuration was found in the superproject's `.git/config` file.
    /// Note that the presence of a single section is enough, independently of the actual values.
    pub superproject_configuration: bool,
}
