use crate::{Name, NameRef};
use bstr::BString;

impl<'a> NameRef<'a> {
    pub fn to_owned(self) -> Name {
        Name(self.0.into())
    }

    pub fn as_str(&self) -> &str {
        self.0
    }
}

impl AsRef<str> for NameRef<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl<'a> Name {
    pub fn as_ref(&'a self) -> NameRef<'a> {
        NameRef(self.0.as_ref())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Attribute has non-ascii characters or starts with '-': {attribute}")]
pub struct Error {
    pub attribute: BString,
}
