use crate::store::file::log::Line;
use git_hash::ObjectId;

impl<'a> Line<'a> {
    /// The previous object id of the ref. It will be a null hash if there was no previous id as
    /// this ref is being created.
    pub fn previous_oid(&self) -> ObjectId {
        ObjectId::from_hex(&self.previous_oid).expect("parse validation")
    }
    /// The new object id of the ref, or a null hash if it is removed.
    pub fn new_oid(&self) -> ObjectId {
        ObjectId::from_hex(&self.new_oid).expect("parse validation")
    }
}

mod decode {
    #![allow(unused)]
    use crate::{file::log::Line, parse::hex_sha1};

    use bstr::{BStr, ByteSlice};
    use nom::{
        bytes::{complete::tag, complete::take_while},
        combinator::{map, opt},
        error::{context, ContextError, ParseError},
        sequence::{preceded, terminated, tuple},
        IResult,
    };

    fn message<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], &'a BStr, E> {
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
                    context("<old-hexsha>", terminated(hex_sha1, tag(b" "))),
                    context("<new-hexsha>", terminated(hex_sha1, tag(b" "))),
                    context("<name> <<email>> <timestamp>", git_actor::immutable::signature::decode),
                    context("<optional message>", preceded(opt(tag(b"\t")), message)),
                )),
            ),
            |(old, new, signature, message)| Line {
                previous_oid: old.as_bstr(),
                new_oid: new.as_bstr(),
                signature,
                message,
                _prevent_initialization: (),
            },
        )(bytes)
    }

    #[cfg(test)]
    mod test {
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

        mod invalid {
            use super::line;

            use bstr::{BStr, ByteSlice};
            use nom::error::VerboseError;

            fn to_bstr_err(err: VerboseError<&[u8]>) -> VerboseError<&BStr> {
                VerboseError {
                    errors: err.errors.into_iter().map(|(i, v)| (i.as_bstr(), v)).collect(),
                }
            }

            #[test]
            fn completely_bogus_shows_error_with_context() {
                let err = line::<VerboseError<&[u8]>>(b"definitely not a log entry")
                    .expect_err("this should fail")
                    .map(|e| to_bstr_err(e).to_string());
                assert!(err.to_string().contains("<old-hexsha> <new-hexsha>"));
            }
        }

        const NULL_SHA1: &[u8] = b"0000000000000000000000000000000000000000";

        #[test]
        fn entry_with_empty_message() {
            let line_without_nl: Vec<_> = b"0000000000000000000000000000000000000000 0000000000000000000000000000000000000000 name <foo@example.com> 1234567890 -0000".to_vec();
            let line_with_nl = with_newline(line_without_nl.clone());
            for input in &[line_without_nl, line_with_nl] {
                assert_eq!(
                    line::<nom::error::Error<_>>(input).expect("successful parsing").1,
                    Line {
                        previous_oid: NULL_SHA1.as_bstr(),
                        new_oid: NULL_SHA1.as_bstr(),
                        signature: git_actor::immutable::Signature {
                            name: b"name".as_bstr(),
                            email: b"foo@example.com".as_bstr(),
                            time: Time {
                                time: 1234567890,
                                offset: 0,
                                sign: Sign::Minus
                            }
                        },
                        message: b"".as_bstr(),
                        _prevent_initialization: ()
                    }
                );
            }
        }

        #[test]
        fn entry_with_message_without_newline_and_with_newline() {
            let line_without_nl: Vec<_> = b"a5828ae6b52137b913b978e16cd2334482eb4c1f 89b43f80a514aee58b662ad606e6352e03eaeee4 Sebastian Thiel <foo@example.com> 1618030561 +0800\tpull --ff-only: Fast-forward".to_vec();
            let line_with_nl = with_newline(line_without_nl.clone());

            for input in &[line_without_nl, line_with_nl] {
                let (remaining, res) = line::<nom::error::Error<_>>(&input).expect("successful parsing");
                assert!(remaining.is_empty(), "all consuming even without trailing newline");
                let actual = Line {
                    previous_oid: b"a5828ae6b52137b913b978e16cd2334482eb4c1f".as_bstr(),
                    new_oid: b"89b43f80a514aee58b662ad606e6352e03eaeee4".as_bstr(),
                    signature: git_actor::immutable::Signature {
                        name: b"Sebastian Thiel".as_bstr(),
                        email: b"foo@example.com".as_bstr(),
                        time: Time {
                            time: 1618030561,
                            offset: 28800,
                            sign: Sign::Plus,
                        },
                    },
                    message: b"pull --ff-only: Fast-forward".as_bstr(),
                    _prevent_initialization: (),
                };
                assert_eq!(res, actual);
                assert_eq!(
                    actual.previous_oid(),
                    hex_to_oid("a5828ae6b52137b913b978e16cd2334482eb4c1f")
                );
                assert_eq!(actual.new_oid(), hex_to_oid("89b43f80a514aee58b662ad606e6352e03eaeee4"));
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
