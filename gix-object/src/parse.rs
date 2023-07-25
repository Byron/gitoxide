use bstr::{BStr, BString, ByteVec};
use winnow::{
    bytes::complete::{is_not, take_until, take_while_m_n},
    combinator::{peek, recognize},
    error::{context, ContextError, ParseError},
    multi::many1_count,
    sequence::{preceded, terminated},
    IResult,
};

use crate::ByteSlice;

pub(crate) const NL: &[u8] = b"\n";
pub(crate) const SPACE: &[u8] = b" ";
const SPACE_OR_NL: &[u8] = b" \n";

pub(crate) fn any_header_field_multi_line<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    i: &'a [u8],
) -> IResult<&'a [u8], (&'a [u8], BString), E> {
    let (i, (k, o)) = context(
        "name <multi-line-value>",
        peek((
            terminated(is_not(SPACE_OR_NL), SPACE),
            recognize((is_not(NL), NL, many1_count(terminated((SPACE, take_until(NL)), NL)))),
        )),
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
    terminated(preceded(terminated(name, SPACE), parse_value), NL)(i)
}

pub(crate) fn any_header_field<'a, T, E: ParseError<&'a [u8]>>(
    i: &'a [u8],
    parse_value: impl Fn(&'a [u8]) -> IResult<&'a [u8], T, E>,
) -> IResult<&'a [u8], (&'a [u8], T), E> {
    terminated((terminated(is_not(SPACE_OR_NL), SPACE), parse_value), NL)(i)
}

fn is_hex_digit_lc(b: u8) -> bool {
    matches!(b, b'0'..=b'9' | b'a'..=b'f')
}

pub fn hex_hash<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], &'a BStr, E> {
    take_while_m_n(
        gix_hash::Kind::shortest().len_in_hex(),
        gix_hash::Kind::longest().len_in_hex(),
        is_hex_digit_lc,
    )(i)
    .map(|(i, hex)| (i, hex.as_bstr()))
}

pub(crate) fn signature<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    i: &'a [u8],
) -> IResult<&'a [u8], gix_actor::SignatureRef<'a>, E> {
    gix_actor::signature::decode(i)
}
