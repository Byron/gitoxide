use git_sec::{permission::Resource, Access, Trust};

use crate::permission;

/// Permissions associated with various resources of a git repository
#[derive(Debug, Clone)]
pub struct Permissions {
    /// Control how a git-dir can be used.
    ///
    /// Note that a repository won't be usable at all unless read and write permissions are given.
    pub git_dir: Access<Resource, git_sec::ReadWrite>,
    /// Permissions related to the environment
    pub env: Environment,
    /// Permissions related to the handling of git configuration.
    pub config: Config,
}

/// Configure security relevant options when loading a git configuration.
#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug, Hash)]
pub struct Config {
    /// Whether to use the system configuration.
    /// This is defined as `$(prefix)/etc/gitconfig` on unix.
    pub system: bool,
    /// Whether to use the git application configuration.
    ///
    /// A platform defined location for where a user's git application configuration should be located.
    /// If `$XDG_CONFIG_HOME` is not set or empty, `$HOME/.config/git/config` will be used
    /// on unix.
    pub git: bool,
    /// Whether to use the user configuration.
    /// This is usually `~/.gitconfig` on unix.
    pub user: bool,
    /// Whether to use worktree configuration from `config.worktree`.
    // TODO: figure out how this really applies and provide more information here.
    // pub worktree: bool,
    /// Whether to use the configuration from environment variables.
    pub env: bool,
    /// Whether to follow include files are encountered in loaded configuration,
    /// via `include` and `includeIf` sections.
    ///
    /// Note that this needs access to `GIT_*` prefixed environment variables.
    pub includes: bool,
}

impl Config {
    /// Allow everything which usually relates to a fully trusted environment
    pub fn all() -> Self {
        Config {
            system: true,
            git: true,
            user: true,
            env: true,
            includes: true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::all()
    }
}

/// Permissions related to the usage of environment variables
#[derive(Debug, Clone)]
pub struct Environment {
    /// Control whether resources pointed to by `XDG_CONFIG_HOME` can be used when looking up common configuration values.
    ///
    /// Note that [`git_sec::Permission::Forbid`] will cause the operation to abort if a resource is set via the XDG config environment.
    pub xdg_config_home: permission::env_var::Resource,
    /// Control the way resources pointed to by the home directory (similar to `xdg_config_home`) may be used.
    pub home: permission::env_var::Resource,
    /// Control if resources pointed to by `GIT_*` prefixed environment variables can be used.
    pub git_prefix: permission::env_var::Resource,
}

impl Environment {
    /// Allow access to the entire environment.
    pub fn all() -> Self {
        Environment {
            xdg_config_home: Access::resource(git_sec::Permission::Allow),
            home: Access::resource(git_sec::Permission::Allow),
            git_prefix: Access::resource(git_sec::Permission::Allow),
        }
    }
}

impl Permissions {
    /// Return permissions similar to what git does when the repository isn't owned by the current user,
    /// thus refusing all operations in it.
    pub fn strict() -> Self {
        Permissions {
            git_dir: Access::resource(git_sec::ReadWrite::READ),
            env: Environment::all(),
            config: Config::all(),
        }
    }

    /// Return permissions that will not include configuration files not owned by the current user,
    /// but trust system and global configuration files along with those which are owned by the current user.
    ///
    /// This allows to read and write repositories even if they aren't owned by the current user, but avoid using
    /// anything else that could cause us to write into unknown locations or use programs beyond our `PATH`.
    pub fn secure() -> Self {
        Permissions {
            git_dir: Access::resource(git_sec::ReadWrite::all()),
            env: Environment::all(),
            config: Config::all(),
        }
    }

    /// Everything is allowed with this set of permissions, thus we read all configuration and do what git typically
    /// does with owned repositories.
    pub fn all() -> Self {
        Permissions {
            git_dir: Access::resource(git_sec::ReadWrite::all()),
            env: Environment::all(),
            config: Config::all(),
        }
    }
}

impl git_sec::trust::DefaultForLevel for Permissions {
    fn default_for_level(level: Trust) -> Self {
        match level {
            Trust::Full => Permissions::all(),
            Trust::Reduced => Permissions::secure(),
        }
    }
}

impl Default for Permissions {
    fn default() -> Self {
        Permissions::secure()
    }
}
