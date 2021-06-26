use btoi::btoi;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until, take_while_m_n},
    character::is_digit,
    error::context,
    sequence::{terminated, tuple},
    IResult,
};

use crate::{
    immutable::{signature::decode, Signature},
    Sign, Time,
};
use bstr::ByteSlice;

pub(crate) const SPACE: &[u8] = b" ";

pub(crate) fn signature(i: &[u8]) -> IResult<&[u8], Signature<'_>, decode::Error> {
    let (i, (name, email, time_in_seconds, tzsign, tzhour, tzminute)) = context(
        "<name> <<email>> <time seconds since epoch> <+|-><HHMM>",
        tuple((
            terminated(take_until(&b" <"[..]), take(2usize)),
            terminated(take_until(&b"> "[..]), take(2usize)),
            terminated(take_until(SPACE), take(1usize)),
            alt((tag(b"-"), tag(b"+"))),
            take_while_m_n(2usize, 2, is_digit),
            take_while_m_n(2usize, 2, is_digit),
        )),
    )(i)?;

    let sign = if tzsign[0] == b'-' { Sign::Minus } else { Sign::Plus };
    let hours = btoi::<i32>(&tzhour).map_err(|e| {
        nom::Err::Error(decode::Error::ParseIntegerError(
            "invalid 'hours' string",
            tzhour.into(),
            e,
        ))
    })?;
    let minutes = btoi::<i32>(&tzminute).map_err(|e| {
        nom::Err::Error(decode::Error::ParseIntegerError(
            "invalid 'minutes' string",
            tzminute.into(),
            e,
        ))
    })?;
    let offset = (hours * 3600 + minutes * 60) * if sign == Sign::Minus { -1 } else { 1 };

    Ok((
        i,
        Signature {
            name: name.as_bstr(),
            email: email.as_bstr(),
            time: Time {
                time: btoi::<u32>(time_in_seconds).map_err(|e| {
                    nom::Err::Error(decode::Error::ParseIntegerError(
                        "Could parse to seconds",
                        time_in_seconds.into(),
                        e,
                    ))
                })?,
                offset,
                sign,
            },
        },
    ))
}

#[cfg(test)]
mod tests {
    mod parse_signature {
        use crate::{
            immutable::{decode, signature, Signature},
            Sign, Time,
        };
        use bstr::ByteSlice;

        fn signature(
            name: &'static str,
            email: &'static str,
            time: u32,
            sign: Sign,
            offset: i32,
        ) -> Signature<'static> {
            Signature {
                name: name.as_bytes().as_bstr(),
                email: email.as_bytes().as_bstr(),
                time: Time { time, offset, sign },
            }
        }

        #[test]
        fn tz_minus() {
            assert_eq!(
                decode::signature(b"Sebastian Thiel <byronimo@gmail.com> 1528473343 -0230")
                    .expect("parse to work")
                    .1,
                signature("Sebastian Thiel", "byronimo@gmail.com", 1528473343, Sign::Minus, -9000)
            );
        }

        #[test]
        fn tz_plus() {
            assert_eq!(
                decode::signature(b"Sebastian Thiel <byronimo@gmail.com> 1528473343 +0230")
                    .expect("parse to work")
                    .1,
                signature("Sebastian Thiel", "byronimo@gmail.com", 1528473343, Sign::Plus, 9000)
            );
        }

        #[test]
        fn negative_offset_0000() {
            assert_eq!(
                decode::signature(b"Sebastian Thiel <byronimo@gmail.com> 1528473343 -0000")
                    .expect("parse to work")
                    .1,
                signature("Sebastian Thiel", "byronimo@gmail.com", 1528473343, Sign::Minus, 0)
            );
        }

        #[test]
        fn empty_name_and_email() {
            assert_eq!(
                decode::signature(b" <> 12345 -1215").expect("parse to work").1,
                signature("", "", 12345, Sign::Minus, -44100)
            );
        }

        #[test]
        fn invalid_signature() {
            assert_eq!(
                signature::decode::Error::from(
                    decode::signature(b"hello < 12345 -1215").expect_err("parse fails as > is missing")
                )
                .to_string(),
                r##""hello < 12345 -1215" did not match '<name> <<email>> <time seconds since epoch> <+|-><HHMM>'"##
            );
        }

        #[test]
        fn invalid_time() {
            assert_eq!(
                signature::decode::Error::from(
                    decode::signature(b"hello <> abc -1215").expect_err("parse fails as > is missing")
                )
                .to_string(),
                r##"Could parse to seconds: "abc""##
            );
        }
    }
}
