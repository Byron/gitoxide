use winnow::{
    combinator::alt, combinator::eof, combinator::preceded, combinator::rest, combinator::terminated,
    error::ParserError, prelude::*, stream::Offset, token::take_till1,
};

use crate::bstr::{BStr, ByteSlice};

pub(crate) fn newline<'a, E: ParserError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], &'a [u8], E> {
    alt((b"\n", b"\r\n")).parse_next(i)
}

fn subject_and_body<'a, E: ParserError<&'a [u8]>>(
    mut i: &'a [u8],
) -> IResult<&'a [u8], (&'a BStr, Option<&'a BStr>), E> {
    let start = i;
    while !i.is_empty() {
        match take_till1::<_, _, E>(|c| c == b'\n' || c == b'\r').parse_next(i) {
            Ok((next, _)) => {
                let consumed_bytes = next.offset_from(start);
                match preceded((newline::<E>, newline::<E>), rest).parse_next(next) {
                    Ok((next, body)) => {
                        let body = (!body.is_empty()).then(|| body.as_bstr());
                        return Ok((next, (start[0usize..consumed_bytes].as_bstr(), body)));
                    }
                    Err(_) => match next.get(1..) {
                        Some(next) => {
                            i = next;
                        }
                        None => break,
                    },
                }
            }
            Err(_) => match i.get(1..) {
                Some(next) => {
                    i = next;
                }
                None => break,
            },
        }
    }

    i = start;
    rest.map(|r: &[u8]| (r.as_bstr(), None)).parse_next(i)
}

/// Returns title and body, without separator
pub fn message(input: &[u8]) -> (&BStr, Option<&BStr>) {
    terminated(subject_and_body::<()>, eof)
        .parse_next(input)
        .expect("cannot fail")
        .1
}
