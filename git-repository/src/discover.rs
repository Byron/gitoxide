use std::path::Path;

pub use git_discover::*;

use crate::ThreadSafeRepository;

/// The error returned by [`crate::discover()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    Discover(#[from] upwards::Error),
    #[error(transparent)]
    Open(#[from] crate::open::Error),
}

impl ThreadSafeRepository {
    /// Try to open a git repository in `directory` and search upwards through its parents until one is found.
    pub fn discover(directory: impl AsRef<Path>) -> Result<Self, Error> {
        Self::discover_opts(directory, Default::default(), Default::default())
    }

    /// Try to open a git repository in `directory` and search upwards through its parents until one is found,
    /// while applying `options`. Then use the `trust_map` to determine which of our own repository options to use
    /// for instantiations.
    pub fn discover_opts(
        directory: impl AsRef<Path>,
        options: upwards::Options,
        trust_map: git_sec::trust::Mapping<crate::open::Options>,
    ) -> Result<Self, Error> {
        let (path, trust) = upwards_opts(directory, options)?;
        let (git_dir, worktree_dir) = path.into_repository_and_work_tree_directories();
        let options = trust_map.into_value_by_level(trust);
        Self::open_from_paths(git_dir, worktree_dir, options).map_err(Into::into)
    }

    /// Try to open a git repository directly from the environment.
    /// If that fails, discover upwards from `directory` until one is found,
    /// while applying discovery options from the environment.
    pub fn discover_with_environment_overrides(directory: impl AsRef<Path>) -> Result<Self, Error> {
        Self::discover_with_environment_overrides_opts(directory, Default::default(), Default::default())
    }

    /// Try to open a git repository directly from the environment, which reads `GIT_DIR`
    /// if it is set. If unset, discover upwards from `directory` until one is found,
    /// while applying `options` with overrides from the environment which includes:
    ///
    /// - `GIT_DISCOVERY_ACROSS_FILESYSTEM`
    /// - `GIT_CEILING_DIRECTORIES`
    ///
    /// Finally, use the `trust_map` to determine which of our own repository options to use
    /// based on the trust level of the effective repository directory.
    pub fn discover_with_environment_overrides_opts(
        directory: impl AsRef<Path>,
        mut options: upwards::Options,
        trust_map: git_sec::trust::Mapping<crate::open::Options>,
    ) -> Result<Self, Error> {
        fn apply_additional_environment(mut opts: upwards::Options) -> upwards::Options {
            use std::convert::TryFrom;

            use crate::bstr::ByteVec;

            if let Some(cross_fs) =
                std::env::var_os("GIT_DISCOVERY_ACROSS_FILESYSTEM").and_then(|v| Vec::from_os_string(v).ok())
            {
                if let Ok(b) = git_config::values::Boolean::try_from(cross_fs) {
                    opts.cross_fs = b.to_bool();
                }
            }
            opts
        }

        if std::env::var_os("GIT_DIR").is_some() {
            return Self::open_with_environment_overrides(directory.as_ref(), trust_map).map_err(Error::Open);
        }

        options = apply_additional_environment(options.apply_environment());
        Self::discover_opts(directory, options, trust_map)
    }
}
