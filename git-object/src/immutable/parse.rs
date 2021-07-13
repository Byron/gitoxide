use bstr::{BStr, BString, ByteVec};
use nom::{
    bytes::complete::{is_not, tag, take_until, take_while_m_n},
    combinator::{peek, recognize},
    error::{context, ContextError, ParseError},
    multi::many1_count,
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::ByteSlice;

pub(crate) const NL: &[u8] = b"\n";
pub(crate) const SPACE: &[u8] = b" ";
pub(crate) const SPACE_OR_NL: &[u8] = b" \n";

pub(crate) fn any_header_field_multi_line<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    i: &'a [u8],
) -> IResult<&'a [u8], (&'a [u8], BString), E> {
    let (i, (k, o)) = context(
        "name <multi-line-value>",
        peek(tuple((
            terminated(is_not(SPACE_OR_NL), tag(SPACE)),
            recognize(tuple((
                is_not(NL),
                tag(NL),
                many1_count(terminated(tuple((tag(SPACE), take_until(NL))), tag(NL))),
            ))),
        ))),
    )(i)?;
    assert!(!o.is_empty(), "we have parsed more than one value here");
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

pub(crate) fn header_field<'a, T, E: ParseError<&'a [u8]>>(
    i: &'a [u8],
    name: &'static [u8],
    parse_value: impl Fn(&'a [u8]) -> IResult<&'a [u8], T, E>,
) -> IResult<&'a [u8], T, E> {
    terminated(preceded(terminated(tag(name), tag(SPACE)), parse_value), tag(NL))(i)
}

pub(crate) fn any_header_field<'a, T, E: ParseError<&'a [u8]>>(
    i: &'a [u8],
    parse_value: impl Fn(&'a [u8]) -> IResult<&'a [u8], T, E>,
) -> IResult<&'a [u8], (&'a [u8], T), E> {
    terminated(
        tuple((terminated(is_not(SPACE_OR_NL), tag(SPACE)), parse_value)),
        tag(NL),
    )(i)
}

fn is_hex_digit_lc(b: u8) -> bool {
    matches!(b, b'0'..=b'9' | b'a'..=b'f')
}

pub fn hex_hash<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], &'a BStr, E> {
    take_while_m_n(
        git_hash::Kind::shortest().len_in_hex(),
        git_hash::Kind::longest().len_in_hex(),
        is_hex_digit_lc,
    )(i)
    .map(|(i, hex)| (i, hex.as_bstr()))
}

pub(crate) fn signature<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    i: &'a [u8],
) -> IResult<&'a [u8], git_actor::immutable::Signature<'a>, E> {
    git_actor::immutable::signature::decode(i)
}
