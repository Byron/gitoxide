use git_repository as git;
use git_repository::bstr::{BStr, ByteSlice};

use crate::command::changelog_impl::commit::Message;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Addition<'a> {
    /// The plain issue ID, like "123".
    IssueId(&'a str),
}

mod additions {
    use std::borrow::Cow;

    use crate::command::changelog_impl::commit::message::Addition;

    fn strip(title: &str) -> (Cow<'_, str>, Vec<Addition<'_>>) {
        todo!("strip")
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[ignore]
        fn no_addition() {
            let (nt, a) = strip("hello there [abc] (abc)");
            assert_eq!(nt, "hello there [abc] (abc)");
            assert_eq!(a, vec![]);
        }

        #[test]
        #[ignore]
        fn strip_trailing_issue_number() {
            let (nt, a) = strip("hello (#1)");
            assert_eq!(nt, "hello");
            assert_eq!(a, vec![Addition::IssueId("1")]);
        }
    }
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

impl<'a> From<&'a str> for Message<'a> {
    fn from(m: &'a str) -> Self {
        let (title, kind, body, breaking, breaking_description) = git_conventional::Commit::parse(m)
            .map(|c: git_conventional::Commit| {
                (
                    c.description().into(),
                    Some(c.type_()),
                    c.body().map(Into::into),
                    c.breaking(),
                    c.breaking_description().map(Into::into).and_then(|d| {
                        if d == c.description() {
                            None
                        } else {
                            Some(d)
                        }
                    }),
                )
            })
            .unwrap_or_else(|_| {
                let m = git::objs::commit::MessageRef::from_bytes(m.as_bytes());
                (
                    m.summary().as_ref().to_string().into(),
                    None,
                    m.body().map(|b| b.without_trailer().to_str_lossy()),
                    false,
                    None,
                )
            });
        Message {
            title,
            kind,
            body,
            breaking,
            breaking_description,
            additions: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_conventional_no_additions_no_body() {
        assert_eq!(
            Message::from("hi"),
            Message {
                title: "hi".into(),
                body: None,
                kind: None,
                breaking: false,
                breaking_description: None,
                additions: vec![]
            }
        )
    }

    #[test]
    fn no_conventional_uses_summary() {
        assert_eq!(
            Message::from("hi\nho\nfoo\n\nbody"),
            Message {
                title: "hi ho foo".into(),
                body: Some("body".into()),
                kind: None,
                breaking: false,
                breaking_description: None,
                additions: vec![]
            }
        )
    }

    #[test]
    fn no_conventional_no_additions() {
        assert_eq!(
            Message::from("hi\n\nbody\nother\n\nSigned: bar"),
            Message {
                title: "hi".into(),
                body: Some("body\nother".into()),
                kind: None,
                breaking: false,
                breaking_description: None,
                additions: vec![]
            }
        )
    }

    #[test]
    fn conventional_no_additions() {
        assert_eq!(
            Message::from("feat!: hi\n\nthe body\nBREAKING-CHANGE: breaks\n\nSigned: foobar"),
            Message {
                title: "hi".into(),
                body: Some("the body".into()),
                kind: Some(git_conventional::Type::new_unchecked("feat")),
                breaking: true,
                breaking_description: Some("breaks".into()),
                additions: vec![]
            }
        )
    }
}
