use crate::{Name, NameRef};
use bstr::{BStr, BString, ByteSlice};

impl<'a> NameRef<'a> {
    pub fn to_owned(self) -> Name {
        Name(self.0.into())
    }

    pub fn inner(&self) -> &str {
        self.0
    }
}

impl<'a> Name {
    pub fn as_ref(&'a self) -> NameRef<'a> {
        NameRef(self.0.as_ref())
    }

    pub fn inner(&'a self) -> &'a BStr {
        self.0.as_bytes().as_bstr()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Attribute has non-ascii characters or starts with '-': {attribute}")]
pub struct Error {
    pub attribute: BString,
}
