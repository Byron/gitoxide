use crate::{
    immutable::{decode, Signature},
    ByteSlice, Sign, Time,
};
use bstr::{BStr, BString, ByteVec};
use btoi::btoi;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take, take_until, take_while_m_n},
    character::is_digit,
    combinator::{peek, recognize},
    multi::many1_count,
    sequence::{preceded, terminated, tuple},
    IResult,
};

pub(crate) const NL: &[u8] = b"\n";
pub(crate) const SPACE: &[u8] = b" ";
pub(crate) const SPACE_OR_NL: &[u8] = b" \n";

pub(crate) fn any_header_field_multi_line(i: &[u8]) -> IResult<&[u8], (&[u8], BString), decode::Error> {
    let (i, (k, o)) = peek(tuple((
        terminated(is_not(SPACE_OR_NL), tag(SPACE)),
        recognize(tuple((
            is_not(NL),
            tag(NL),
            many1_count(terminated(tuple((tag(SPACE), take_until(NL))), tag(NL))),
        ))),
    )))(i)?;
    assert!(!o.is_empty());
    let end = &o[o.len() - 1] as *const u8 as usize;
    let start_input = &i[0] as *const u8 as usize;

    let bytes = o[..o.len() - 1].as_bstr();
    let mut out = BString::from(Vec::with_capacity(bytes.len()));
    let mut lines = bytes.lines();
    out.push_str(lines.next().expect("first line"));
    for line in lines {
        out.push(b'\n');
        out.push_str(&line[1..]); // cut leading space
    }
    Ok((&i[end - start_input + 1..], (k, out)))
}

pub(crate) fn header_field<'a, T>(
    i: &'a [u8],
    name: &'static [u8],
    parse_value: impl Fn(&'a [u8]) -> IResult<&'a [u8], T, decode::Error>,
) -> IResult<&'a [u8], T, decode::Error> {
    terminated(preceded(terminated(tag(name), tag(SPACE)), parse_value), tag(NL))(i)
}

pub(crate) fn any_header_field<'a, T>(
    i: &'a [u8],
    parse_value: impl Fn(&'a [u8]) -> IResult<&'a [u8], T, decode::Error>,
) -> IResult<&'a [u8], (&'a [u8], T), decode::Error> {
    terminated(
        tuple((terminated(is_not(SPACE_OR_NL), tag(SPACE)), parse_value)),
        tag(NL),
    )(i)
}

fn is_hex_digit_lc(b: u8) -> bool {
    matches!(b, b'0'..=b'9' | b'a'..=b'f')
}

pub(crate) fn hex_sha1(i: &[u8]) -> IResult<&[u8], &BStr, decode::Error> {
    take_while_m_n(40usize, 40, is_hex_digit_lc)(i).map(|(i, o)| (i, o.as_bstr()))
}

pub(crate) fn signature(i: &[u8]) -> IResult<&[u8], Signature<'_>, decode::Error> {
    let (i, (name, email, time_in_seconds, tzsign, tzhour, tzminute)) = tuple((
        terminated(take_until(&b" <"[..]), take(2usize)),
        terminated(take_until(&b"> "[..]), take(2usize)),
        terminated(take_until(SPACE), take(1usize)),
        alt((tag(b"-"), tag(b"+"))),
        take_while_m_n(2usize, 2, is_digit),
        take_while_m_n(2usize, 2, is_digit),
    ))(i)
    .map_err(decode::Error::context(
        "tagger <name> <<email>> <time seconds since epoch> <+|-><HHMM>",
    ))?;

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
            immutable::{parse, Signature},
            ByteSlice, Sign, Time,
        };

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
                parse::signature(b"Sebastian Thiel <byronimo@gmail.com> 1528473343 -0230")
                    .expect("parse to work")
                    .1,
                signature("Sebastian Thiel", "byronimo@gmail.com", 1528473343, Sign::Minus, -9000)
            );
        }

        #[test]
        fn tz_plus() {
            assert_eq!(
                parse::signature(b"Sebastian Thiel <byronimo@gmail.com> 1528473343 +0230")
                    .expect("parse to work")
                    .1,
                signature("Sebastian Thiel", "byronimo@gmail.com", 1528473343, Sign::Plus, 9000)
            );
        }

        #[test]
        fn negative_offset_0000() {
            assert_eq!(
                parse::signature(b"Sebastian Thiel <byronimo@gmail.com> 1528473343 -0000")
                    .expect("parse to work")
                    .1,
                signature("Sebastian Thiel", "byronimo@gmail.com", 1528473343, Sign::Minus, 0)
            );
        }

        #[test]
        fn empty_name_and_email() {
            assert_eq!(
                parse::signature(b" <> 12345 -1215").expect("parse to work").1,
                signature("", "", 12345, Sign::Minus, -44100)
            );
        }
    }
}
