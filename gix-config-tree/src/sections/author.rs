use crate::{sections::gitoxide, keys, Author, Key, Section, Tree};

impl Author {
    /// The `author.name` key.
    pub const NAME: keys::Any =
        keys::Any::new("name", &Tree::AUTHOR).with_fallback(&gitoxide::Author::NAME_FALLBACK);
    /// The `author.email` key.
    pub const EMAIL: keys::Any =
        keys::Any::new("email", &Tree::AUTHOR).with_fallback(&gitoxide::Author::EMAIL_FALLBACK);
}

impl Section for Author {
    fn name(&self) -> &str {
        "author"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[&Self::NAME, &Self::EMAIL]
    }
}
