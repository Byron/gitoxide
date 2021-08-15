use std::io;

use bstr::{BStr, BString};
use quick_error::quick_error;

use crate::mutable::{encode, NL};

quick_error! {
    /// An Error used in [`Tag::write_to()`].
    #[derive(Debug)]
    pub enum Error {
        StartsWithDash {
            display("Tags must not start with a dash: '-'")
        }
        InvalidRefName(err: git_validate::tag::name::Error) {
            display("The tag name was no valid reference name")
            from()
            source(err)
        }
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> Self {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

/// A mutable git tag.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Tag {
    /// The hash this tag is pointing to.
    pub target: git_hash::ObjectId,
    /// The kind of object this tag is pointing to.
    pub target_kind: crate::Kind,
    /// The name of the tag, e.g. "v1.0".
    pub name: BString,
    /// The message describing the tag.
    pub message: BString,
    /// The tags author.
    pub signature: Option<git_actor::Signature>,
    /// A pgp signature over all bytes of the encoded tag, excluding the pgp signature itself.
    pub pgp_signature: Option<BString>,
}

impl Tag {
    /// Writes the encoded tag to `out`.
    pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
        encode::trusted_header_id(b"object", &self.target, &mut out)?;
        encode::trusted_header_field(b"type", self.target_kind.as_bytes(), &mut out)?;
        encode::header_field(b"tag", validated_name(self.name.as_ref())?, &mut out)?;
        if let Some(tagger) = &self.signature {
            encode::trusted_header_signature(b"tagger", tagger, &mut out)?;
        }

        if !self.message.is_empty() {
            out.write_all(NL)?;
            out.write_all(&self.message)?;
        }
        if let Some(ref message) = self.pgp_signature {
            out.write_all(NL)?;
            out.write_all(message)?;
        }
        Ok(())
    }
}

fn validated_name(name: &BStr) -> Result<&BStr, Error> {
    git_validate::tag::name(name)?;
    if name[0] == b'-' {
        return Err(Error::StartsWithDash);
    }
    Ok(name)
}

#[cfg(test)]
mod tests;
