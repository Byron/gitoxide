use super::Error;
use crate::borrowed::{
    util::{parse_hex_sha1, parse_oneline_header},
    Signature,
};
use bstr::BStr;
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

pub fn parse(i: &[u8]) -> IResult<&[u8], Commit, Error> {
    let (i, target) = parse_oneline_header(i, b"tree", parse_hex_sha1)
        .map_err(Error::context("tree <40 lowercase hex char>"))?;
    // let (i, target) = parse_oneline_header(i, b"tree", parse_hex_sha1)
    //     .map_err(Error::context("tree <40 lowercase hex char>"))?;
    unimplemented!("todo parse commit");
}
