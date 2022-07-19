use crate::Source;
use std::borrow::Cow;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

impl Source {
    /// Return true if the source indicates a location within a file of a repository.
    pub fn is_in_repository(self) -> bool {
        matches!(self, Source::Local | Source::Worktree)
    }

    /// Returns the location at which a file of this type would be stored, or `None` if
    /// there is no notion of persistent storage for this source, with `env_var` to obtain environment variables.
    /// Note that the location can be relative for repository-local sources like `Local` and `Worktree`,
    /// and the caller has to known which base it it relative to, namely the `common_dir` in the `Local` case
    /// and the `git_dir` in the `Worktree` case.
    /// Be aware that depending on environment overrides, multiple scopes might return the same path, which should
    /// only be loaded once nonetheless.
    ///
    /// With `env_var` it becomes possible to prevent accessing environment variables entirely to comply with `git-sec`
    /// permissions for example.
    pub fn storage_location(self, env_var: &mut dyn FnMut(&str) -> Option<OsString>) -> Option<Cow<'static, Path>> {
        use Source::*;
        match self {
            System => env_var("GIT_CONFIG_NO_SYSTEM")
                .is_none()
                .then(|| PathBuf::from(env_var("GIT_CONFIG_SYSTEM").unwrap_or_else(|| "/etc/gitconfig".into())).into()),
            Global => match env_var("GIT_CONFIG_GLOBAL") {
                Some(global_override) => Some(PathBuf::from(global_override).into()),
                None => env_var("XDG_CONFIG_HOME")
                    .map(|home| {
                        let mut p = PathBuf::from(home);
                        p.push("git");
                        p.push("config");
                        p
                    })
                    .or_else(|| {
                        env_var("HOME").map(|home| {
                            let mut p = PathBuf::from(home);
                            p.push(".config");
                            p.push("git");
                            p.push("config");
                            p
                        })
                    })
                    .map(Cow::Owned),
            },
            User => env_var("GIT_CONFIG_GLOBAL")
                .map(|global_override| PathBuf::from(global_override).into())
                .or_else(|| {
                    env_var("HOME").map(|home| {
                        let mut p = PathBuf::from(home);
                        p.push(".gitconfig");
                        p.into()
                    })
                }),
            Local => Some(Path::new("config").into()),
            Worktree => Some(Path::new("config.worktree").into()),
            Env | Cli | Api => None,
        }
    }
}
