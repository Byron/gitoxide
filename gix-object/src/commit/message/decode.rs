use winnow::{
    combinator::{alt, eof, preceded, rest, terminated},
    error::ParserError,
    prelude::*,
    stream::{Offset, Stream},
    token::take_till1,
};

use crate::bstr::{BStr, ByteSlice};

pub(crate) fn newline<'a, E: ParserError<&'a [u8]>>(i: &mut &'a [u8]) -> PResult<&'a [u8], E> {
    alt((b"\n", b"\r\n")).parse_next(i)
}

fn subject_and_body<'a, E: ParserError<&'a [u8]>>(i: &mut &'a [u8]) -> PResult<(&'a BStr, Option<&'a BStr>), E> {
    let start_i = *i;
    let start = i.checkpoint();
    while !i.is_empty() {
        match take_till1::<_, _, E>(|c| c == b'\n' || c == b'\r').parse_next(i) {
            Ok(_) => {
                let consumed_bytes = i.offset_from(&start);
                match preceded((newline::<E>, newline::<E>), rest).parse_next(i) {
                    Ok(body) => {
                        let body = (!body.is_empty()).then(|| body.as_bstr());
                        return Ok((start_i[0usize..consumed_bytes].as_bstr(), body));
                    }
                    Err(_) => match i.next_token() {
                        Some(_) => {}
                        None => break,
                    },
                }
            }
            Err(_) => match i.next_token() {
                Some(_) => {}
                None => break,
            },
        }
    }

    i.reset(start);
    rest.map(|r: &[u8]| (r.as_bstr(), None)).parse_next(i)
}

/// Returns title and body, without separator
pub fn message(mut input: &[u8]) -> (&BStr, Option<&BStr>) {
    terminated(subject_and_body::<()>, eof)
        .parse_next(&mut input)
        .expect("cannot fail")
}
