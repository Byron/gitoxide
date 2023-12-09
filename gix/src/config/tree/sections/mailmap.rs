use crate::config::{
    tree::{keys, Key, Mailmap, Section},
    Tree,
};

impl Mailmap {
    /// The `mailmap.blob` key
    pub const BLOB: keys::Any = keys::Any::new("blob", &Tree::MAILMAP);
    /// The `mailmap.file` key
    pub const FILE: keys::Any = keys::Any::new("file", &Tree::MAILMAP);
}

impl Section for Mailmap {
    fn name(&self) -> &str {
        "mailmap"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[&Self::BLOB, &Self::FILE]
    }
}
