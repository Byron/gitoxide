use std::{borrow::Cow, convert::TryFrom};

use bstr::{BStr, BString, ByteSlice, ByteVec};

// TODO This is a copy of gix/src/remote/name.rs; decide where this lives

pub enum Name<'repo> {
    /// A symbolic name, like `origin`.
    /// Note that it has not necessarily been validated yet.
    Symbol(Cow<'repo, str>),
    /// A url pointing to the remote host directly.
    Url(Cow<'repo, BStr>),
}

/// The error returned by [validated()].
#[derive(Debug, thiserror::Error)]
#[error("remote names must be valid within refspecs for fetching: {name:?}")]
#[allow(missing_docs)]
pub struct Error {
    pub source: gix_refspec::parse::Error,
    pub name: BString,
}

/// Return `name` if it is valid as symbolic remote name.
///
/// This means it has to be valid within a the ref path of a tracking branch.
pub fn validated(name: impl Into<BString>) -> Result<BString, Error> {
    let name = name.into();
    match gix_refspec::parse(
        format!("refs/heads/test:refs/remotes/{name}/test").as_str().into(),
        gix_refspec::parse::Operation::Fetch,
    ) {
        Ok(_) => Ok(name),
        Err(err) => Err(Error { source: err, name }),
    }
}

impl Name<'_> {
    /// Obtain the name as string representation.
    pub fn as_bstr(&self) -> &BStr {
        match self {
            Name::Symbol(v) => v.as_ref().into(),
            Name::Url(v) => v.as_ref(),
        }
    }

    /// Return this instance as a symbolic name, if it is one.
    #[allow(unused)]
    pub fn as_symbol(&self) -> Option<&str> {
        match self {
            Name::Symbol(n) => n.as_ref().into(),
            Name::Url(_) => None,
        }
    }

    /// Return this instance as url, if it is one.
    #[allow(unused)]
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

impl From<BString> for Name<'static> {
    fn from(name: BString) -> Self {
        Self::try_from(Cow::Owned(name)).expect("String is never illformed")
    }
}

impl<'a> AsRef<BStr> for Name<'a> {
    fn as_ref(&self) -> &BStr {
        self.as_bstr()
    }
}
