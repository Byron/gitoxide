#![allow(unused)]

use std::borrow::Cow;

pub mod history;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Message<'a> {
    /// The cleared, plain title with any `additions` removed.
    pub title: Cow<'a, str>,
    /// More detailed information about the changes.
    pub body: Option<Cow<'a, str>>,
    /// If set, the git-conventional scope to help organizing changes
    pub kind: Option<git_conventional::Type<'a>>,
    /// If set, this is a breaking change as indicated git-conventional.
    pub breaking: bool,
    /// If set, this commit message body contains a specific description of the breaking change.
    pub breaking_description: Option<&'a str>,
    /// all dditional information parsed from the title.
    pub additions: Vec<message::Addition>,
}

pub struct History {
    pub head: git_repository::refs::Reference,
    pub items: Vec<history::Item>,
}

mod message;
