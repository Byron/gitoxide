use super::Error;
use crate::borrowed::{
    util::{parse_header_field, parse_hex_sha1, parse_signature, NL},
    Signature,
};
use bstr::{BStr, ByteSlice};
use nom::bytes::complete::is_not;
use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, opt},
    multi::many0,
    IResult,
};

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Commit<'data> {
    // SHA1 of tree object we point to
    pub tree: &'data BStr,
    // SHA1 of each parent commit. Empty for first commit in repository.
    pub parents: Vec<&'data BStr>,
    pub author: Signature<'data>,
    pub committer: Signature<'data>,
    // The name of the message encoding, otherwise UTF-8 should be assumed.
    pub encoding: Option<&'data BStr>,
    pub message: &'data BStr,
    pub pgp_signature: Option<&'data BStr>,
}

pub fn parse_message(i: &[u8]) -> IResult<&[u8], &BStr, Error> {
    if i.len() < 2 {
        // newline + [message] + newline
        return Err(nom::Err::Error(Error::NomDetail(
            i.into(),
            "commit message is missing",
        )));
    }
    let (i, _) = tag(NL)(i).map_err(Error::context(
        "a newline separates headers from the message",
    ))?;
    debug_assert!(!i.is_empty());
    let (x, _) = tag(NL)(&i[i.len() - 1..])
        .map_err(Error::context("commit message must end with newline"))?;
    Ok((x, &i[..i.len() - 1].as_bstr()))
}

pub fn parse(i: &[u8]) -> IResult<&[u8], Commit, Error> {
    let (i, tree) = parse_header_field(i, b"tree", parse_hex_sha1)
        .map_err(Error::context("tree <40 lowercase hex char>"))?;
    let (i, parents) = many0(|i| parse_header_field(i, b"parent", parse_hex_sha1))(i).map_err(
        Error::context("zero or more 'parent <40 lowercase hex char>'"),
    )?;
    let (i, author) = parse_header_field(i, b"author", parse_signature)
        .map_err(Error::context("author <signature>"))?;
    let (i, committer) = parse_header_field(i, b"committer", parse_signature)
        .map_err(Error::context("author <signature>"))?;
    let (i, encoding) = opt(|i| parse_header_field(i, b"encoding", is_not(NL)))(i)
        .map_err(Error::context("author <signature>"))?;
    let (i, message) = all_consuming(parse_message)(i)?;

    Ok((
        i,
        Commit {
            tree,
            parents,
            author,
            committer,
            encoding: encoding.map(ByteSlice::as_bstr),
            message,
            pgp_signature: None,
        },
    ))
}
