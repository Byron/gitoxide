use std::collections::HashMap;

use git_repository as git;

pub mod history;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Message {
    /// The cleared, plain title with any `additions` removed.
    pub title: String,
    /// More detailed information about the changes.
    pub body: Option<String>,
    /// If set, the git-conventional scope to help organizing changes.
    pub kind: Option<&'static str>,
    /// If set, this is a breaking change as indicated git-conventional.
    pub breaking: bool,
    /// If set, this commit message body contains a specific description of the breaking change.
    pub breaking_description: Option<String>,
    /// all additional information parsed from the title.
    pub additions: Vec<message::Addition>,
}

pub struct History {
    pub head: git::refs::Reference,
    pub items: Vec<history::Item>,
    /// A mapping between trees and their data
    pub data_by_tree_id: HashMap<git::ObjectId, Vec<u8>>,
}

pub mod message;
