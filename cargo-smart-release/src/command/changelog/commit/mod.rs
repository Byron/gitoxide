#![allow(unused)]
pub mod history;

pub struct Message<'a> {
    conventional: Option<git_conventional::Commit<'a>>,
    unconventional: &'a str,
    additions: message::Additions<'a>,
}

mod message {
    use git_repository::bstr::BStr;

    pub struct Additions<'a> {
        issue_id: Option<&'a BStr>,
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
        ) -> IResult<&'a [u8], message::Additions<'a>, E> {
            todo!("probably not to be done")
        }
    }
}

pub struct History {
    pub head: git_repository::refs::Reference,
    pub items: Vec<history::Item>,
}
