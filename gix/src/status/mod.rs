use crate::config::cache::util::ApplyLeniencyDefault;
use crate::util::OwnedOrStaticAtomicBool;
use crate::{config, Repository};
pub use gix_status as plumbing;

/// A structure to hold options configuring the status request, which can then be turned into an iterator.
pub struct Platform<'repo, Progress>
where
    Progress: gix_features::progress::Progress + 'static,
{
    repo: &'repo Repository,
    progress: Progress,
    index: Option<crate::worktree::IndexPersistedOrInMemory>,
    submodules: Submodule,
    index_worktree_options: index_worktree::Options,
    should_interrupt: Option<OwnedOrStaticAtomicBool>,
}

/// How to obtain a submodule's status.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Submodule {
    /// Use the ['ignore' value](crate::Submodule::ignore) to determine which submodules
    /// participate in the status query, and to which extent.
    AsConfigured {
        /// If `true`, default `false`, the computation will stop once the first in a ladder operations
        /// ordered from cheap to expensive shows that the submodule is dirty.
        /// Thus, submodules that are clean will still impose the complete set of computation, as configured.
        check_dirty: bool,
    },
    /// Instead of the configuration, use the given ['ignore' value](crate::submodule::config::Ignore).
    /// This makes it possible to fine-tune the amount of work invested in this status, while allowing
    /// to turn off all submodule status information.
    Given {
        /// The portion of the submodule status to ignore.
        ignore: crate::submodule::config::Ignore,
        /// If `true`, default `false`, the computation will stop once the first in a ladder operations
        /// ordered from cheap to expensive shows that the submodule is dirty.
        /// Thus, submodules that are clean will still impose the complete set of computation, as given.
        check_dirty: bool,
    },
}

/// How untracked files should be handled.
#[derive(Default, Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum UntrackedFiles {
    /// Do not show any untracked files.
    ///
    /// This can mean no directory walk is performed.
    None,
    /// If possible, collapse files into their parent folders to reduce the amount of
    /// emitted untracked files.
    #[default]
    Collapsed,
    /// Show each individual untracked file or directory (if empty directories are emitted) that the dirwalk encountered .
    Files,
}

impl Default for Submodule {
    fn default() -> Self {
        Submodule::AsConfigured { check_dirty: false }
    }
}

/// The error returned by [status()](Repository::status).
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    DirwalkOptions(#[from] config::boolean::Error),
    #[error(transparent)]
    ConfigureUntrackedFiles(#[from] config::key::GenericErrorWithValue),
}

/// Status
impl Repository {
    /// Obtain a platform for configuring iterators for traversing git repository status information.
    ///
    /// By default, this is set to the fastest and most immediate way of obtaining a status,
    /// which is most similar to
    ///
    /// `git status --ignored=no`
    ///
    /// which implies that submodule information is provided by default.
    ///
    /// Note that `status.showUntrackedFiles` is respected, which leads to untracked files being
    /// collapsed by default. If that needs to be controlled,
    /// [configure the directory walk explicitly](Platform::dirwalk_options) or more [implicitly](Platform::untracked_files).
    ///
    /// Pass `progress` to receive progress information on file modifications on this repository.
    /// Use [`progress::Discard`](crate::progress::Discard) to discard all progress information.
    ///
    /// ### Deviation
    ///
    /// Whereas Git runs the index-modified check before the directory walk to set entries
    /// as up-to-date to (potentially) safe some disk-access, we run both in parallel which
    /// ultimately is much faster.
    pub fn status<P>(&self, progress: P) -> Result<Platform<'_, P>, Error>
    where
        P: gix_features::progress::Progress + 'static,
    {
        let platform = Platform {
            repo: self,
            progress,
            index: None,
            submodules: Submodule::default(),
            should_interrupt: None,
            index_worktree_options: index_worktree::Options {
                sorting: None,
                dirwalk_options: Some(self.dirwalk_options()?),
                rewrites: None,
                thread_limit: None,
            },
        };

        let untracked = self
            .config
            .resolved
            .string("status", None, "showUntrackedFiles")
            .map(|value| {
                config::tree::Status::SHOW_UNTRACKED_FILES
                    .try_into_show_untracked_files(value)
                    .with_lenient_default(self.config.lenient_config)
            })
            .transpose()?
            .unwrap_or_default();
        Ok(platform.untracked_files(untracked))
    }
}

///
#[allow(clippy::empty_docs)]
pub mod is_dirty {
    use crate::Repository;

    /// The error returned by [Repository::is_dirty()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        StatusPlatform(#[from] crate::status::Error),
        #[error(transparent)]
        CreateStatusIterator(#[from] crate::status::index_worktree::iter::Error),
    }

    impl Repository {
        /// Returns `true` if the repository is dirty.
        /// This means it's changed in one of the following ways:
        ///
        /// * the index was changed in comparison to its working tree
        /// * the working tree was changed in comparison to the index
        /// * submodules are taken in consideration, along with their `ignore` and `isActive` configuration
        ///
        /// Note that *untracked files* do *not* affect this flag.
        ///
        /// ### Incomplete Implementation Warning
        ///
        /// Currently, this does not compute changes between the head and the index.
        // TODO: use iterator which also tests for head->index changes.
        pub fn is_dirty(&self) -> Result<bool, Error> {
            let is_dirty = self
                .status(gix_features::progress::Discard)?
                .index_worktree_rewrites(None)
                .index_worktree_submodules(crate::status::Submodule::AsConfigured { check_dirty: true })
                .index_worktree_options_mut(|opts| {
                    opts.dirwalk_options = None;
                })
                .into_index_worktree_iter(Vec::new())?
                .take_while(Result::is_ok)
                .next()
                .is_some();
            Ok(is_dirty)
        }
    }
}

mod platform;

///
#[allow(clippy::empty_docs)]
pub mod index_worktree;
