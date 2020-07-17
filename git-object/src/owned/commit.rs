use crate::owned::{self, ser, NL};
use bstr::{BString, ByteSlice};
use smallvec::SmallVec;
use std::io;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Commit {
    pub tree: owned::Id,
    /// SHA1 of each parent commit. Empty for first commit in repository.
    pub parents: SmallVec<[owned::Id; 1]>,
    pub author: owned::Signature,
    pub committer: owned::Signature,
    /// The name of the message encoding, otherwise UTF-8 should be assumed.
    pub encoding: Option<BString>,
    pub message: BString,
    pub pgp_signature: Option<BString>,
    /// Parsed single or multi-line headers, ready for use.
    pub extra_headers: Vec<(BString, BString)>,
}

impl Commit {
    pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
        ser::trusted_header_id(b"tree", &self.tree, &mut out)?;
        for parent in &self.parents {
            ser::trusted_header_id(b"parent", parent, &mut out)?;
        }
        ser::trusted_header_signature(b"author", &self.author, &mut out)?;
        ser::trusted_header_signature(b"committer", &self.committer, &mut out)?;
        if let Some(encoding) = self.encoding.as_ref() {
            ser::header_field(b"encoding", encoding, &mut out)?;
        }
        for (name, value) in &self.extra_headers {
            let has_newline = value.find_byte(b'\n').is_some();
            if has_newline {
                ser::header_field_multi_line(name, value, &mut out)?;
            } else {
                ser::trusted_header_field(name, value, &mut out)?;
            }
        }
        out.write_all(NL)?;
        out.write_all(&self.message)
    }
}
