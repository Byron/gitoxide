use std::{borrow::Cow, convert::TryFrom};

use super::Name;
use crate::bstr::{BStr, ByteSlice, ByteVec};

impl Name<'_> {
    /// Obtain the name as string representation.
    pub fn as_bstr(&self) -> &BStr {
        match self {
            Name::Symbol(v) => v.as_ref().into(),
            Name::Url(v) => v.as_ref(),
        }
    }

    /// Return this instance as a symbolic name, if it is one.
    pub fn as_symbol(&self) -> Option<&str> {
        match self {
            Name::Symbol(n) => n.as_ref().into(),
            Name::Url(_) => None,
        }
    }

    /// Return this instance as url, if it is one.
    pub fn as_url(&self) -> Option<&BStr> {
        match self {
            Name::Url(n) => n.as_ref().into(),
            Name::Symbol(_) => None,
        }
    }
}

impl<'a> TryFrom<Cow<'a, BStr>> for Name<'a> {
    type Error = Cow<'a, BStr>;

    fn try_from(name: Cow<'a, BStr>) -> Result<Self, Self::Error> {
        if name.contains(&b'/') || name.as_ref() == "." {
            Ok(Name::Url(name))
        } else {
            match name {
                Cow::Borrowed(n) => n.to_str().ok().map(Cow::Borrowed).ok_or(name),
                Cow::Owned(n) => Vec::from(n)
                    .into_string()
                    .map_err(|err| Cow::Owned(err.into_vec().into()))
                    .map(Cow::Owned),
            }
            .map(Name::Symbol)
        }
    }
}

impl<'a> AsRef<BStr> for Name<'a> {
    fn as_ref(&self) -> &BStr {
        self.as_bstr()
    }
}
