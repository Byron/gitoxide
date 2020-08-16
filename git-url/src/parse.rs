use crate::{borrowed, bstr::ByteSlice, Protocol};
use bstr::BStr;
use nom::{
    bytes::complete::{tag, take_till1, take_while1},
    character::complete::alphanumeric1,
    combinator::recognize,
    sequence::tuple,
    IResult, Parser,
};

mod error;
pub use error::Error;

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
            path,
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
