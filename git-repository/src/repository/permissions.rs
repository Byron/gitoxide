use git_sec::Trust;

/// Permissions associated with various resources of a git repository
#[derive(Debug, Clone)]
pub struct Permissions {
    /// Permissions related to the environment
    pub env: Environment,
    /// Permissions related to the handling of git configuration.
    pub config: Config,
}

/// Configure from which sources git configuration may be loaded.
///
/// Note that configuration from inside of the repository is always loaded as it's definitely required for correctness.
#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug, Hash)]
pub struct Config {
    /// The git binary may come with configuration as part of its configuration, and if this is true (default false)
    /// we will load the configuration of the git binary, if present and not a duplicate of the ones below.
    ///
    /// It's disable by default as it involves executing the git binary once per execution of the application.
    pub git_binary: bool,
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
    /// Whether to use the configuration from environment variables.
    pub env: bool,
    /// Whether to follow include files are encountered in loaded configuration,
    /// via `include` and `includeIf` sections.
    pub includes: bool,
}

impl Config {
    /// Allow everything which usually relates to a fully trusted environment
    pub fn all() -> Self {
        Config {
            git_binary: false,
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
    pub xdg_config_home: git_sec::Permission,
    /// Control the way resources pointed to by the home directory (similar to `xdg_config_home`) may be used.
    pub home: git_sec::Permission,
    /// Control if environment variables to configure the HTTP transport, like `http_proxy` may be used.
    ///
    /// Note that http-transport related environment variables prefixed with `GIT_` may also be included here
    /// if they match this category like `GIT_HTTP_USER_AGENT`.
    pub http_transport: git_sec::Permission,
    /// Control if the `EMAIL` environment variables may be read.
    ///
    /// Note that identity related environment variables prefixed with `GIT_` may also be included here
    /// if they match this category.
    pub identity: git_sec::Permission,
    /// Control if environment variables related to the object database are handled. This includes features and performance
    /// options alike.
    pub objects: git_sec::Permission,
    /// Control if resources pointed to by `GIT_*` prefixed environment variables can be used, **but only** if they
    /// are not contained in any other category. This is a catch-all section.
    pub git_prefix: git_sec::Permission,
    /// Control if resources pointed to by `SSH_*` prefixed environment variables can be used (like `SSH_ASKPASS`)
    pub ssh_prefix: git_sec::Permission,
}

impl Environment {
    /// Allow access to the entire environment.
    pub fn all() -> Self {
        let allow = git_sec::Permission::Allow;
        Environment {
            xdg_config_home: allow,
            home: allow,
            git_prefix: allow,
            ssh_prefix: allow,
            http_transport: allow,
            identity: allow,
            objects: allow,
        }
    }
}

impl Permissions {
    /// Return permissions that will not include configuration files not owned by the current user,
    /// but trust system and global configuration files along with those which are owned by the current user.
    ///
    /// This allows to read and write repositories even if they aren't owned by the current user, but avoid using
    /// anything else that could cause us to write into unknown locations or use programs beyond our `PATH`.
    pub fn secure() -> Self {
        Permissions {
            env: Environment::all(),
            config: Config::all(),
        }
    }

    /// Everything is allowed with this set of permissions, thus we read all configuration and do what git typically
    /// does with owned repositories.
    pub fn all() -> Self {
        Permissions {
            env: Environment::all(),
            config: Config::all(),
        }
    }

    /// Don't read any but the local git configuration and deny reading any environment variables.
    pub fn isolated() -> Self {
        Permissions {
            config: Config {
                git_binary: false,
                system: false,
                git: false,
                user: false,
                env: false,
                includes: false,
            },
            env: {
                let deny = git_sec::Permission::Deny;
                Environment {
                    xdg_config_home: deny,
                    home: deny,
                    ssh_prefix: deny,
                    git_prefix: deny,
                    http_transport: deny,
                    identity: deny,
                    objects: deny,
                }
            },
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
