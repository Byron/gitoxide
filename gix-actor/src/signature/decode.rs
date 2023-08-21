pub(crate) mod function {
    use bstr::ByteSlice;
    use btoi::btoi;
    use gix_date::{time::Sign, OffsetInSeconds, SecondsSinceUnixEpoch, Time};
    use winnow::{
        combinator::{alt, separated_pair, terminated},
        error::{AddContext, ParserError, StrContext},
        prelude::*,
        stream::AsChar,
        token::{take, take_until0, take_while},
    };

    use crate::{IdentityRef, SignatureRef};

    const SPACE: &[u8] = b" ";

    /// Parse a signature from the bytes input `i` using `nom`.
    pub fn decode<'a, E: ParserError<&'a [u8]> + AddContext<&'a [u8], StrContext>>(
        i: &mut &'a [u8],
    ) -> PResult<SignatureRef<'a>, E> {
        separated_pair(
            identity,
            b" ",
            (
                terminated(take_until0(SPACE), take(1usize))
                    .verify_map(|v| btoi::<SecondsSinceUnixEpoch>(v).ok())
                    .context(StrContext::Expected("<timestamp>".into())),
                alt((
                    take_while(1.., b'-').map(|_| Sign::Minus),
                    take_while(1.., b'+').map(|_| Sign::Plus),
                ))
                .context(StrContext::Expected("+|-".into())),
                take_while(2, AsChar::is_dec_digit)
                    .verify_map(|v| btoi::<OffsetInSeconds>(v).ok())
                    .context(StrContext::Expected("HH".into())),
                take_while(1..=2, AsChar::is_dec_digit)
                    .verify_map(|v| btoi::<OffsetInSeconds>(v).ok())
                    .context(StrContext::Expected("MM".into())),
            )
                .map(|(time, sign, hours, minutes)| {
                    let offset = (hours * 3600 + minutes * 60) * if sign == Sign::Minus { -1 } else { 1 };
                    Time {
                        seconds: time,
                        offset,
                        sign,
                    }
                }),
        )
        .context(StrContext::Expected("<name> <<email>> <timestamp> <+|-><HHMM>".into()))
        .map(|(identity, time)| SignatureRef {
            name: identity.name,
            email: identity.email,
            time,
        })
        .parse_next(i)
    }

    /// Parse an identity from the bytes input `i` (like `name <email>`) using `nom`.
    pub fn identity<'a, E: ParserError<&'a [u8]> + AddContext<&'a [u8], StrContext>>(
        i: &mut &'a [u8],
    ) -> PResult<IdentityRef<'a>, E> {
        (
            terminated(take_until0(&b" <"[..]), take(2usize)).context(StrContext::Expected("<name>".into())),
            terminated(take_until0(&b">"[..]), take(1usize)).context(StrContext::Expected("<email>".into())),
        )
            .map(|(name, email): (&[u8], &[u8])| IdentityRef {
                name: name.as_bstr(),
                email: email.as_bstr(),
            })
            .context(StrContext::Expected("<name> <<email>>".into()))
            .parse_next(i)
    }
}
pub use function::identity;

#[cfg(test)]
mod tests {
    mod parse_signature {
        use bstr::ByteSlice;
        use gix_date::{time::Sign, OffsetInSeconds, SecondsSinceUnixEpoch};
        use gix_testtools::to_bstr_err;
        use winnow::prelude::*;

        use crate::{signature, SignatureRef, Time};

        fn decode<'i>(
            i: &mut &'i [u8],
        ) -> PResult<SignatureRef<'i>, winnow::error::TreeError<&'i [u8], winnow::error::StrContext>> {
            signature::decode.parse_next(i)
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
                decode
                    .parse_peek(b"Sebastian Thiel <byronimo@gmail.com> 1528473343 -0230")
                    .expect("parse to work")
                    .1,
                signature("Sebastian Thiel", "byronimo@gmail.com", 1528473343, Sign::Minus, -9000)
            );
        }

        #[test]
        fn tz_plus() {
            assert_eq!(
                decode
                    .parse_peek(b"Sebastian Thiel <byronimo@gmail.com> 1528473343 +0230")
                    .expect("parse to work")
                    .1,
                signature("Sebastian Thiel", "byronimo@gmail.com", 1528473343, Sign::Plus, 9000)
            );
        }

        #[test]
        fn negative_offset_0000() {
            assert_eq!(
                decode
                    .parse_peek(b"Sebastian Thiel <byronimo@gmail.com> 1528473343 -0000")
                    .expect("parse to work")
                    .1,
                signature("Sebastian Thiel", "byronimo@gmail.com", 1528473343, Sign::Minus, 0)
            );
        }

        #[test]
        fn negative_offset_double_dash() {
            assert_eq!(
                decode
                    .parse_peek(b"name <name@example.com> 1288373970 --700")
                    .expect("parse to work")
                    .1,
                signature("name", "name@example.com", 1288373970, Sign::Minus, -252000)
            );
        }

        #[test]
        fn empty_name_and_email() {
            assert_eq!(
                decode.parse_peek(b" <> 12345 -1215").expect("parse to work").1,
                signature("", "", 12345, Sign::Minus, -44100)
            );
        }

        #[test]
        fn invalid_signature() {
            assert_eq!(
                        decode.parse_peek(b"hello < 12345 -1215")
                            .map_err(to_bstr_err)
                            .expect_err("parse fails as > is missing")
                            .to_string(),
                        "in slice at ' 12345 -1215'\n  0: expected `<email>` at ' 12345 -1215'\n  1: expected `<name> <<email>>` at ' 12345 -1215'\n  2: expected `<name> <<email>> <timestamp> <+|-><HHMM>` at ' 12345 -1215'\n"
                    );
        }

        #[test]
        fn invalid_time() {
            assert_eq!(
                        decode.parse_peek(b"hello <> abc -1215")
                            .map_err(to_bstr_err)
                            .expect_err("parse fails as > is missing")
                            .to_string(),
                        "in predicate verification at 'abc -1215'\n  0: expected `<timestamp>` at 'abc -1215'\n  1: expected `<name> <<email>> <timestamp> <+|-><HHMM>` at 'abc -1215'\n"
                    );
        }
    }
}
