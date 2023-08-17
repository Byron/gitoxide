use bstr::{BStr, BString, ByteSlice};
use std::ops::Range;

use crate::{Commit, CommitRef, TagRef};

mod decode;
///
pub mod message;

/// A parsed commit message that assumes a title separated from the body by two consecutive newlines.
///
/// Titles can have any amount of whitespace
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageRef<'a> {
    /// The title of the commit, as separated from the body with two consecutive newlines. The newlines are not included.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub title: &'a BStr,
    /// All bytes not consumed by the title, excluding the separating newlines.
    ///
    /// The body is `None` if there was now title separation or the body was empty after the separator.
    pub body: Option<&'a BStr>,
}

/// The raw commit data, parseable by [`CommitRef`] or [`Commit`], which was fed into a program to produce a signature.
///
/// See [`extract_signature()`](crate::CommitRefIter::signature()) for how to obtain it.
// TODO: implement `std::io::Read` to avoid allocations
#[derive(PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignedData<'a> {
    /// The raw commit data that includes the signature.
    data: &'a [u8],
    /// The byte range at which we find the signature. All but the signature is the data that was signed.
    signature_range: Range<usize>,
}

impl SignedData<'_> {
    /// Convenience method to obtain a copy of the signed data.
    pub fn to_bstring(&self) -> BString {
        let mut buf = BString::from(&self.data[..self.signature_range.start]);
        buf.extend_from_slice(&self.data[self.signature_range.end..]);
        buf
    }
}

impl From<SignedData<'_>> for BString {
    fn from(value: SignedData<'_>) -> Self {
        value.to_bstring()
    }
}

///
pub mod ref_iter;

mod write;

/// Lifecycle
impl<'a> CommitRef<'a> {
    /// Deserialize a commit from the given `data` bytes while avoiding most allocations.
    pub fn from_bytes(mut data: &'a [u8]) -> Result<CommitRef<'a>, crate::decode::Error> {
        decode::commit(&mut data).map_err(crate::decode::Error::from)
    }
}

/// Access
impl<'a> CommitRef<'a> {
    /// Return the `tree` fields hash digest.
    pub fn tree(&self) -> gix_hash::ObjectId {
        gix_hash::ObjectId::from_hex(self.tree).expect("prior validation of tree hash during parsing")
    }

    /// Returns an iterator of parent object ids
    pub fn parents(&self) -> impl Iterator<Item = gix_hash::ObjectId> + '_ {
        self.parents
            .iter()
            .map(|hex_hash| gix_hash::ObjectId::from_hex(hex_hash).expect("prior validation of hashes during parsing"))
    }

    /// Returns a convenient iterator over all extra headers.
    pub fn extra_headers(&self) -> crate::commit::ExtraHeaders<impl Iterator<Item = (&BStr, &BStr)>> {
        ExtraHeaders::new(self.extra_headers.iter().map(|(k, v)| (*k, v.as_ref())))
    }

    /// Return the author, with whitespace trimmed.
    ///
    /// This is different from the `author` field which may contain whitespace.
    pub fn author(&self) -> gix_actor::SignatureRef<'a> {
        self.author.trim()
    }

    /// Return the committer, with whitespace trimmed.
    ///
    /// This is different from the `committer` field which may contain whitespace.
    pub fn committer(&self) -> gix_actor::SignatureRef<'a> {
        self.committer.trim()
    }

    /// Returns a partially parsed message from which more information can be derived.
    pub fn message(&self) -> MessageRef<'a> {
        MessageRef::from_bytes(self.message)
    }

    /// Returns the time at which this commit was created.
    pub fn time(&self) -> gix_date::Time {
        self.committer.time
    }
}

impl Commit {
    /// Returns a convenient iterator over all extra headers.
    pub fn extra_headers(&self) -> ExtraHeaders<impl Iterator<Item = (&BStr, &BStr)>> {
        ExtraHeaders::new(self.extra_headers.iter().map(|(k, v)| (k.as_bstr(), v.as_bstr())))
    }
}

/// An iterator over extra headers in [owned][crate::Commit] and [borrowed][crate::CommitRef] commits.
pub struct ExtraHeaders<I> {
    inner: I,
}

/// Instantiation and convenience.
impl<'a, I> ExtraHeaders<I>
where
    I: Iterator<Item = (&'a BStr, &'a BStr)>,
{
    /// Create a new instance from an iterator over tuples of (name, value) pairs.
    pub fn new(iter: I) -> Self {
        ExtraHeaders { inner: iter }
    }
    /// Find the _value_ of the _first_ header with the given `name`.
    pub fn find(mut self, name: &str) -> Option<&'a BStr> {
        self.inner
            .find_map(move |(k, v)| if k == name.as_bytes().as_bstr() { Some(v) } else { None })
    }
    /// Return an iterator over all _values_ of headers with the given `name`.
    pub fn find_all(self, name: &'a str) -> impl Iterator<Item = &'a BStr> {
        self.inner
            .filter_map(move |(k, v)| if k == name.as_bytes().as_bstr() { Some(v) } else { None })
    }
    /// Return an iterator over all git mergetags.
    ///
    /// A merge tag is a tag object embedded within the respective header field of a commit, making
    /// it a child object of sorts.
    pub fn mergetags(self) -> impl Iterator<Item = Result<TagRef<'a>, crate::decode::Error>> {
        self.find_all("mergetag").map(|b| TagRef::from_bytes(b))
    }

    /// Return the cryptographic signature provided by gpg/pgp verbatim.
    pub fn pgp_signature(self) -> Option<&'a BStr> {
        self.find("gpgsig")
    }
}
