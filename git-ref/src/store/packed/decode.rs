use crate::parse::hex_sha1;
use crate::{
    parse::newline,
    store::{packed, packed::Peeled},
};
use bstr::{BStr, ByteSlice};
use nom::combinator::map;
use nom::sequence::{preceded, terminated};
use nom::{
    bytes::complete::{tag, take_while},
    combinator::opt,
    error::ParseError,
    sequence::{delimited, tuple},
    IResult,
};

fn until_newline<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], &'a BStr, E>
where
    E: ParseError<&'a [u8]>,
{
    map(
        terminated(take_while(|b: u8| b != b'\r' && b != b'\n'), newline),
        |not_newline| not_newline.as_bstr(),
    )(input)
}

fn header<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], packed::Header, E>
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

    Ok((rest, packed::Header { peeled, sorted }))
}

fn reference<'a, E: ParseError<&'a [u8]>>(input: &'a [u8]) -> IResult<&'a [u8], packed::Reference<'a>, E> {
    let (input, (target, full_name)) = tuple((terminated(hex_sha1, tag(b" ")), until_newline))(input)?;
    let (rest, object) = opt(delimited(tag(b"^"), hex_sha1, newline))(input)?;
    Ok((
        rest,
        packed::Reference {
            full_name,
            target,
            object,
        },
    ))
}

#[cfg(test)]
mod tests;
