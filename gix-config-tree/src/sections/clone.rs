use crate::{keys, Clone, Key, Section, Tree};

impl Clone {
    /// The `clone.defaultRemoteName` key.
    pub const DEFAULT_REMOTE_NAME: keys::RemoteName =
        keys::RemoteName::new_remote_name("defaultRemoteName", &Tree::CLONE);
    /// The `clone.rejectShallow` key.
    pub const REJECT_SHALLOW: keys::Boolean = keys::Boolean::new_boolean("rejectShallow", &Tree::CLONE);
}

impl Section for Clone {
    fn name(&self) -> &str {
        "clone"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[&Self::DEFAULT_REMOTE_NAME, &Self::REJECT_SHALLOW]
    }
}
