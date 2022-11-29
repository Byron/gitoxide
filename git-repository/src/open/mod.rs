use std::path::PathBuf;

use crate::{bstr::BString, config, permission, Permissions};

/// The options used in [`ThreadSafeRepository::open_opts()`][crate::ThreadSafeRepository::open_opts()].
///
/// ### Replacement Objects for the object database
///
/// The environment variables `GIT_REPLACE_REF_BASE` and `GIT_NO_REPLACE_OBJECTS` are mapped to `gitoxide.objects.replaceRefBase`
/// and `gitoxide.objects.noReplace` respectively and then interpreted exactly as their environment variable counterparts.
///
/// Use [Permissions] to control which environment variables can be read, and config-overrides to control these values programmatically.
#[derive(Clone)]
pub struct Options {
    pub(crate) object_store_slots: git_odb::store::init::Slots,
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
