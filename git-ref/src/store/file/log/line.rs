use git_hash::ObjectId;

use crate::log::Line;
use crate::store::file::log::LineRef;

impl<'a> LineRef<'a> {
    /// Convert this instance into its mutable counterpart
    pub fn to_owned(&self) -> Line {
        self.clone().into()
    }
}

mod write {
    use std::io;

    use bstr::{BStr, ByteSlice};
    use quick_error::quick_error;

    use crate::log::Line;

    quick_error! {
        /// The Error produced by [`Line::write_to()`] (but wrapped in an io error).
        #[derive(Debug)]
        #[allow(missing_docs)]
        enum Error {
            IllegalCharacter {
                display("Messages must not contain newlines\\n")
            }
        }
    }

    impl From<Error> for io::Error {
        fn from(err: Error) -> Self {
            io::Error::new(io::ErrorKind::Other, err)
        }
    }

    /// Output
    impl Line {
        /// Serialize this instance to `out` in the git serialization format for ref log lines.
        pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
            write!(out, "{} {} ", self.previous_oid, self.new_oid)?;
            self.signature.write_to(&mut out)?;
            writeln!(out, "\t{}", check_newlines(self.message.as_ref())?)
        }
    }

    fn check_newlines(input: &BStr) -> Result<&BStr, Error> {
        if input.find_byte(b'\n').is_some() {
            return Err(Error::IllegalCharacter);
        }
        Ok(input)
    }
}

impl<'a> LineRef<'a> {
    /// The previous object id of the ref. It will be a null hash if there was no previous id as
    /// this ref is being created.
    pub fn previous_oid(&self) -> ObjectId {
        ObjectId::from_hex(self.previous_oid).expect("parse validation")
    }
    /// The new object id of the ref, or a null hash if it is removed.
    pub fn new_oid(&self) -> ObjectId {
        ObjectId::from_hex(self.new_oid).expect("parse validation")
    }
}

impl<'a> From<LineRef<'a>> for Line {
    fn from(v: LineRef<'a>) -> Self {
        Line {
            previous_oid: v.previous_oid(),
            new_oid: v.new_oid(),
            signature: v.signature.into(),
            message: v.message.into(),
        }
    }
}

///
pub mod decode {
    use bstr::{BStr, ByteSlice};
    use nom::{
        bytes::complete::{tag, take_while},
        combinator::opt,
        error::{context, ContextError, ParseError},
        sequence::{terminated, tuple},
        IResult,
    };

    use crate::{file::log::LineRef, parse::hex_hash};

    ///
    mod error {
        use bstr::{BString, ByteSlice};

        /// The error returned by [from_bytes(…)][super::Line::from_bytes()]
        #[derive(Debug)]
        pub struct Error {
            pub input: BString,
        }

        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{:?} did not match '<old-hexsha> <new-hexsha> <name> <<email>> <timestamp> <tz>\\t<message>'",
                    self.input
                )
            }
        }

        impl<'a> std::error::Error for Error {}

        impl Error {
            pub(crate) fn new(input: &[u8]) -> Self {
                Error {
                    input: input.as_bstr().to_owned(),
                }
            }
        }
    }
    pub use error::Error;

    impl<'a> LineRef<'a> {
        /// Decode a line from the given bytes which are expected to start at a hex sha.
        pub fn from_bytes(input: &'a [u8]) -> Result<LineRef<'a>, Error> {
            one::<()>(input).map(|(_, l)| l).map_err(|_| Error::new(input))
        }
    }

    fn message<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], &'a BStr, E> {
        if i.is_empty() {
            Ok((&[], i.as_bstr()))
        } else {
            terminated(take_while(|c| c != b'\n'), opt(tag(b"\n")))(i).map(|(i, o)| (i, o.as_bstr()))
        }
    }

    fn one<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(bytes: &'a [u8]) -> IResult<&[u8], LineRef<'a>, E> {
        let (i, (old, new, signature, message_sep, message)) = context(
            "<old-hexsha> <new-hexsha> <name> <<email>> <timestamp> <tz>\\t<message>",
            tuple((
                context("<old-hexsha>", terminated(hex_hash, tag(b" "))),
                context("<new-hexsha>", terminated(hex_hash, tag(b" "))),
                context("<name> <<email>> <timestamp>", git_actor::signature::decode),
                opt(tag(b"\t")),
                context("<optional message>", message),
            )),
        )(bytes)?;

        if message_sep.is_none() {
            if let Some(first) = message.first() {
                if !first.is_ascii_whitespace() {
                    return Err(nom::Err::Error(E::add_context(
                        i,
                        "log message must be separated from signature with whitespace",
                        E::from_error_kind(i, nom::error::ErrorKind::MapRes),
                    )));
                }
            }
        }

        Ok((
            i,
            LineRef {
                previous_oid: old,
                new_oid: new,
                signature,
                message,
            },
        ))
    }

    #[cfg(test)]
    mod test {
        use bstr::ByteSlice;
        use git_actor::{Sign, Time};
        use git_hash::ObjectId;

        use super::*;

        fn hex_to_oid(hex: &str) -> ObjectId {
            ObjectId::from_hex(hex.as_bytes()).unwrap()
        }

        fn with_newline(mut v: Vec<u8>) -> Vec<u8> {
            v.push(b'\n');
            v
        }

        mod invalid {
            use git_testtools::to_bstr_err;
            use nom::error::VerboseError;

            use super::one;

            #[test]
            fn completely_bogus_shows_error_with_context() {
                let err = one::<VerboseError<&[u8]>>(b"definitely not a log entry")
                    .map_err(to_bstr_err)
                    .expect_err("this should fail");
                assert!(err.to_string().contains("<old-hexsha> <new-hexsha>"));
            }

            #[test]
            fn missing_whitespace_between_signature_and_message() {
                let line = "0000000000000000000000000000000000000000 0000000000000000000000000000000000000000 one <foo@example.com> 1234567890 -0000message";
                let err = one::<VerboseError<&[u8]>>(line.as_bytes())
                    .map_err(to_bstr_err)
                    .expect_err("this should fail");
                assert!(err
                    .to_string()
                    .contains("log message must be separated from signature with whitespace"));
            }
        }

        const NULL_SHA1: &[u8] = b"0000000000000000000000000000000000000000";

        #[test]
        fn entry_with_empty_message() {
            let line_without_nl: Vec<_> = b"0000000000000000000000000000000000000000 0000000000000000000000000000000000000000 name <foo@example.com> 1234567890 -0000".to_vec();
            let line_with_nl = with_newline(line_without_nl.clone());
            for input in &[line_without_nl, line_with_nl] {
                assert_eq!(
                    one::<nom::error::Error<_>>(input).expect("successful parsing").1,
                    LineRef {
                        previous_oid: NULL_SHA1.as_bstr(),
                        new_oid: NULL_SHA1.as_bstr(),
                        signature: git_actor::SignatureRef {
                            name: b"name".as_bstr(),
                            email: b"foo@example.com".as_bstr(),
                            time: Time {
                                time: 1234567890,
                                offset: 0,
                                sign: Sign::Minus
                            }
                        },
                        message: b"".as_bstr(),
                    }
                );
            }
        }

        #[test]
        fn entry_with_message_without_newline_and_with_newline() {
            let line_without_nl: Vec<_> = b"a5828ae6b52137b913b978e16cd2334482eb4c1f 89b43f80a514aee58b662ad606e6352e03eaeee4 Sebastian Thiel <foo@example.com> 1618030561 +0800\tpull --ff-only: Fast-forward".to_vec();
            let line_with_nl = with_newline(line_without_nl.clone());

            for input in &[line_without_nl, line_with_nl] {
                let (remaining, res) = one::<nom::error::Error<_>>(input).expect("successful parsing");
                assert!(remaining.is_empty(), "all consuming even without trailing newline");
                let actual = LineRef {
                    previous_oid: b"a5828ae6b52137b913b978e16cd2334482eb4c1f".as_bstr(),
                    new_oid: b"89b43f80a514aee58b662ad606e6352e03eaeee4".as_bstr(),
                    signature: git_actor::SignatureRef {
                        name: b"Sebastian Thiel".as_bstr(),
                        email: b"foo@example.com".as_bstr(),
                        time: Time {
                            time: 1618030561,
                            offset: 28800,
                            sign: Sign::Plus,
                        },
                    },
                    message: b"pull --ff-only: Fast-forward".as_bstr(),
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
            let (remainder, parsed) = one::<nom::error::Error<_>>(lines).expect("parse single line");
            assert_eq!(parsed.message, b"".as_bstr(), "first message is empty");

            let (remainder, parsed) = one::<nom::error::Error<_>>(remainder).expect("parse single line");
            assert_eq!(
                parsed.message,
                b"hello".as_bstr(),
                "second message is not and contains no newline"
            );
            assert!(remainder.is_empty());
        }
    }
}
