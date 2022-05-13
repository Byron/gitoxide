#![allow(missing_docs)]
use std::iter::Peekable;

use git_hash::prefix;
use git_hash::ObjectId;
use git_object::bstr::{BStr, BString, ByteSlice};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to resolve HEAD to an id")]
    Head,
    #[error("Error during id lookup")]
    IdLookUp,
    #[error("Short id matched multiple objects")]
    IdMultiMatch,
    #[error("Failed to resolve ref '{0}' to an id")]
    RefLookUp(String),
    #[error("Invalid hex input '{0}'")]
    InvalidHex(String),
    #[error(transparent)]
    InvalidShortId(#[from] prefix::from_hex::Error),
    #[error("Invalid navigation string: {0}")]
    InvalidNavigation(String),
    #[error("Invalid rev string: {0}")]
    InvalidRev(String),
    #[error("{0}")]
    Other(String),
}

pub trait Database {
    fn rev_resolve_head(&self) -> Result<ObjectId, Error>;
    fn rev_nth_ancestor(&self, id: ObjectId, n: usize) -> Result<ObjectId, Error>;
    fn rev_nth_parent(&self, id: ObjectId, n: usize) -> Result<ObjectId, Error>;
    fn rev_resolve_ref(&self, input: &BStr) -> Result<Option<ObjectId>, Error>;
    fn rev_find_id(&self, input: &BStr) -> Result<ObjectId, Error>;
    fn rev_find_by_prefix(&self, input: &BStr) -> Result<ObjectId, Error>;
}

pub fn rev_parse(repo: &impl Database, input: &BStr) -> Result<ObjectId, Error> {
    if input.is_empty() || input == "@" || input == "HEAD" {
        return repo.rev_resolve_head();
    }

    if input.contains_str("@{") {
        todo!("NIY @{{ handling")
    }

    if input.starts_with(b":/") {
        todo!("NIY Regex search")
    }

    let (rev_str, nav_str, spec_str) = split_parts(input)?;

    let rev = if rev_str.is_empty() {
        Err(Error::InvalidRev(rev_str.to_string()))
    } else if rev_str == *"@" {
        repo.rev_resolve_head()
    } else if let Some(r) = repo.rev_resolve_ref(rev_str.as_bstr())? {
        Ok(r)
    } else if rev_str.len() == 40 {
        // The function bellow does not what its named after
        repo.rev_find_id(rev_str.as_bstr())
    } else {
        repo.rev_find_by_prefix(rev_str.as_bstr())
    }?;

    if nav_str.is_empty() && spec_str.is_empty() {
        Ok(rev)
    } else if !nav_str.is_empty() && spec_str.is_empty() {
        Ok(navigate(repo, rev, &nav_str)?)
    } else {
        todo!("Applying specials NIY");
    }
}

fn split_parts(input: &BStr) -> Result<(BString, BString, BString), Error> {
    #[derive(Copy, Clone, Debug)]
    enum State {
        Rev,
        Nav,
        Spec,
    }

    let mut rev = BString::default();
    let mut nav = BString::default();
    let mut spec = BString::default();

    let mut state = State::Rev;
    let mut it = input.bytes().peekable();
    while let Some(c) = it.next() {
        let next = match c {
            b'^' => match it.peek() {
                Some(b'{') => State::Spec,
                Some(b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' | b'^' | b'~') | None => {
                    State::Nav
                }
                _ => return Err(Error::InvalidNavigation(input.to_string())),
            },
            b'~' => State::Nav,
            _ => state,
        };

        match (state, next) {
            // valid transitions
            (State::Rev, State::Nav | State::Spec) | (State::Nav, State::Spec) => state = next,

            // No transitions
            (State::Rev, State::Rev) | (State::Nav, State::Nav) | (State::Spec, State::Spec) => {}

            // Invalid
            (State::Nav | State::Spec, State::Rev) | (State::Spec, State::Nav) => {
                panic!("Invalid transition {:?} / {:?}", state, next);
            }
        }
        match (state, next) {
            // valid transitions
            (State::Rev, State::Rev) => rev.push(c),
            (State::Rev | State::Nav, State::Nav) => nav.push(c),
            (State::Rev | State::Nav | State::Spec, State::Spec) => spec.push(c),

            // Invalid
            (State::Nav | State::Spec, State::Rev) | (State::Spec, State::Nav) => {
                panic!("Invalid transition {:?} / {:?}", state, next);
            }
        }
    }
    Ok((rev, nav, spec))
}

fn navigate(repo: &impl Database, rev: ObjectId, nav_str: &BString) -> Result<ObjectId, Error> {
    let mut cur = rev;
    let mut it = nav_str.chars().peekable();
    while let Some(c) = it.next() {
        if c == '^' {
            let length = parse_counter(&mut it);
            cur = repo.rev_nth_parent(cur, length)?;
        } else if c == '~' {
            let n = parse_counter(&mut it);
            cur = repo.rev_nth_ancestor(cur, n)?;
        } else {
            return Err(Error::InvalidNavigation(nav_str.to_string()));
        }
    }
    Ok(cur)
}

fn parse_counter(it: &mut Peekable<git_object::bstr::Chars<'_>>) -> usize {
    let mut buf = String::default();
    while let Some(next) = it.peek() {
        if !next.is_digit(10) {
            break;
        }
        buf.push(it.next().expect("Should be some"));
    }
    if buf.is_empty() {
        1
    } else {
        buf.parse().unwrap()
    }
}
