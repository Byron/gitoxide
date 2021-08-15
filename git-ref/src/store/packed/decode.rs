use std::convert::TryFrom;

use bstr::{BStr, ByteSlice};
use nom::{
    bytes::complete::{tag, take_while},
    combinator::{map, map_res, opt},
    error::{FromExternalError, ParseError},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::{
    parse::{hex_hash, newline},
    store::packed,
};

#[derive(Debug, PartialEq, Eq)]
enum Peeled {
    Unspecified,
    Partial,
    Fully,
}

/// Information parsed from the header of a packed ref file
#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    peeled: Peeled,
    pub sorted: bool,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            peeled: Peeled::Unspecified,
            sorted: false,
        }
    }
}

fn until_newline<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], &'a BStr, E>
where
    E: ParseError<&'a [u8]>,
{
    map(
        terminated(take_while(|b: u8| b != b'\r' && b != b'\n'), newline),
        |not_newline| not_newline.as_bstr(),
    )(input)
}

pub fn header<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Header, E>
where
    E: ParseError<&'a [u8]>,
{
    let (rest, traits) = preceded(tag(b"# pack-refs with: "), until_newline)(input)?;

    let mut peeled = Peeled::Unspecified;
    let mut sorted = false;
    for token in traits.as_bstr().split_str(b" ") {
        if token == b"fully-peeled" {
            peeled = Peeled::Fully;
        } else if token == b"peeled" {
            peeled = Peeled::Partial;
        } else if token == b"sorted" {
            sorted = true;
        }
    }

    Ok((rest, Header { peeled, sorted }))
}

pub fn reference<'a, E: ParseError<&'a [u8]> + FromExternalError<&'a [u8], crate::name::Error>>(
    input: &'a [u8],
) -> IResult<&'a [u8], packed::Reference<'a>, E> {
    let (input, (target, name)) = tuple((
        terminated(hex_hash, tag(b" ")),
        map_res(until_newline, crate::FullName::try_from),
    ))(input)?;
    let (rest, object) = opt(delimited(tag(b"^"), hex_hash, newline))(input)?;
    Ok((rest, packed::Reference { name, target, object }))
}

#[cfg(test)]
mod tests;
