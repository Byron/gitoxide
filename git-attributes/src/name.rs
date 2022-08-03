use bstr::BString;

use crate::{Name, NameRef};

impl<'a> NameRef<'a> {
    /// Turn this ref into its owned counterpart.
    pub fn to_owned(self) -> Name {
        Name(self.0.into())
    }

    /// Return the inner `str`.
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
    /// Provide our ref-type.
    pub fn as_ref(&'a self) -> NameRef<'a> {
        NameRef(self.0.as_ref())
    }

    /// Return the inner `str`.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

/// The error returned by [`parse::Iter`][crate::parse::Iter].
#[derive(Debug, thiserror::Error)]
#[error("Attribute has non-ascii characters or starts with '-': {attribute}")]
pub struct Error {
    /// The attribute that failed to parse.
    pub attribute: BString,
}
