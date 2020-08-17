use crate::{borrowed, bstr::ByteSlice, Protocol};
use bstr::BStr;

mod error;
pub use error::Error;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1, take_while, take_while1, take_while_m_n},
    character::{complete::alphanumeric1, is_digit},
    combinator::{map_res, opt, recognize},
    multi::many_m_n,
    sequence::{preceded, terminated, tuple},
    IResult, Parser,
};

fn protocol(i: &[u8]) -> IResult<&[u8], Protocol, Error> {
    tag(b"ssh://")
        .map(|_| Protocol::Ssh)
        .parse(i)
        .map_err(Error::context("protocol parsing failed"))
}
fn v4n(i: &[u8]) -> IResult<&[u8], u8, Error> {
    map_res(take_while_m_n(1, 3, is_digit), |d| btoi::btoi(d))(i)
}

fn host(i: &[u8]) -> IResult<&[u8], &BStr, Error> {
    alt((
        recognize(tuple((many_m_n(3, 3, tuple((v4n, tag(b".")))), v4n))),
        recognize(tuple((take_till1(|c| c == b'.'), tag(b"."), alphanumeric1))),
    ))
    .map(|host: &[u8]| host.as_bstr())
    .parse(i)
}

fn path(i: &[u8]) -> IResult<&[u8], &BStr, Error> {
    // TODO: be much less permissive
    recognize(preceded(tag(b"/"), take_while(|_| true)))
        .map(|path: &[u8]| path.as_bstr())
        .parse(i)
        .map_err(Error::context("paths cannot be empty and start with '/'"))
}
fn user(i: &[u8]) -> IResult<&[u8], &BStr, Error> {
    terminated(take_while1(|c: u8| c != b'@'), tag(b"@"))
        .map(|user: &[u8]| user.as_bstr())
        .parse(i)
}

fn port(i: &[u8]) -> IResult<&[u8], u32, Error> {
    map_res(preceded(tag(b":"), take_while1(is_digit)), |input: &[u8]| {
        btoi::btoi(input)
    })(i)
}

fn full_url(i: &[u8]) -> IResult<&[u8], borrowed::Url, Error> {
    tuple((protocol, opt(user), host, opt(port), path))
        .map(|(proto, user, host, port, path)| borrowed::Url {
            protocol: proto,
            user,
            host: Some(host),
            port,
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
