use git_sec::{permission::Resource, Access, Trust};

use crate::permission::EnvVarResourcePermission;

/// Permissions associated with various resources of a git repository
pub struct Permissions {
    /// Control how a git-dir can be used.
    ///
    /// Note that a repository won't be usable at all unless read and write permissions are given.
    pub git_dir: Access<Resource, git_sec::ReadWrite>,
    /// Control whether resources pointed to by `XDG_CONFIG_HOME` can be used when looking up common configuration values.
    ///
    /// Note that [`git_sec::Permission::Forbid`] will cause the operation to abort if a resource is set via the XDG config environment.
    pub xdg_config_home: EnvVarResourcePermission,
    /// Control if resources pointed to by the
    pub home: EnvVarResourcePermission,
}

impl Permissions {
    /// Return permissions similar to what git does when the repository isn't owned by the current user,
    /// thus refusing all operations in it.
    pub fn strict() -> Self {
        Permissions {
            git_dir: Access::resource(git_sec::ReadWrite::empty()),
            xdg_config_home: Access::resource(git_sec::Permission::Allow),
            home: Access::resource(git_sec::Permission::Allow),
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
            xdg_config_home: Access::resource(git_sec::Permission::Allow),
            home: Access::resource(git_sec::Permission::Allow),
        }
    }

    /// Everything is allowed with this set of permissions, thus we read all configuration and do what git typically
    /// does with owned repositories.
    pub fn all() -> Self {
        Permissions {
            git_dir: Access::resource(git_sec::ReadWrite::all()),
            xdg_config_home: Access::resource(git_sec::Permission::Allow),
            home: Access::resource(git_sec::Permission::Allow),
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
