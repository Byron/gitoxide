#![allow(missing_docs, unused)]

use bstr::BStr;
use git_hash::ObjectId;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Line<'a> {
    pub previous_oid: ObjectId,
    pub new_oid: ObjectId,
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub signature: git_actor::immutable::Signature<'a>,
    pub message: &'a BStr,
}

mod decode {
    use crate::{file::log::Line, parse::hex_sha1};

    use bstr::{BStr, ByteSlice};
    use git_hash::ObjectId;
    use nom::{
        bytes::{
            complete::tag,
            complete::{take_until, take_while},
        },
        combinator::opt,
        combinator::{map, map_res},
        error::{context, FromExternalError},
        error::{ContextError, ParseError},
        sequence::{terminated, tuple},
        IResult,
    };

    fn parse_message<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], &'a BStr, E> {
        if i.is_empty() {
            Ok((&[], i.as_bstr()))
        } else {
            terminated(take_while(|c| c != b'\n'), opt(tag(b"\n")))(i).map(|(i, o)| (i, o.as_bstr()))
        }
    }

    pub fn line<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(bytes: &'a [u8]) -> IResult<&[u8], Line<'a>, E> {
        map(
            context(
                "<old-hexsha> <new-hexsha> <name> <<email>> <timestamp> <tz>\\t<message>",
                tuple((
                    hex_sha1,
                    tag(b" "),
                    hex_sha1,
                    tag(b" "),
                    |i| {
                        git_actor::immutable::signature::decode(i)
                            .map_err(|e| nom::Err::Error(E::from_error_kind(i, nom::error::ErrorKind::MapRes)))
                    },
                    opt(tag(b"\t")),
                    parse_message,
                )),
            ),
            |(old, _, new, _, signature, _, message)| Line {
                previous_oid: ObjectId::from_hex(old).expect("parser validation"),
                new_oid: ObjectId::from_hex(new).expect("parser validation"),
                signature,
                message,
            },
        )(bytes)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use bstr::ByteSlice;
        use git_actor::{Sign, Time};
        use git_hash::ObjectId;
        use nom::error::VerboseError;

        fn hex_to_oid(hex: &str) -> ObjectId {
            ObjectId::from_hex(hex.as_bytes()).unwrap()
        }

        fn with_newline(mut v: Vec<u8>) -> Vec<u8> {
            v.push(b'\n');
            v
        }

        #[test]
        #[should_panic]
        fn completely_bogus_shows_error_with_context() {
            assert_eq!(
                line::<VerboseError<&[u8]>>(b"definitely not a log entry")
                    .expect_err("this should fail")
                    // .map(|err| err.to_string())
                    .to_string(),
                "hello"
            );
        }

        #[test]
        fn entry_with_empty_message() {
            let line_without_nl: Vec<_> = b"0000000000000000000000000000000000000000 0000000000000000000000000000000000000000 name <foo@example.com> 1234567890 -0000".to_vec();
            let line_with_nl = with_newline(line_without_nl.clone());
            for input in &[line_without_nl, line_with_nl] {
                assert_eq!(
                    line::<nom::error::Error<_>>(input).expect("successful parsing").1,
                    Line {
                        previous_oid: ObjectId::null_sha1(),
                        new_oid: ObjectId::null_sha1(),
                        signature: git_actor::immutable::Signature {
                            name: b"name".as_bstr(),
                            email: b"foo@example.com".as_bstr(),
                            time: Time {
                                time: 1234567890,
                                offset: 0,
                                sign: Sign::Minus
                            }
                        },
                        message: b"".as_bstr()
                    }
                );
            }
        }

        #[test]
        fn entry_with_message_without_newline_and_with_newline() {
            let line_without_nl: Vec<_>= b"a5828ae6b52137b913b978e16cd2334482eb4c1f 89b43f80a514aee58b662ad606e6352e03eaeee4 Sebastian Thiel <foo@example.com> 1618030561 +0800\tpull --ff-only: Fast-forward".to_vec();
            let line_with_nl = with_newline(line_without_nl.clone());

            for input in &[line_without_nl, line_with_nl] {
                let (remaining, res) = line::<nom::error::Error<_>>(&input).expect("successful parsing");
                assert!(remaining.is_empty(), "all consuming even without trailing newline");
                assert_eq!(
                    res,
                    Line {
                        previous_oid: hex_to_oid("a5828ae6b52137b913b978e16cd2334482eb4c1f"),
                        new_oid: hex_to_oid("89b43f80a514aee58b662ad606e6352e03eaeee4"),
                        signature: git_actor::immutable::Signature {
                            name: b"Sebastian Thiel".as_bstr(),
                            email: b"foo@example.com".as_bstr(),
                            time: Time {
                                time: 1618030561,
                                offset: 28800,
                                sign: Sign::Plus
                            }
                        },
                        message: b"pull --ff-only: Fast-forward".as_bstr()
                    }
                );
            }
        }
        #[test]
        fn two_lines_in_a_row_with_and_without_newline() {
            let lines = b"0000000000000000000000000000000000000000 0000000000000000000000000000000000000000 one <foo@example.com> 1234567890 -0000\t\n0000000000000000000000000000000000000000 0000000000000000000000000000000000000000 two <foo@example.com> 1234567890 -0000\thello";
            let (remainder, parsed) = line::<nom::error::Error<_>>(lines).expect("parse single line");
            assert_eq!(parsed.message, b"".as_bstr(), "first message is empty");

            let (remainder, parsed) = line::<nom::error::Error<_>>(remainder).expect("parse single line");
            assert_eq!(
                parsed.message,
                b"hello".as_bstr(),
                "second message is not and contains no newline"
            );
            assert!(remainder.is_empty());
        }
    }
}
