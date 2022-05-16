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

    let at_sign = BString::from("@");
    let mut cur = if *rev_str == at_sign {
        repo.rev_resolve_head()
    } else if let Some(r) = repo.rev_resolve_ref(rev_str)? {
        Ok(r)
    } else if rev_str.len() == 40 {
        // The function bellow does not what its named after
        repo.rev_find_id(rev_str.as_bstr())
    } else {
        repo.rev_find_by_prefix(rev_str.as_bstr())
    }?;

    if let Some(nav) = nav_str {
        cur = navigate(repo, cur, nav)?;
    }

    if let Some(spec) = spec_str {
        todo!("Applying specials '{}' NIY", spec);
    }
    Ok(cur)
}

fn split_parts(input: &BStr) -> Result<(&BStr, Option<&BStr>, Option<&BStr>), Error> {
    let end = input.len();

    let name = {
        let mut rev_end: usize = input.len();
        for (i, b) in input.bytes().enumerate() {
            match b {
                b':' | b'^' | b'~' => {
                    rev_end = i;
                    break;
                }
                _ => continue,
            }
        }
        if rev_end == 0 {
            return Err(Error::InvalidRev(input[0..rev_end].to_string()));
        }
        &input[0..rev_end]
    };

    let nav: Option<&BStr> = if name.len() < end {
        let rev_end = name.len();
        let tmp = &input[rev_end..end];
        let mut nav_end: usize = end;
        for (i, b) in tmp.bytes().enumerate() {
            match b {
                b':' => {
                    nav_end = rev_end + i;
                    break;
                }
                b'^' => {
                    if rev_end + i + 1 < end && input[rev_end + i + 1] == b'{' {
                        nav_end = rev_end + i;
                        break;
                    }
                }
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' | b'~' => continue,
                _ => {
                    nav_end = rev_end + i;
                    break;
                }
            }
        }
        if rev_end != nav_end {
            Some(&input[rev_end..nav_end])
        } else {
            None
        }
    } else {
        None
    };

    let nav_end = if let Some(n) = nav {
        name.len() + n.len()
    } else {
        name.len()
    };

    let spec = if nav_end < end {
        Some(&input[nav_end..end])
    } else {
        None
    };
    Ok((name, nav, spec))
}

#[cfg(test)]
mod split_parts {
    use git_object::bstr::BString;

    use super::split_parts;

    #[test]
    fn head_only() {
        let input = BString::from("HEAD");
        let expected = (input.as_ref(), None, None);
        let actual = split_parts(input.as_ref()).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn head_previous() {
        let input = BString::from("HEAD~1");

        let name = BString::from("HEAD");
        let nav = BString::from("~1");

        let expected = (name.as_ref(), Some(nav.as_ref()), None);
        let actual = split_parts(input.as_ref()).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn at_first_parent() {
        let input = BString::from("@^");

        let name = BString::from("@");
        let nav = BString::from("^");

        let expected = (name.as_ref(), Some(nav.as_ref()), None);
        let actual = split_parts(input.as_ref()).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn branch_first_parent_previous() {
        let input = BString::from("HEAD~1^1");

        let name = BString::from("HEAD");
        let nav = BString::from("~1^1");

        let expected = (name.as_ref(), Some(nav.as_ref()), None);
        let actual = split_parts(input.as_ref()).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn head_commit_type() {
        let input = BString::from("HEAD^{commit}");
        let name = BString::from("HEAD");
        let spec = BString::from("^{commit}");

        let expected = (name.as_ref(), None, Some(spec.as_ref()));
        let actual = split_parts(input.as_ref()).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn head_regex() {
        let input = BString::from("HEAD^{/fix nasty bug}");
        let name = BString::from("HEAD");
        let spec = BString::from("^{/fix nasty bug}");

        let expected = (name.as_ref(), None, Some(spec.as_ref()));
        let actual = split_parts(input.as_ref()).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn head_path() {
        let input = BString::from("HEAD:README");
        let name = BString::from("HEAD");
        let spec = BString::from(":README");

        let expected = (name.as_ref(), None, Some(spec.as_ref()));
        let actual = split_parts(input.as_ref()).unwrap();
        assert_eq!(actual, expected);
    }
}

fn navigate(repo: &impl Database, rev: ObjectId, nav_str: &BStr) -> Result<ObjectId, Error> {
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
