use bstr::{BStr, BString, ByteVec};
use nom::{
    bytes::complete::{is_not, tag, take_until, take_while_m_n},
    combinator::{peek, recognize},
    multi::many1_count,
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::{immutable::object::decode, ByteSlice};

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

pub(crate) fn signature(i: &[u8]) -> IResult<&[u8], git_actor::immutable::Signature<'_>, decode::Error> {
    git_actor::immutable::signature::decode(i).map_err(|err| {
        nom::Err::Error(decode::Error::from(
            git_actor::immutable::signature::decode::Error::from(err),
        ))
    })
}
