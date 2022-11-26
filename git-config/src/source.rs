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
        use Source::*;
        match self {
            GitInstallation => Kind::GitInstallation,
            System => Kind::System,
            Git | User => Kind::Global,
            Local | Worktree => Kind::Repository,
            Env | Cli | Api | EnvOverride => Kind::Override,
        }
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
            GitInstallation => git::install_config_path().map(git_path::from_bstr),
            System => env_var("GIT_CONFIG_NO_SYSTEM")
                .is_none()
                .then(|| PathBuf::from(env_var("GIT_CONFIG_SYSTEM").unwrap_or_else(|| "/etc/gitconfig".into())).into()),
            Git => match env_var("GIT_CONFIG_GLOBAL") {
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
            Env | Cli | Api | EnvOverride => None,
        }
    }
}

/// Environment information involving the `git` program itself.
mod git {
    use std::process::{Command, Stdio};

    use bstr::{BStr, BString, ByteSlice};

    /// Returns the file that contains git configuration coming with the installation of the `git` file in the current `PATH`, or `None`
    /// if no `git` executable was found or there were other errors during execution.
    pub fn install_config_path() -> Option<&'static BStr> {
        static PATH: once_cell::sync::Lazy<Option<BString>> = once_cell::sync::Lazy::new(|| {
            let mut cmd = Command::new(if cfg!(windows) { "git.exe" } else { "git" });
            cmd.args(["config", "-l", "--show-origin"])
                .stdin(Stdio::null())
                .stderr(Stdio::null());
            first_file_from_config_with_origin(cmd.output().ok()?.stdout.as_slice().into()).map(ToOwned::to_owned)
        });
        PATH.as_ref().map(|b| b.as_ref())
    }

    fn first_file_from_config_with_origin(source: &BStr) -> Option<&BStr> {
        let file = source.strip_prefix(b"file:")?;
        let end_pos = file.find_byte(b'\t')?;
        file[..end_pos].as_bstr().into()
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn first_file_from_config_with_origin() {
            let macos = "file:/Applications/Xcode.app/Contents/Developer/usr/share/git-core/gitconfig	credential.helper=osxkeychain\nfile:/Users/byron/.gitconfig	push.default=simple\n";
            let win_msys =
                "file:C:/git-sdk-64/etc/gitconfig	core.symlinks=false\r\nfile:C:/git-sdk-64/etc/gitconfig	core.autocrlf=true";
            let win_cmd = "file:C:/Program Files/Git/etc/gitconfig	diff.astextplain.textconv=astextplain\r\nfile:C:/Program Files/Git/etc/gitconfig	filter.lfs.clean=git-lfs clean -- %f\r\n";
            let linux = "file:/home/parallels/.gitconfig	core.excludesfile=~/.gitignore\n";
            let bogus = "something unexpected";
            let empty = "";

            for (source, expected) in [
                (
                    macos,
                    Some("/Applications/Xcode.app/Contents/Developer/usr/share/git-core/gitconfig"),
                ),
                (win_msys, Some("C:/git-sdk-64/etc/gitconfig")),
                (win_cmd, Some("C:/Program Files/Git/etc/gitconfig")),
                (linux, Some("/home/parallels/.gitconfig")),
                (bogus, None),
                (empty, None),
            ] {
                assert_eq!(
                    super::first_file_from_config_with_origin(source.into()),
                    expected.map(Into::into)
                );
            }
        }
    }
}
