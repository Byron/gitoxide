use super::Error;
use crate::{
    commit,
    immutable::{parse, parse::NL, Signature},
    BStr, ByteSlice,
};
use nom::{
    branch::alt,
    bytes::{complete::is_not, complete::tag},
    combinator::{all_consuming, opt},
    multi::many0,
    IResult,
};
use smallvec::SmallVec;
use std::borrow::Cow;

/// A git commit parsed using [`from_bytes()`][Commit::from_bytes()].
///
/// A commit encapsulates information about a point in time at which the state of the repository is recorded, usually after a
/// change which is documented in the commit `message`.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Commit<'a> {
    /// HEX hash of tree object we point to. Usually 40 bytes long.
    ///
    /// Use [`tree()`][Commit::tree()] to obtain a decoded version of it.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub tree: &'a BStr,
    /// HEX hash of each parent commit. Empty for first commit in repository.
    pub parents: SmallVec<[&'a BStr; 1]>,
    /// Who wrote this commit.
    pub author: Signature<'a>,
    /// Who committed this commit.
    ///
    /// This may be different from the `author` in case the author couldn't write to the repository themselves and
    /// is commonly encountered with contributed commits.
    pub committer: Signature<'a>,
    /// The name of the message encoding, otherwise [UTF-8 should be assumed](https://github.com/git/git/blob/e67fbf927dfdf13d0b21dc6ea15dc3c7ef448ea0/commit.c#L1493:L1493).
    pub encoding: Option<&'a BStr>,
    /// The commit message documenting the change.
    pub message: &'a BStr,
    /// Extra header fields, in order of them being encountered, made accessible with the iterator returned by [`extra_headers()`][Commit::extra_headers()].
    pub extra_headers: Vec<(&'a BStr, Cow<'a, BStr>)>,
}

impl<'a> Commit<'a> {
    /// Deserialize a commit from the given `data` bytes while avoiding most allocations.
    pub fn from_bytes(data: &'a [u8]) -> Result<Commit<'a>, Error> {
        parse(data).map(|(_, t)| t).map_err(Error::from)
    }
    /// Return the `tree` fields hash digest.
    pub fn tree(&self) -> git_hash::ObjectId {
        git_hash::ObjectId::from_hex(self.tree).expect("prior validation of tree hash during parsing")
    }

    /// Returns an iterator of parent object ids
    pub fn parents(&self) -> impl Iterator<Item = git_hash::ObjectId> + '_ {
        self.parents
            .iter()
            .map(|hex_hash| git_hash::ObjectId::from_hex(hex_hash).expect("prior validation of hashes during parsing"))
    }

    /// Returns a convenient iterator over all extra headers.
    pub fn extra_headers(&self) -> commit::ExtraHeaders<impl Iterator<Item = (&BStr, &BStr)>> {
        commit::ExtraHeaders::new(self.extra_headers.iter().map(|(k, v)| (*k, v.as_ref())))
    }
}

fn parse_message(i: &[u8]) -> IResult<&[u8], &BStr, Error> {
    if i.is_empty() {
        // newline + [message]
        return Err(nom::Err::Error(Error::NomDetail(i.into(), "commit message is missing")));
    }
    let (i, _) = tag(NL)(i).map_err(Error::context("a newline separates headers from the message"))?;
    debug_assert!(!i.is_empty());
    Ok((&[], &i.as_bstr()))
}

fn parse(i: &[u8]) -> IResult<&[u8], Commit<'_>, Error> {
    let (i, tree) =
        parse::header_field(i, b"tree", parse::hex_sha1).map_err(Error::context("tree <40 lowercase hex char>"))?;
    let (i, parents) = many0(|i| parse::header_field(i, b"parent", parse::hex_sha1))(i)
        .map_err(Error::context("zero or more 'parent <40 lowercase hex char>'"))?;
    let (i, author) =
        parse::header_field(i, b"author", parse::signature).map_err(Error::context("author <signature>"))?;
    let (i, committer) =
        parse::header_field(i, b"committer", parse::signature).map_err(Error::context("committer <signature>"))?;
    let (i, encoding) =
        opt(|i| parse::header_field(i, b"encoding", is_not(NL)))(i).map_err(Error::context("encoding <encoding>"))?;
    let (i, extra_headers) = many0(alt((
        |i| parse::any_header_field_multi_line(i).map(|(i, (k, o))| (i, (k.as_bstr(), Cow::Owned(o)))),
        |i| parse::any_header_field(i, is_not(NL)).map(|(i, (k, o))| (i, (k.as_bstr(), Cow::Borrowed(o.as_bstr())))),
    )))(i)
    .map_err(Error::context("<field> <single-line|multi-line>"))?;
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
            extra_headers,
        },
    ))
}
