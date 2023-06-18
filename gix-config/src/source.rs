use std::{
    borrow::Cow,
    ffi::OsString,
    path::{Path, PathBuf},
};

use crate::Source;

/// The category of a [`Source`], in order of ascending precedence.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Kind {
    /// A special configuration file that ships with the git installation, and is thus tied to the used git binary.
    GitInstallation,
    /// A source shared for the entire system.
    System,
    /// Application specific configuration unique for each user of the `System`.
    Global,
    /// Configuration relevant only to the repository, possibly including the worktree.
    Repository,
    /// Configuration specified after all other configuration was loaded for the purpose of overrides.
    Override,
}

impl Kind {
    /// Return a list of sources associated with this `Kind` of source, in order of ascending precedence.
    pub fn sources(self) -> &'static [Source] {
        let src = match self {
            Kind::GitInstallation => &[Source::GitInstallation] as &[_],
            Kind::System => &[Source::System],
            Kind::Global => &[Source::Git, Source::User],
            Kind::Repository => &[Source::Local, Source::Worktree],
            Kind::Override => &[Source::Env, Source::Cli, Source::Api],
        };
        debug_assert!(
            src.iter().all(|src| src.kind() == self),
            "BUG: classification of source has to match the ordering here, see `Source::kind()`"
        );
        src
    }
}

impl Source {
    /// Return true if the source indicates a location within a file of a repository.
    pub const fn kind(self) -> Kind {
        match self {
            Self::GitInstallation => Kind::GitInstallation,
            Self::System => Kind::System,
            Self::Git | Self::User => Kind::Global,
            Self::Local | Self::Worktree => Kind::Repository,
            Self::Env | Self::Cli | Self::Api | Self::EnvOverride => Kind::Override,
        }
    }

    /// Returns the location at which a file of this type would be stored, or `None` if
    /// there is no notion of persistent storage for this source, with `env_var` to obtain environment variables.
    /// Note that the location can be relative for repository-local sources like `Local` and `Worktree`,
    /// and the caller has to known which base it is relative to, namely the `common_dir` in the `Local` case
    /// and the `git_dir` in the `Worktree` case.
    /// Be aware that depending on environment overrides, multiple scopes might return the same path, which should
    /// only be loaded once nonetheless.
    ///
    /// With `env_var` it becomes possible to prevent accessing environment variables entirely to comply with `gix-sec`
    /// permissions for example.
    pub fn storage_location(self, env_var: &mut dyn FnMut(&str) -> Option<OsString>) -> Option<Cow<'static, Path>> {
        match self {
            Self::GitInstallation => gix_path::env::installation_config().map(Into::into),
            Self::System => {
                if env_var("GIT_CONFIG_NO_SYSTEM").is_some() {
                    None
                } else {
                    env_var("GIT_CONFIG_SYSTEM")
                        .map(|p| Cow::Owned(p.into()))
                        .or_else(|| gix_path::env::system_prefix().map(|p| p.join("etc/gitconfig").into()))
                }
            }
            Self::Git => match env_var("GIT_CONFIG_GLOBAL") {
                Some(global_override) => Some(PathBuf::from(global_override).into()),
                None => gix_path::env::xdg_config("config", env_var).map(Cow::Owned),
            },
            Self::User => env_var("GIT_CONFIG_GLOBAL")
                .map(|global_override| PathBuf::from(global_override).into())
                .or_else(|| {
                    env_var("HOME").map(|home| {
                        let mut p = PathBuf::from(home);
                        p.push(".gitconfig");
                        p.into()
                    })
                }),
            Self::Local => Some(Path::new("config").into()),
            Self::Worktree => Some(Path::new("config.worktree").into()),
            Self::Env | Self::Cli | Self::Api | Self::EnvOverride => None,
        }
    }
}
