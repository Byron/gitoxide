pub(crate) mod function {
    use bstr::ByteSlice;
    use btoi::btoi;
    use gix_date::{time::Sign, OffsetInSeconds, SecondsSinceUnixEpoch, Time};
    use nom::{
        branch::alt,
        bytes::complete::{tag, take, take_until, take_while_m_n},
        character::is_digit,
        error::{context, ContextError, ParseError},
        sequence::{terminated, tuple},
        IResult,
    };

    use crate::{IdentityRef, SignatureRef};

    const SPACE: &[u8] = b" ";

    /// Parse a signature from the bytes input `i` using `nom`.
    pub fn decode<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        i: &'a [u8],
    ) -> IResult<&'a [u8], SignatureRef<'a>, E> {
        let (i, (identity, _, time, tzsign, hours, minutes)) = context(
            "<name> <<email>> <timestamp> <+|-><HHMM>",
            tuple((
                identity,
                tag(b" "),
                context("<timestamp>", |i| {
                    terminated(take_until(SPACE), take(1usize))(i).and_then(|(i, v)| {
                        btoi::<SecondsSinceUnixEpoch>(v)
                            .map(|v| (i, v))
                            .map_err(|_| nom::Err::Error(E::from_error_kind(i, nom::error::ErrorKind::MapRes)))
                    })
                }),
                context("+|-", alt((tag(b"-"), tag(b"+")))),
                context("HH", |i| {
                    take_while_m_n(2usize, 2, is_digit)(i).and_then(|(i, v)| {
                        btoi::<OffsetInSeconds>(v)
                            .map(|v| (i, v))
                            .map_err(|_| nom::Err::Error(E::from_error_kind(i, nom::error::ErrorKind::MapRes)))
                    })
                }),
                context("MM", |i| {
                    take_while_m_n(2usize, 2, is_digit)(i).and_then(|(i, v)| {
                        btoi::<OffsetInSeconds>(v)
                            .map(|v| (i, v))
                            .map_err(|_| nom::Err::Error(E::from_error_kind(i, nom::error::ErrorKind::MapRes)))
                    })
                }),
            )),
        )(i)?;

        debug_assert!(tzsign[0] == b'-' || tzsign[0] == b'+', "parser assure it's +|- only");
        let sign = if tzsign[0] == b'-' { Sign::Minus } else { Sign::Plus }; //
        let offset = (hours * 3600 + minutes * 60) * if sign == Sign::Minus { -1 } else { 1 };

        Ok((
            i,
            SignatureRef {
                name: identity.name,
                email: identity.email,
                time: Time {
                    seconds: time,
                    offset,
                    sign,
                },
            },
        ))
    }

    /// Parse an identity from the bytes input `i` (like `name <email>`) using `nom`.
    pub fn identity<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        i: &'a [u8],
    ) -> IResult<&'a [u8], IdentityRef<'a>, E> {
        let (i, (name, email)) = context(
            "<name> <<email>>",
            tuple((
                context("<name>", terminated(take_until(&b" <"[..]), take(2usize))),
                context("<email>", terminated(take_until(&b">"[..]), take(1usize))),
            )),
        )(i)?;

        Ok((
            i,
            IdentityRef {
                name: name.as_bstr(),
                email: email.as_bstr(),
            },
        ))
    }
}
pub use function::identity;

#[cfg(test)]
mod tests {
    mod parse_signature {
        use bstr::ByteSlice;
        use gix_date::{time::Sign, OffsetInSeconds, SecondsSinceUnixEpoch};
        use gix_testtools::to_bstr_err;
        use nom::IResult;

        use crate::{signature, SignatureRef, Time};

        fn decode(i: &[u8]) -> IResult<&[u8], SignatureRef<'_>, nom::error::VerboseError<&[u8]>> {
            signature::decode(i)
        }

        fn signature(
            name: &'static str,
            email: &'static str,
            seconds: SecondsSinceUnixEpoch,
            sign: Sign,
            offset: OffsetInSeconds,
        ) -> SignatureRef<'static> {
            SignatureRef {
                name: name.as_bytes().as_bstr(),
                email: email.as_bytes().as_bstr(),
                time: Time { seconds, offset, sign },
            }
        }

        #[test]
        fn tz_minus() {
            assert_eq!(
                decode(b"Sebastian Thiel <byronimo@gmail.com> 1528473343 -0230")
                    .expect("parse to work")
                    .1,
                signature("Sebastian Thiel", "byronimo@gmail.com", 1528473343, Sign::Minus, -9000)
            );
        }

        #[test]
        fn tz_plus() {
            assert_eq!(
                decode(b"Sebastian Thiel <byronimo@gmail.com> 1528473343 +0230")
                    .expect("parse to work")
                    .1,
                signature("Sebastian Thiel", "byronimo@gmail.com", 1528473343, Sign::Plus, 9000)
            );
        }

        #[test]
        fn negative_offset_0000() {
            assert_eq!(
                decode(b"Sebastian Thiel <byronimo@gmail.com> 1528473343 -0000")
                    .expect("parse to work")
                    .1,
                signature("Sebastian Thiel", "byronimo@gmail.com", 1528473343, Sign::Minus, 0)
            );
        }

        #[test]
        fn empty_name_and_email() {
            assert_eq!(
                decode(b" <> 12345 -1215").expect("parse to work").1,
                signature("", "", 12345, Sign::Minus, -44100)
            );
        }

        #[test]
        fn invalid_signature() {
            assert_eq!(
                        decode(b"hello < 12345 -1215")
                            .map_err(to_bstr_err)
                            .expect_err("parse fails as > is missing")
                            .to_string(),
                        "Parse error:\nTakeUntil at:  12345 -1215\nin section '<email>', at:  12345 -1215\nin section '<name> <<email>>', at: hello < 12345 -1215\nin section '<name> <<email>> <timestamp> <+|-><HHMM>', at: hello < 12345 -1215\n"
                    );
        }

        #[test]
        fn invalid_time() {
            assert_eq!(
                        decode(b"hello <> abc -1215")
                            .map_err(to_bstr_err)
                            .expect_err("parse fails as > is missing")
                            .to_string(),
                        "Parse error:\nMapRes at: -1215\nin section '<timestamp>', at: abc -1215\nin section '<name> <<email>> <timestamp> <+|-><HHMM>', at: hello <> abc -1215\n"
                    );
        }
    }
}
