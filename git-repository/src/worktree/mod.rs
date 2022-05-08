use crate::bstr::{BStr, BString};
use crate::Repository;
pub use git_worktree::*;
use std::path::PathBuf;

/// A stand-in to a worktree as result of a worktree iteration.
///
/// It provides access to typical worktree state, but may not actually point to a valid checkout as the latter has been moved or
/// deleted.
#[derive(Debug, Clone)]
pub struct Proxy<'repo> {
    pub(crate) parent: &'repo Repository,
    pub(crate) git_dir: PathBuf,
}

/// Access
impl<'repo> crate::Worktree<'repo> {
    /// Read the location of the checkout, the base of the work tree
    pub fn base(&self) -> &'repo std::path::Path {
        self.path
    }

    /// Return true if this worktree is the main worktree associated with a non-bare git repository.
    ///
    /// It cannot be removed.
    pub fn is_main(&self) -> bool {
        self.id().is_none()
    }

    /// Return true if this worktree cannot be pruned, moved or deleted, which is useful if it is located on an external storage device.
    ///
    /// Always false for the main worktree.
    pub fn is_locked(&self) -> bool {
        Proxy::new(self.parent, self.parent.git_dir()).is_locked()
    }

    /// Provide a reason for the locking of this worktree, if it is locked at all.
    ///
    /// Note that we squelch errors in case the file cannot be read in which case the
    /// reason is an empty string.
    pub fn lock_reason(&self) -> Option<BString> {
        Proxy::new(self.parent, self.parent.git_dir()).lock_reason()
    }

    /// Return the ID of the repository worktree, if it is a linked worktree, or `None` if it's a linked worktree.
    pub fn id(&self) -> Option<&BStr> {
        id(self.parent.git_dir(), self.parent.common_dir.is_some())
    }
}

pub(crate) fn id(git_dir: &std::path::Path, has_common_dir: bool) -> Option<&BStr> {
    if !has_common_dir {
        return None;
    }
    let candidate = git_path::os_str_into_bstr(git_dir.file_name().expect("at least one directory level"))
        .expect("no illformed UTF-8");
    let maybe_worktrees = git_dir.parent()?;
    (maybe_worktrees.file_name()?.to_str()? == "worktrees").then(|| candidate)
}

///
pub mod proxy;

///
#[cfg(feature = "git-index")]
pub mod open_index {
    use crate::bstr::BString;

    /// The error returned by [`Worktree::open_index()`][crate::Worktree::open_index()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Could not interpret value '{}' as 'index.threads'", .value)]
        ConfigIndexThreads {
            value: BString,
            #[source]
            err: git_config::value::parse::Error,
        },
        #[error(transparent)]
        IndexFile(#[from] git_index::file::init::Error),
    }

    impl<'repo> crate::Worktree<'repo> {
        /// A shortcut to [`crate::Repository::open_index()`].
        pub fn open_index(&self) -> Result<git_index::File, Error> {
            self.parent.open_index()
        }
    }
}

///
#[cfg(feature = "git-index")]
pub mod excludes {
    use std::path::PathBuf;

    /// The error returned by [`Worktree::excludes()`][crate::Worktree::excludes()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Could not read repository exclude.")]
        Io(#[from] std::io::Error),
        #[error(transparent)]
        EnvironmentPermission(#[from] git_sec::permission::Error<PathBuf, git_sec::Permission>),
    }

    impl<'repo> crate::Worktree<'repo> {
        /// Configure a file-system cache checking if files below the repository are excluded.
        ///
        /// This takes into consideration all the usual repository configuration.
        // TODO: test
        pub fn excludes<'a>(
            &self,
            index: &'a git_index::State,
            overrides: Option<git_attributes::MatchGroup<git_attributes::Ignore>>,
        ) -> Result<git_worktree::fs::Cache<'a>, Error> {
            let repo = self.parent;
            let case = repo
                .config
                .ignore_case
                .then(|| git_glob::pattern::Case::Fold)
                .unwrap_or_default();
            let mut buf = Vec::with_capacity(512);
            let state = git_worktree::fs::cache::State::IgnoreStack(git_worktree::fs::cache::state::Ignore::new(
                overrides.unwrap_or_default(),
                git_attributes::MatchGroup::<git_attributes::Ignore>::from_git_dir(
                    repo.git_dir(),
                    match repo.config.excludes_file.as_ref() {
                        Some(user_path) => Some(user_path.to_owned()),
                        None => repo.config.xdg_config_path("ignore")?,
                    },
                    &mut buf,
                )?,
                None,
                case,
            ));
            let attribute_list = state.build_attribute_list(index, index.path_backing(), case);
            Ok(git_worktree::fs::Cache::new(
                self.path,
                state,
                case,
                buf,
                attribute_list,
            ))
        }
    }
}
