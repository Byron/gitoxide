use crate::{
    config,
    config::tree::{keys, Init, Key, Section},
};

impl Init {
    /// The `init.defaultBranch` key.
    pub const DEFAULT_BRANCH: keys::Any = keys::Any::new("defaultBranch", &config::Tree::INIT)
        .with_deviation("If not set, we use `main` instead of `master`");
}

impl Section for Init {
    fn name(&self) -> &str {
        "init"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[&Self::DEFAULT_BRANCH]
    }
}
