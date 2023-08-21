use std::convert::TryInto;

use gix_object::bstr::{BStr, ByteSlice};
use winnow::{
    combinator::{delimited, opt, preceded, terminated},
    error::{FromExternalError, ParserError},
    prelude::*,
    token::take_while,
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

fn until_newline<'a, E>(input: &mut &'a [u8]) -> PResult<&'a BStr, E>
where
    E: ParserError<&'a [u8]>,
{
    terminated(take_while(0.., |b: u8| b != b'\r' && b != b'\n'), newline)
        .map(ByteSlice::as_bstr)
        .parse_next(input)
}

pub fn header<'a, E>(input: &mut &'a [u8]) -> PResult<Header, E>
where
    E: ParserError<&'a [u8]>,
{
    preceded(b"# pack-refs with: ", until_newline)
        .map(|traits| {
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
            Header { peeled, sorted }
        })
        .parse_next(input)
}

pub fn reference<'a, E: ParserError<&'a [u8]> + FromExternalError<&'a [u8], crate::name::Error>>(
    input: &mut &'a [u8],
) -> PResult<packed::Reference<'a>, E> {
    (
        terminated(hex_hash, b" "),
        until_newline.try_map(TryInto::try_into),
        opt(delimited(b"^", hex_hash, newline)),
    )
        .map(|(target, name, object)| packed::Reference { name, target, object })
        .parse_next(input)
}

#[cfg(test)]
mod tests;
