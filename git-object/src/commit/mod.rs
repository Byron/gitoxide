use bstr::{BStr, ByteSlice};

pub use crate::CommitRefIter;
use crate::{immutable, immutable::object, CommitRef, TagRef};

mod decode;

///
pub mod ref_iter;

impl<'a> CommitRef<'a> {
    /// Deserialize a commit from the given `data` bytes while avoiding most allocations.
    pub fn from_bytes(data: &'a [u8]) -> Result<CommitRef<'a>, object::decode::Error> {
        decode::commit(data)
            .map(|(_, t)| t)
            .map_err(object::decode::Error::from)
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
    pub fn extra_headers(&self) -> crate::commit::ExtraHeaders<impl Iterator<Item = (&BStr, &BStr)>> {
        crate::commit::ExtraHeaders::new(self.extra_headers.iter().map(|(k, v)| (*k, v.as_ref())))
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
    pub fn mergetags(self) -> impl Iterator<Item = Result<TagRef<'a>, immutable::object::decode::Error>> {
        self.find_all("mergetag").map(|b| TagRef::from_bytes(b))
    }

    /// Return the cryptographic signature provided by gpg/pgp verbatim.
    pub fn pgp_signature(self) -> Option<&'a BStr> {
        self.find("gpgsig")
    }
}
