use super::Error;
use crate::{borrowed::Signature, Id};
use bstr::BStr;
use nom::IResult;

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Commit<'data> {
    pub tree: Id,
    pub parents: Vec<Id>,
    pub author: Signature<'data>,
    pub committer: Signature<'data>,
    pub encoding: Option<&'data BStr>,
    pub message: &'data BStr,
    pub pgp_signature: Option<&'data BStr>,
}
pub fn parse(i: &[u8]) -> IResult<&[u8], Commit, Error> {
    unimplemented!("todo parse commit");
}
