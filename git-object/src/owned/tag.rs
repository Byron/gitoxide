use crate::{
    owned::{self, ser, NL},
    Id,
};
use bstr::{BStr, BString};
use quick_error::quick_error;
use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        StartsWithDash {
            description("Tags must not start with a dash: '-'")
        }
        InvalidRefName(err: git_ref::validated::NameError) {
            display("The tag name was no valid reference name")
            from()
            cause(err)
        }
    }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> Self {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Tag {
    // Target SHA1 in hex, always 40 lower case characters from 0-9 and a-f
    pub target: Id,
    // The name of the tag, e.g. "v1.0"
    pub name: BString,
    pub target_kind: crate::Kind,
    pub message: BString,
    pub signature: owned::object::Signature,
    pub pgp_signature: Option<BString>,
}

impl Tag {
    pub fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
        ser::trusted_header_field_id(b"object", &self.target, &mut out)?;
        ser::trusted_header_field(b"type", self.target_kind.to_bytes(), &mut out)?;
        ser::header_field(b"tag", validated_name(self.name.as_ref())?, &mut out)?;
        ser::trusted_header_field_signature(b"tagger", &self.signature, &mut out)?;

        if !self.message.is_empty() {
            out.write_all(NL)?;
            out.write_all(&self.message)?;
        }
        if let Some(ref message) = self.pgp_signature {
            out.write_all(NL)?;
            out.write_all(&message)?;
        }
        Ok(())
    }
}

fn validated_name(name: &BStr) -> Result<&BStr, Error> {
    git_ref::validated::name(name)?;
    if name[0] == b'-' {
        return Err(Error::StartsWithDash);
    }
    Ok(name)
}

#[cfg(test)]
mod tests;
