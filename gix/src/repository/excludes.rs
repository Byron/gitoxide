//! exclude information
use crate::Repository;
use std::path::PathBuf;

/// The error returned by [`Repository::excludes()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Could not read repository exclude")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    EnvironmentPermission(#[from] gix_sec::permission::Error<PathBuf>),
    #[error("The value for `core.excludesFile` could not be read from configuration")]
    ExcludesFilePathInterpolation(#[from] gix_config::path::interpolate::Error),
}

impl Repository {
    /// Configure a file-system cache checking if files below the repository are excluded.
    ///
    /// Note that no worktree is required for this to work, even though access to in-tree `.gitignore` files would require
    /// a non-empty `index` that represents a tree with `.gitignore` files.
    ///
    /// This takes into consideration all the usual repository configuration, namely:
    ///
    /// * `$XDG_CONFIG_HOME/…/ignore` if `core.excludesFile` is *not* set, otherwise use the configured file.
    /// * `$GIT_DIR/info/exclude` if present.
    // TODO: test, provide higher-level custom Cache wrapper that is much easier to use and doesn't panic when accessing entries
    //       by non-relative path.
    pub fn excludes(
        &self,
        index: &gix_index::State,
        overrides: Option<gix_ignore::Search>,
    ) -> Result<gix_worktree::Cache, Error> {
        let case = if self.config.ignore_case {
            gix_glob::pattern::Case::Fold
        } else {
            gix_glob::pattern::Case::Sensitive
        };
        let mut buf = Vec::with_capacity(512);
        let excludes_file = match self.config.excludes_file().transpose()? {
            Some(user_path) => Some(user_path),
            None => self.config.xdg_config_path("ignore")?,
        };
        let state = gix_worktree::cache::State::IgnoreStack(gix_worktree::cache::state::Ignore::new(
            overrides.unwrap_or_default(),
            gix_ignore::Search::from_git_dir(self.git_dir(), excludes_file, &mut buf)?,
            None,
        ));
        let attribute_list = state.id_mappings_from_index(index, index.path_backing(), case);
        Ok(gix_worktree::Cache::new(
            // this is alright as we don't cause mutation of that directory, it's virtual.
            self.work_dir().unwrap_or(self.git_dir()),
            state,
            case,
            buf,
            attribute_list,
        ))
    }
}
