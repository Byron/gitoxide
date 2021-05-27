use crate::SafePartialName;
use bstr::{BStr, BString, ByteSlice};
use quick_error::quick_error;
use std::{borrow::Cow, convert::TryFrom, path::Path};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        RefnameValidation{err: git_validate::reference::name::Error, path: BString} {
            display("The path to a symbolic reference is invalid")
            source(err)
        }
    }
}

impl<'a> SafePartialName<'a> {
    pub fn to_path(&self) -> Cow<'a, Path> {
        self.0.to_path_lossy()
    }
}

impl<'a> TryFrom<&'a BStr> for SafePartialName<'a> {
    type Error = Error;

    fn try_from(v: &'a BStr) -> Result<Self, Self::Error> {
        Ok(SafePartialName(
            git_validate::reference::name_partial(v).map_err(|err| Error::RefnameValidation { err, path: v.into() })?,
        ))
    }
}

impl<'a> TryFrom<&'a str> for SafePartialName<'a> {
    type Error = Error;

    fn try_from(v: &'a str) -> Result<Self, Self::Error> {
        let v = v.as_bytes().as_bstr();
        Ok(SafePartialName(
            git_validate::reference::name_partial(v).map_err(|err| Error::RefnameValidation { err, path: v.into() })?,
        ))
    }
}
