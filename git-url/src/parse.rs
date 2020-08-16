use crate::{borrowed, bstr::ByteSlice, Protocol};
use bstr::BStr;
use nom::{
    bytes::complete::{tag, take_till1, take_while1},
    character::complete::alphanumeric1,
    combinator::recognize,
    error::ParseError,
    sequence::tuple,
    IResult, Parser,
};
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Nom(err_msg: String) {
            display("{}", err_msg)
        }
        NomDetail(input: bstr::BString, msg: &'static str) {
            display("{}: '{}' could not be parsed", msg, input)
        }
    }
}

impl From<nom::Err<Error>> for Error {
    fn from(e: nom::Err<Error>) -> Self {
        match e {
            nom::Err::Error(err) | nom::Err::Failure(err) => Error::Nom(err.to_string()),
            nom::Err::Incomplete(_) => unreachable!("we do not implement streaming parsers"),
        }
    }
}

impl Error {
    fn set_parse_context(mut self, ctx: &'static str) -> Self {
        if let Error::NomDetail(_, ref mut message) = self {
            *message = ctx
        }
        self
    }

    pub(crate) fn context(msg: &'static str) -> impl Fn(nom::Err<Self>) -> nom::Err<Self> {
        move |e: nom::Err<Self>| e.map(|e| e.set_parse_context(msg))
    }
}

impl ParseError<&[u8]> for Error {
    fn from_error_kind(input: &[u8], _kind: nom::error::ErrorKind) -> Self {
        Error::NomDetail(input.into(), "parse error")
    }

    fn append(_: &[u8], _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

fn protocol(i: &[u8]) -> IResult<&[u8], Protocol, Error> {
    tag(b"ssh://")
        .map(|_| Protocol::Ssh)
        .parse(i)
        .map_err(Error::context("protocol parsing failed"))
}

fn host(i: &[u8]) -> IResult<&[u8], &BStr, Error> {
    recognize(tuple((take_till1(|c| c == b'.'), tag(b"."), alphanumeric1)))
        .map(|host: &[u8]| host.as_bstr())
        .parse(i)
}

fn path(i: &[u8]) -> IResult<&[u8], &BStr, Error> {
    // TODO: be much less permissive
    take_while1(|_| true).map(|path: &[u8]| path.as_bstr()).parse(i)
}

fn full_url(i: &[u8]) -> IResult<&[u8], borrowed::Url, Error> {
    tuple((protocol, host, path))
        .map(|(proto, host, path)| borrowed::Url {
            protocol: proto,
            user: None,
            host: Some(host),
            port: None,
            path: path,
            expand_user: None,
        })
        .parse(i)
}

fn any(i: &[u8]) -> IResult<&[u8], borrowed::Url, Error> {
    full_url(i)
}

pub fn parse(url: &[u8]) -> Result<borrowed::Url, Error> {
    any(url).map(|(_, url)| url).map_err(Into::into)
}
