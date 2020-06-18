use crate::borrowed::Error;
use bstr::BStr;
use nom::IResult;

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum Mode {
    Tree = 0o040000,
    Blob = 0o100644,
    BlobExecutable = 0o100755,
    Link = 0o120000,
    Commit = 0o160000,
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Entry<'data> {
    pub mode: Mode,
    pub filename: &'data BStr,
    // 20 bytes SHA1
    pub oid: &'data [u8],
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Tree<'data>(pub Vec<Entry<'data>>);

pub(crate) fn parse(i: &[u8]) -> IResult<&[u8], Tree, Error> {
    unimplemented!("todo tree parse")
}
