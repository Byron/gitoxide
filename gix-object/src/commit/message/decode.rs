use winnow::{branch::alt, bytes::take_till1, combinator::eof, error::ParseError, prelude::*, sequence::terminated};

use crate::bstr::{BStr, ByteSlice};

pub(crate) fn newline<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], &'a [u8], E> {
    alt((b"\r\n", b"\n"))(i)
}

fn subject_and_body<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], (&'a BStr, Option<&'a BStr>), E> {
    let mut c = i;
    let mut consumed_bytes = 0;
    while !c.is_empty() {
        c = match take_till1::<_, _, E>(|c| c == b'\n' || c == b'\r')(c) {
            Ok((i1, segment)) => {
                consumed_bytes += segment.len();
                match (newline::<E>, newline::<E>).parse_next(i1) {
                    Ok((body, _)) => {
                        return Ok((
                            &[],
                            (
                                i[0usize..consumed_bytes].as_bstr(),
                                (!body.is_empty()).then(|| body.as_bstr()),
                            ),
                        ));
                    }
                    Err(_) => match i1.get(1..) {
                        Some(next) => {
                            consumed_bytes += 1;
                            next
                        }
                        None => break,
                    },
                }
            }
            Err(_) => match c.get(1..) {
                Some(next) => {
                    consumed_bytes += 1;
                    next
                }
                None => break,
            },
        };
    }
    Ok((&[], (i.as_bstr(), None)))
}

/// Returns title and body, without separator
pub fn message(input: &[u8]) -> (&BStr, Option<&BStr>) {
    terminated(subject_and_body::<()>, eof)(input).expect("cannot fail").1
}
