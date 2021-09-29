use std::fmt;

use git_repository as git;

#[derive(PartialEq, Eq, Ord, PartialOrd, Debug, Clone)]
pub enum Category {
    Issue(String),
    Uncategorized,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Uncategorized => f.write_str("Uncategorized"),
            Category::Issue(issue) => write!(f, "#{}", issue),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Message {
    pub title: String,
    pub id: git::hash::ObjectId,
}

impl From<&crate::commit::history::Item> for Message {
    fn from(v: &crate::commit::history::Item) -> Self {
        Message {
            title: v.message.title.to_owned(),
            id: v.id,
        }
    }
}
