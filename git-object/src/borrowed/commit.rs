use super::Error;
use crate::borrowed::util::{parse_signature, NL};
use crate::borrowed::{
    util::{parse_hex_sha1, parse_oneline_header},
    Signature,
};
use bstr::{BStr, ByteSlice};
use nom::bytes::complete::tag;
use nom::combinator::all_consuming;
use nom::multi::many0;
use nom::IResult;

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
    let (i, tree) = parse_oneline_header(i, b"tree", parse_hex_sha1)
        .map_err(Error::context("tree <40 lowercase hex char>"))?;
    let (i, parents) = many0(|i| parse_oneline_header(i, b"parent", parse_hex_sha1))(i).map_err(
        Error::context("zero or more 'parent <40 lowercase hex char>'"),
    )?;
    let (i, author) = parse_oneline_header(i, b"author", parse_signature)
        .map_err(Error::context("author <signature>"))?;
    let (i, committer) = parse_oneline_header(i, b"committer", parse_signature)
        .map_err(Error::context("author <signature>"))?;
    let (i, message) = all_consuming(parse_message)(i)?;

    Ok((
        i,
        Commit {
            tree,
            parents,
            author,
            committer,
            encoding: None,
            message,
            pgp_signature: None,
        },
    ))
}
