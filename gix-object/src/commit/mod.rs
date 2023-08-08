use bstr::{BStr, ByteSlice};

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

///
pub mod ref_iter;

mod write;

impl<'a> CommitRef<'a> {
    /// Deserialize a commit from the given `data` bytes while avoiding most allocations.
    pub fn from_bytes(data: &'a [u8]) -> Result<CommitRef<'a>, crate::decode::Error> {
        decode::commit(data).map(|(_, t)| t).map_err(crate::decode::Error::from)
    }

    /// Extracts the PGP signature and the signed commit data, if it is signed
    pub fn extract_signature(
        data: &'a [u8],
        signed_data: &mut Vec<u8>,
    ) -> Result<Option<std::borrow::Cow<'a, BStr>>, crate::decode::Error> {
        use crate::commit::ref_iter::Token;
        let pgp_sig_header = gix_actor::bstr::BStr::new(b"gpgsig");

        signed_data.clear();

        let mut signature = None;
        let out = signed_data;

        for token in crate::CommitRefIter::from_bytes(data).into_raw_iter() {
            let token = token?;

            // We don't care what the tokens are, just that they are valid, except
            // in the case of the signature, which we skip as it is the only part
            // of the commit that is not signed, obviously
            if let Token::ExtraHeader((hname, hval)) = &token.token {
                if hname == &pgp_sig_header {
                    signature = Some(hval.clone());
                    continue;
                }
            }

            // There is only one exception to the outputs, which is that the message
            // is always separated by an additional empty line from the rest of the
            // data
            if matches!(token.token, Token::Message(_)) {
                // If we don't have a signature when we've reached the message,
                // there is none and we can just short circuit
                if signature.is_none() {
                    out.clear();
                    return Ok(None);
                }

                out.push(b'\n');
            }

            out.extend_from_slice(token.buffer);
        }

        Ok(signature)
    }

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
        crate::commit::ExtraHeaders::new(self.extra_headers.iter().map(|(k, v)| (*k, v.as_ref())))
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
