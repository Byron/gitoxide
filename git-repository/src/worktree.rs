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

#[allow(missing_docs)]
pub mod git_dir {
    #[derive(Debug, thiserror::Error)]
    #[error("TBD: errors when reading git dir file")]
    pub struct Error;
}

mod proxy {
    use crate::bstr::{BStr, BString, ByteSlice};
    use crate::worktree::Proxy;
    use crate::{Repository, ThreadSafeRepository};
    use std::path::{Path, PathBuf};

    impl<'repo> Proxy<'repo> {
        pub(crate) fn new(parent: &'repo Repository, git_dir: impl Into<PathBuf>) -> Self {
            Proxy {
                parent,
                git_dir: git_dir.into(),
            }
        }
    }

    impl<'repo> Proxy<'repo> {
        /// Read the location of the checkout, the base of the work tree.
        /// Note that the location might not exist.
        pub fn base(&self) -> std::io::Result<PathBuf> {
            let git_dir = self.git_dir.join("gitdir");
            let mut base_dot_git = crate::path::read_from_file(&git_dir).ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Required file '{}' does not exist", git_dir.display()),
                )
            })??;
            if base_dot_git.file_name().and_then(|n| n.to_str()) == Some(".git") {
                base_dot_git.pop();
            }
            Ok(base_dot_git)
        }

        /// The git directory for the work tree, typically contained within the parent git dir.
        pub fn git_dir(&self) -> &Path {
            &self.git_dir
        }

        /// The name of the worktree, which is derived from its folder within the `worktrees` directory within the parent `.git` folder.
        pub fn id(&self) -> &BStr {
            git_path::os_str_into_bstr(self.git_dir.file_name().expect("worktrees/ parent dir"))
                .expect("no illformed UTF-8")
        }

        /// Return true if the worktree cannot be pruned, moved or deleted, which is useful if it is located on an external storage device.
        pub fn is_locked(&self) -> bool {
            self.git_dir.join("locked").is_file()
        }

        /// Provide a reason for the locking of this worktree, if it is locked at all.
        ///
        /// Note that we squelch errors in case the file cannot be read in which case the
        /// reason is an empty string.
        pub fn lock_reason(&self) -> Option<BString> {
            std::fs::read(self.git_dir.join("locked"))
                .ok()
                .map(|contents| contents.trim().into())
        }

        /// Transform this proxy into a [`Repository`] while ignoring issues reading `base()` and ignoring that it might not exist.
        ///
        /// Most importantly, the `Repository` might be initialized with a non-existing work tree directory as the checkout
        /// was removed or moved in the mean time or is unavailable for other reasons.
        /// The caller will encounter io errors if it's used like the work tree is guaranteed to be present, but can still access
        /// a lot of information if work tree access is avoided.
        pub fn into_repo_with_possibly_inaccessible_worktree(self) -> Result<Repository, crate::open::Error> {
            let base = self.base().ok();
            let repo =
                ThreadSafeRepository::open_from_paths(self.git_dir, base, self.parent.linked_worktree_options.clone())?;
            Ok(repo.into())
        }

        /// Like `into_repo_with_possibly_inaccessible_worktree()` but will fail if the `base()` cannot be read or
        /// if the worktree doesn't exist.
        ///
        /// Note that it won't fail if the worktree doesn't exist.
        pub fn into_repo(self) -> Result<Repository, crate::open::Error> {
            let base = self.base()?;
            if !base.is_dir() {
                todo!()
            }
            let repo = ThreadSafeRepository::open_from_paths(
                self.git_dir,
                base.into(),
                self.parent.linked_worktree_options.clone(),
            )?;
            Ok(repo.into())
        }
    }
}

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
}

/// Access
impl<'repo> crate::Worktree<'repo> {
    /// Returns the root of the worktree under which all checked out files are located.
    pub fn root(&self) -> &std::path::Path {
        self.path
    }
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
        todo!()
    }
}

impl<'repo> crate::Worktree<'repo> {
    /// Configure a file-system cache checking if files below the repository are excluded.
    ///
    /// This takes into consideration all the usual repository configuration.
    // TODO: test
    #[cfg(feature = "git-index")]
    pub fn excludes<'a>(
        &self,
        index: &'a git_index::State,
        overrides: Option<git_attributes::MatchGroup<git_attributes::Ignore>>,
    ) -> Result<git_worktree::fs::Cache<'a>, excludes::Error> {
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

    /// A shortcut to [`crate::Repository::open_index()`].
    #[cfg(feature = "git-index")]
    pub fn open_index(&self) -> Result<git_index::File, crate::worktree::open_index::Error> {
        self.parent.open_index()
    }
}
