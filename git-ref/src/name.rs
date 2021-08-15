use std::{
    borrow::Cow,
    convert::{Infallible, TryFrom},
    path::Path,
};

use bstr::{BStr, ByteSlice};

use crate::{FullName, PartialName};

/// The error used in the [`PartialName`][super::PartialName]::try_from(…) implementations.
pub type Error = git_validate::reference::name::Error;

impl<'a> FullName<'a> {
    /// Convert this name into the relative path identifying the reference location.
    pub fn to_path(self) -> Cow<'a, Path> {
        self.0.to_path().expect("UTF-8 conversion always succeeds").into()
    }

    /// Return ourselves as byte string which is a valid refname
    pub fn as_bstr(&self) -> &'a BStr {
        self.0
    }
}

impl<'a> PartialName<'a> {
    /// Convert this name into the relative path possibly identifying the reference location.
    /// Note that it may be only a partial path though.
    pub fn to_partial_path(self) -> Cow<'a, Path> {
        self.0.to_path().expect("UTF-8 conversion always succeeds").into()
    }

    /// Provide the name as binary string which is known to be a valid partial ref name.
    pub fn as_bstr(&self) -> &'a BStr {
        self.0
    }
}

impl<'a> TryFrom<&'a BStr> for FullName<'a> {
    type Error = Error;

    fn try_from(v: &'a BStr) -> Result<Self, Self::Error> {
        Ok(FullName(git_validate::reference::name(v)?))
    }
}

impl<'a> TryFrom<FullName<'a>> for PartialName<'a> {
    type Error = Infallible;

    fn try_from(v: FullName<'a>) -> Result<Self, Self::Error> {
        Ok(PartialName(v.0))
    }
}

impl<'a> TryFrom<&'a BStr> for PartialName<'a> {
    type Error = Error;

    fn try_from(v: &'a BStr) -> Result<Self, Self::Error> {
        Ok(PartialName(git_validate::reference::name_partial(v)?))
    }
}

impl<'a> TryFrom<&'a str> for FullName<'a> {
    type Error = Error;

    fn try_from(v: &'a str) -> Result<Self, Self::Error> {
        let v = v.as_bytes().as_bstr();
        Ok(FullName(git_validate::reference::name(v)?))
    }
}

impl<'a> TryFrom<&'a str> for PartialName<'a> {
    type Error = Error;

    fn try_from(v: &'a str) -> Result<Self, Self::Error> {
        let v = v.as_bytes().as_bstr();
        Ok(PartialName(git_validate::reference::name_partial(v)?))
    }
}

impl<'a> TryFrom<&'a String> for FullName<'a> {
    type Error = Error;

    fn try_from(v: &'a String) -> Result<Self, Self::Error> {
        let v = v.as_bytes().as_bstr();
        Ok(FullName(git_validate::reference::name(v)?))
    }
}

impl<'a> TryFrom<&'a String> for PartialName<'a> {
    type Error = Error;

    fn try_from(v: &'a String) -> Result<Self, Self::Error> {
        let v = v.as_bytes().as_bstr();
        Ok(PartialName(git_validate::reference::name_partial(v)?))
    }
}
