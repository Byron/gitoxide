use crate::SafeName;
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

impl<'a> SafeName<'a> {
    pub fn to_path(&self) -> Cow<'a, Path> {
        self.0.to_path_lossy()
    }
}

impl<'a> TryFrom<&'a BStr> for SafeName<'a> {
    type Error = Error;

    fn try_from(v: &'a BStr) -> Result<Self, Self::Error> {
        Ok(SafeName(
            git_validate::refname(v).map_err(|err| Error::RefnameValidation { err, path: v.into() })?,
        ))
    }
}

impl<'a> TryFrom<&'a str> for SafeName<'a> {
    type Error = Error;

    fn try_from(v: &'a str) -> Result<Self, Self::Error> {
        let v = v.as_bytes().as_bstr();
        Ok(SafeName(
            git_validate::refname(v).map_err(|err| Error::RefnameValidation { err, path: v.into() })?,
        ))
    }
}
