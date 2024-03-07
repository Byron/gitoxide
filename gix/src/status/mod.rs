use crate::{config, Repository};
pub use gix_status as plumbing;

/// A structure to hold options configuring the status request, which can then be turned into an iterator.
pub struct Platform<'repo, Progress>
where
    Progress: gix_features::progress::Progress + 'static,
{
    #[cfg_attr(not(feature = "parallel"), allow(dead_code))]
    repo: &'repo Repository,
    #[cfg_attr(not(feature = "parallel"), allow(dead_code))]
    progress: Progress,
    index: Option<crate::worktree::IndexPersistedOrInMemory>,
    submodules: Submodule,
    index_worktree_options: index_worktree::Options,
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

impl Default for Submodule {
    fn default() -> Self {
        Submodule::AsConfigured { check_dirty: false }
    }
}

/// Status
impl Repository {
    /// Obtain a platform for configuring iterators for traversing git repository status information.
    ///
    /// By default, this is set to the fastest and most immediate way of obtaining a status,
    /// which is most similar to
    ///
    /// `git status --untracked=all --ignored=no --no-renames`
    ///
    /// which implies that submodule information is provided by default.
    ///
    /// Pass `progress` to receive progress information on file modifications on this repository.
    /// Use [`progress::Discard`](crate::progress::Discard) to discard all progress information.
    ///
    /// ### Deviation
    ///
    /// Whereas Git runs the index-modified check before the directory walk to set entries
    /// as up-to-date to (potentially) safe some disk-access, we run both in parallel which
    /// ultimately is much faster.
    pub fn status<P>(&self, progress: P) -> Result<Platform<'_, P>, config::boolean::Error>
    where
        P: gix_features::progress::Progress + 'static,
    {
        Ok(Platform {
            repo: self,
            progress,
            index: None,
            submodules: Submodule::default(),
            index_worktree_options: index_worktree::Options {
                sorting: None,
                dirwalk_options: Some(self.dirwalk_options()?),
                rewrites: None,
                thread_limit: None,
            },
        })
    }
}

mod platform;

///
pub mod index_worktree;
