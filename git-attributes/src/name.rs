use crate::{Name, NameRef, StateRef};
use bstr::{BStr, BString, ByteSlice};

impl<'a> NameRef<'a> {
    pub fn name(&self) -> &'a BStr {
        self.0
    }

    pub fn state(&self) -> StateRef<'a> {
        self.1
    }

    pub fn to_owned(self) -> Name {
        self.into()
    }
}

impl<'a> From<NameRef<'a>> for Name {
    fn from(v: NameRef<'a>) -> Self {
        Name(v.0.to_owned(), v.1.into())
    }
}

impl Name {
    pub fn name(&self) -> &BStr {
        self.0.as_bstr()
    }

    pub fn state(&self) -> StateRef<'_> {
        self.1.as_ref()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Attribute has non-ascii characters or starts with '-': {attribute}")]
pub struct Error {
    pub attribute: BString,
}
