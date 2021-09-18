#![allow(unused)]

use std::borrow::Cow;

pub mod history;

pub struct Message<'a> {
    /// The cleared, plain title with any `additions` removed.
    title: Cow<'a, str>,
    /// More detailed information about the changes.
    body: &'a str,
    /// If set, this is a breaking change as indicated git-conventional.
    breaking: bool,
    /// If set, this commit message body contains a specific description of the breaking change.
    breaking_description: Option<&'a str>,
    /// all dditional information parsed from the title.
    additions: message::Addition<'a>,
}

pub struct History {
    pub head: git_repository::refs::Reference,
    pub items: Vec<history::Item>,
}

mod message {
    use git_repository::bstr::BStr;

    pub enum Addition<'a> {
        /// The plain issue ID, like "123".
        IssueId(Option<&'a BStr>),
    }

    mod decode {
        use nom::{
            error::{ContextError, ParseError},
            IResult,
        };

        use crate::command::changelog_impl::commit::message;

        /// Parse a signature from the bytes input `i` using `nom`.
        pub fn _decode_description<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
            _i: &'a [u8],
        ) -> IResult<&'a [u8], message::Addition<'a>, E> {
            todo!("probably not to be done")
        }
    }
}
