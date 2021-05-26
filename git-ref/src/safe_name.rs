use crate::SafeName;
use bstr::{BStr, ByteSlice};
use std::convert::TryFrom;

impl<'a> TryFrom<&'a BStr> for SafeName<'a> {
    type Error = git_validate::reference::name::Error;

    fn try_from(v: &'a BStr) -> Result<Self, Self::Error> {
        Ok(SafeName(git_validate::refname(v)?))
    }
}

impl<'a> TryFrom<&'a str> for SafeName<'a> {
    type Error = git_validate::reference::name::Error;

    fn try_from(v: &'a str) -> Result<Self, Self::Error> {
        Ok(SafeName(git_validate::refname(v.as_bytes().as_bstr())?))
    }
}
