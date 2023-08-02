use std::convert::TryInto;

use gix_object::bstr::{BStr, ByteSlice};
use winnow::{
    bytes::take_while,
    combinator::opt,
    error::{FromExternalError, ParseError},
    prelude::*,
    sequence::{delimited, preceded, terminated},
};

use crate::{
    parse::{hex_hash, newline},
    store_impl::packed,
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
    terminated(take_while(0.., |b: u8| b != b'\r' && b != b'\n'), newline)
        .map(ByteSlice::as_bstr)
        .parse_next(input)
}

pub fn header<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], Header, E>
where
    E: ParseError<&'a [u8]>,
{
    let (rest, traits) = preceded(b"# pack-refs with: ", until_newline).parse_next(input)?;

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
    let (input, (target, name)) =
        (terminated(hex_hash, b" "), until_newline.try_map(TryInto::try_into)).parse_next(input)?;
    let (rest, object) = opt(delimited(b"^", hex_hash, newline)).parse_next(input)?;
    Ok((rest, packed::Reference { name, target, object }))
}

#[cfg(test)]
mod tests;
