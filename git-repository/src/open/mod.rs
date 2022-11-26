use std::path::PathBuf;

use crate::{bstr::BString, config, permission, Permissions};

/// A way to configure the usage of replacement objects, see `git replace`.
#[derive(Debug, Clone)]
pub enum ReplacementObjects {
    /// Allow replacement objects and configure the ref prefix the standard environment variable `GIT_REPLACE_REF_BASE`,
    /// or default to the standard `refs/replace/` prefix.
    UseWithEnvironmentRefPrefixOrDefault {
        /// If true, default true, a standard environment variable `GIT_NO_REPLACE_OBJECTS` to disable replacement objects entirely.
        allow_disable_via_environment: bool,
    },
    /// Use replacement objects and configure the prefix yourself.
    UseWithRefPrefix {
        /// The ref prefix to use, like `refs/alternative/` - note the trailing slash.
        prefix: PathBuf,
        /// If true, default true, a standard environment variable `GIT_NO_REPLACE_OBJECTS`
        allow_disable_via_environment: bool,
    },
    /// Do not use replacement objects at all.
    Disable,
}

impl Default for ReplacementObjects {
    fn default() -> Self {
        ReplacementObjects::UseWithEnvironmentRefPrefixOrDefault {
            allow_disable_via_environment: true,
        }
    }
}

impl ReplacementObjects {
    fn refs_prefix(self) -> Option<PathBuf> {
        use ReplacementObjects::*;
        let is_disabled = |allow_env: bool| allow_env && std::env::var_os("GIT_NO_REPLACE_OBJECTS").is_some();
        match self {
            UseWithEnvironmentRefPrefixOrDefault {
                allow_disable_via_environment,
            } => {
                if is_disabled(allow_disable_via_environment) {
                    return None;
                };
                PathBuf::from(std::env::var("GIT_REPLACE_REF_BASE").unwrap_or_else(|_| "refs/replace/".into())).into()
            }
            UseWithRefPrefix {
                prefix,
                allow_disable_via_environment,
            } => {
                if is_disabled(allow_disable_via_environment) {
                    return None;
                };
                prefix.into()
            }
            Disable => None,
        }
    }
}

/// The options used in [`ThreadSafeRepository::open_opts`]
#[derive(Clone)]
pub struct Options {
    pub(crate) object_store_slots: git_odb::store::init::Slots,
    pub(crate) replacement_objects: ReplacementObjects,
    /// Define what is allowed while opening a repository.
    pub permissions: Permissions,
    pub(crate) git_dir_trust: Option<git_sec::Trust>,
    /// Warning: this one is copied to to config::Cache - don't change it after repo open or keep in sync.
    pub(crate) filter_config_section: Option<fn(&git_config::file::Metadata) -> bool>,
    pub(crate) lossy_config: Option<bool>,
    pub(crate) lenient_config: bool,
    pub(crate) bail_if_untrusted: bool,
    pub(crate) api_config_overrides: Vec<BString>,
    pub(crate) cli_config_overrides: Vec<BString>,
    /// Internal to pass an already obtained CWD on to where it may also be used. This avoids the CWD being queried more than once per repo.
    pub(crate) current_dir: Option<PathBuf>,
}

/// The error returned by [`crate::open()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Failed to load the git configuration")]
    Config(#[from] config::Error),
    #[error(transparent)]
    NotARepository(#[from] git_discover::is_git::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("The git directory at '{}' is considered unsafe as it's not owned by the current user.", .path.display())]
    UnsafeGitDir { path: PathBuf },
    #[error(transparent)]
    EnvironmentAccessDenied(#[from] permission::env_var::resource::Error),
}

mod options;

mod repository;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_options() {
        let actual = std::mem::size_of::<Options>();
        let limit = 160;
        assert!(
            actual <= limit,
            "{} <= {}: size shouldn't change without us knowing (on windows, it's bigger)",
            actual,
            limit
        );
    }
}
