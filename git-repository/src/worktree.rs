#[cfg(all(feature = "unstable", feature = "git-worktree"))]
pub use git_worktree::*;

use crate::Repository;

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

/// A structure to make the API more stuctured.
pub struct Platform<'repo> {
    pub(crate) parent: &'repo Repository,
}

/// Access
impl<'repo> crate::Worktree<'repo> {
    /// Returns the root of the worktree under which all checked out files are located.
    pub fn root(&self) -> &std::path::Path {
        self.path
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

    // pub fn
    /// Open a new copy of the index file and decode it entirely.
    ///
    /// It will use the `index.threads` configuration key to learn how many threads to use.
    // TODO: test
    #[cfg(feature = "git-index")]
    pub fn open_index(&self) -> Result<git_index::File, crate::worktree::open_index::Error> {
        use std::convert::{TryFrom, TryInto};
        let repo = self.parent;
        let thread_limit = repo
            .config
            .resolved
            .boolean("index", None, "threads")
            .map(|res| {
                res.map(|value| if value { 0usize } else { 1 }).or_else(|err| {
                    git_config::values::Integer::try_from(err.input.as_ref())
                        .map_err(|err| crate::worktree::open_index::Error::ConfigIndexThreads {
                            value: err.input.clone(),
                            err,
                        })
                        .map(|value| value.to_decimal().and_then(|v| v.try_into().ok()).unwrap_or(1))
                })
            })
            .transpose()?;
        git_index::File::at(
            repo.git_dir().join("index"),
            git_index::decode::Options {
                object_hash: repo.object_hash(),
                thread_limit,
                min_extension_block_in_bytes_for_threading: 0,
            },
        )
        .map_err(Into::into)
    }
}
