/// Configure security relevant options when loading a git configuration.
#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Permissions {
    /// How to use the system configuration.
    /// This is defined as `$(prefix)/etc/gitconfig` on unix.
    pub system: git_sec::Permission,
    /// How to use the global configuration.
    /// This is usually `~/.gitconfig`.
    pub global: git_sec::Permission,
    /// How to use the user configuration.
    /// Second user-specific configuration path; if `$XDG_CONFIG_HOME` is not
    /// set or empty, `$HOME/.config/git/config` will be used.
    pub user: git_sec::Permission,
    /// How to use the repository configuration.
    pub repository: git_sec::Permission,
    /// How to use worktree configuration from `config.worktree`.
    // TODO: figure out how this really applies and provide more information here.
    pub worktree: git_sec::Permission,
    /// How to use the configuration from environment variables.
    pub env: git_sec::Permission,
    /// What to do when include files are encountered in loaded configuration.
    pub includes: git_sec::Permission,
}

impl Permissions {
    /// Allow everything which usually relates to a fully trusted environment
    pub fn all() -> Self {
        use git_sec::Permission::*;
        Permissions {
            system: Allow,
            global: Allow,
            user: Allow,
            repository: Allow,
            worktree: Allow,
            env: Allow,
            includes: Allow,
        }
    }

    /// If in doubt, this configuration can be used to safely load configuration from sources which is usually trusted,
    /// that is system and user configuration. Do load any configuration that isn't trusted as it's now owned by the current user.
    pub fn secure() -> Self {
        use git_sec::Permission::*;
        Permissions {
            system: Allow,
            global: Allow,
            user: Allow,
            repository: Deny,
            worktree: Deny,
            env: Allow,
            includes: Deny,
        }
    }
}
