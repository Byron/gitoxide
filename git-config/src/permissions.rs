use crate::Permissions;

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
