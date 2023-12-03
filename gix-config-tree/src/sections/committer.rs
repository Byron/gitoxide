use crate::{sections::gitoxide, keys, Committer, Key, Section, Tree};

impl Committer {
    /// The `committer.name` key.
    pub const NAME: keys::Any =
        keys::Any::new("name", &Tree::COMMITTER).with_fallback(&gitoxide::Committer::NAME_FALLBACK);
    /// The `committer.email` key.
    pub const EMAIL: keys::Any =
        keys::Any::new("email", &Tree::COMMITTER).with_fallback(&gitoxide::Committer::EMAIL_FALLBACK);
}

impl Section for Committer {
    fn name(&self) -> &str {
        "committer"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[&Self::NAME, &Self::EMAIL]
    }
}
