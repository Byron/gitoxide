use crate::{
    commit,
    mutable::{self, encode, NL},
};
use bstr::{BStr, BString, ByteSlice};
use smallvec::SmallVec;
use std::io;

/// A mutable git commit, representing an annotated state of a working tree along with a reference to its historical commits.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Commit {
    /// The hash of recorded working tree state.
    pub tree: git_hash::ObjectId,
    /// Hash of each parent commit. Empty for the first commit in repository.
    pub parents: SmallVec<[git_hash::ObjectId; 1]>,
    /// Who wrote this commit.
    pub author: mutable::Signature,
    /// Who committed this commit.
    ///
    /// This may be different from the `author` in case the author couldn't write to the repository themselves and
    /// is commonly encountered with contributed commits.
    pub committer: mutable::Signature,
    /// The name of the message encoding, otherwise [UTF-8 should be assumed](https://github.com/git/git/blob/e67fbf927dfdf13d0b21dc6ea15dc3c7ef448ea0/commit.c#L1493:L1493).
    pub encoding: Option<BString>,
    /// The commit message documenting the change.
    pub message: BString,
    /// Extra header fields, in order of them being encountered, made accessible with the iterator returned
    /// by [`extra_headers()`][Commit::extra_headers()].
    pub extra_headers: Vec<(BString, BString)>,
}

impl Commit {
    /// Returns a convenient iterator over all extra headers.
    pub fn extra_headers(&self) -> commit::ExtraHeaders<impl Iterator<Item = (&BStr, &BStr)>> {
        commit::ExtraHeaders::new(self.extra_headers.iter().map(|(k, v)| (k.as_bstr(), v.as_bstr())))
    }
    /// Serializes this instance to `out` in the git serialization format.
    pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
        encode::trusted_header_id(b"tree", &self.tree, &mut out)?;
        for parent in &self.parents {
            encode::trusted_header_id(b"parent", parent, &mut out)?;
        }
        encode::trusted_header_signature(b"author", &self.author, &mut out)?;
        encode::trusted_header_signature(b"committer", &self.committer, &mut out)?;
        if let Some(encoding) = self.encoding.as_ref() {
            encode::header_field(b"encoding", encoding, &mut out)?;
        }
        for (name, value) in &self.extra_headers {
            let has_newline = value.find_byte(b'\n').is_some();
            if has_newline {
                encode::header_field_multi_line(name, value, &mut out)?;
            } else {
                encode::trusted_header_field(name, value, &mut out)?;
            }
        }
        out.write_all(NL)?;
        out.write_all(&self.message)
    }
}
