use crate::{sections::gitoxide, keys, Key, Section, User, Tree};

impl User {
    /// The `user.name` key
    pub const NAME: keys::Any = keys::Any::new("name", &Tree::USER);
    /// The `user.email` key
    pub const EMAIL: keys::Any =
        keys::Any::new("email", &Tree::USER).with_fallback(&gitoxide::User::EMAIL_FALLBACK);
}

impl Section for User {
    fn name(&self) -> &str {
        "user"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[&Self::NAME, &Self::EMAIL]
    }
}
