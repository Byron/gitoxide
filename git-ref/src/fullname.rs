use std::{borrow::Cow, convert::TryFrom, path::Path};

use bstr::{BStr, BString, ByteSlice};

use crate::{FullName, FullNameRef};

impl TryFrom<&str> for FullName {
    type Error = git_validate::refname::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(FullName(git_validate::refname(value.as_bytes().as_bstr())?.into()))
    }
}

impl TryFrom<String> for FullName {
    type Error = git_validate::refname::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        git_validate::refname(value.as_bytes().as_bstr())?;
        Ok(FullName(value.into()))
    }
}

impl TryFrom<&BStr> for FullName {
    type Error = git_validate::refname::Error;

    fn try_from(value: &BStr) -> Result<Self, Self::Error> {
        Ok(FullName(git_validate::refname(value)?.into()))
    }
}

impl TryFrom<BString> for FullName {
    type Error = git_validate::refname::Error;

    fn try_from(value: BString) -> Result<Self, Self::Error> {
        git_validate::refname(value.as_ref())?;
        Ok(FullName(value))
    }
}

impl From<FullName> for BString {
    fn from(name: FullName) -> Self {
        name.0
    }
}

impl<'a> From<FullNameRef<'a>> for &'a BStr {
    fn from(name: FullNameRef<'a>) -> Self {
        name.0
    }
}

impl<'a> From<crate::FullNameRef<'a>> for FullName {
    fn from(value: crate::FullNameRef<'a>) -> Self {
        FullName(value.as_bstr().into())
    }
}

impl FullName {
    /// Interpret this fully qualified reference name as partial name.
    pub fn to_partial(&self) -> crate::PartialNameRef<'_> {
        crate::PartialNameRef(self.0.as_bstr())
    }

    /// Interpret this fully qualified reference as shared full name
    pub fn to_ref(&self) -> crate::FullNameRef<'_> {
        crate::FullNameRef(self.0.as_bstr())
    }

    /// Convert this name into the relative path, lossily, identifying the reference location relative to a repository
    pub fn to_path(&self) -> Cow<'_, Path> {
        self.0.to_path().expect("UTF-8 conversion always succeeds").into()
    }

    /// Dissolve this instance and return the buffer.
    pub fn into_inner(self) -> BString {
        self.0
    }
    /// Return ourselves as byte string which is a valid refname
    pub fn as_bstr(&self) -> &BStr {
        self.0.as_bstr()
    }
}

impl<'a> FullNameRef<'a> {
    /// Create an owned copy of ourself
    pub fn to_owned(&self) -> FullName {
        FullName(self.0.to_owned())
    }
}
