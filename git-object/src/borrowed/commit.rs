use super::Error;
use crate::borrowed::util::parse_header_field_multiline;
use crate::borrowed::{
    util::{parse_header_field, parse_hex_sha1, parse_signature, NL},
    Signature,
};
use crate::{BStr, ByteSlice};
use nom::{
    branch::alt,
    bytes::{complete::is_not, complete::tag},
    combinator::{all_consuming, opt},
    multi::many0,
    IResult,
};
use smallvec::SmallVec;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Commit<'data> {
    // SHA1 of tree object we point to
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub tree: &'data BStr,
    // SHA1 of each parent commit. Empty for first commit in repository.
    pub parents: SmallVec<[&'data BStr; 1]>,
    pub author: Signature<'data>,
    pub committer: Signature<'data>,
    // The name of the message encoding, otherwise UTF-8 should be assumed.
    pub encoding: Option<&'data BStr>,
    pub message: &'data BStr,
    pub pgp_signature: Option<&'data BStr>,
}

pub fn parse_message(i: &[u8]) -> IResult<&[u8], &BStr, Error> {
    if i.is_empty() {
        // newline + [message]
        return Err(nom::Err::Error(Error::NomDetail(i.into(), "commit message is missing")));
    }
    let (i, _) = tag(NL)(i).map_err(Error::context("a newline separates headers from the message"))?;
    debug_assert!(!i.is_empty());
    Ok((&[], &i.as_bstr()))
}

pub fn parse(i: &[u8]) -> IResult<&[u8], Commit, Error> {
    let (i, tree) =
        parse_header_field(i, b"tree", parse_hex_sha1).map_err(Error::context("tree <40 lowercase hex char>"))?;
    let (i, parents) = many0(|i| parse_header_field(i, b"parent", parse_hex_sha1))(i)
        .map_err(Error::context("zero or more 'parent <40 lowercase hex char>'"))?;
    let (i, author) =
        parse_header_field(i, b"author", parse_signature).map_err(Error::context("author <signature>"))?;
    let (i, committer) =
        parse_header_field(i, b"committer", parse_signature).map_err(Error::context("author <signature>"))?;
    let (i, encoding) =
        opt(|i| parse_header_field(i, b"encoding", is_not(NL)))(i).map_err(Error::context("author <signature>"))?;
    let (i, pgp_signature) = opt(alt((
        |i| parse_header_field_multiline(i, b"gpgsig"),
        |i| parse_header_field(i, b"gpgsig", is_not(NL)),
    )))(i)
    .map_err(Error::context("author <signature>"))?;
    let (i, message) = all_consuming(parse_message)(i)?;

    Ok((
        i,
        Commit {
            tree,
            parents: SmallVec::from(parents),
            author,
            committer,
            encoding: encoding.map(ByteSlice::as_bstr),
            message,
            pgp_signature: pgp_signature.map(ByteSlice::as_bstr),
        },
    ))
}

impl<'data> Commit<'data> {
    pub fn tree(&self) -> crate::Id {
        crate::Id::from_hex(self.tree).expect("prior validation")
    }
    pub fn from_bytes(d: &'data [u8]) -> Result<Commit<'data>, Error> {
        parse(d).map(|(_, t)| t).map_err(Error::from)
    }
}
