use bstr::{BStr, BString, ByteSlice};
use git_hash::ObjectId;
use std::{borrow::Cow, convert::TryFrom, fmt, path::Path};

/// Indicate that the given BString is a validate reference name or path that can be used as path on disk or written as target
/// of a symbolic reference
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct FullName(pub(crate) BString);

impl TryFrom<&str> for FullName {
    type Error = git_validate::refname::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(FullName(git_validate::refname(value.as_bytes().as_bstr())?.into()))
    }
}

impl TryFrom<&BStr> for FullName {
    type Error = git_validate::refname::Error;

    fn try_from(value: &BStr) -> Result<Self, Self::Error> {
        Ok(FullName(git_validate::refname(value)?.into()))
    }
}

impl AsRef<BStr> for FullName {
    fn as_ref(&self) -> &BStr {
        self.0.as_bstr()
    }
}

impl FullName {
    /// Interpret this fully qualified reference name as partial name.
    pub fn to_partial(&self) -> crate::PartialName<'_> {
        crate::PartialName(self.0.as_bstr())
    }

    /// Interpret this fully qualified reference as shared full name
    pub fn borrow(&self) -> crate::FullName<'_> {
        crate::FullName(self.0.as_bstr())
    }

    /// Convert this name into the relative path identifying the reference location relative to a repository
    pub fn to_path(&self) -> Cow<'_, Path> {
        self.0.to_path_lossy()
    }
}

/// Denotes a ref target, equivalent to [`Kind`][super::Kind], but with mutable data.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub enum Target {
    /// A ref that points to an object id
    Peeled(ObjectId),
    /// A ref that points to another reference by its validated name, adding a level of indirection.
    ///
    /// Note that this is an extension of gitoxide which will be helpful in logging all reference changes.
    Symbolic(FullName),
}

impl Target {
    /// Return true if this is a peeled target with a null hash
    pub fn is_null(&self) -> bool {
        match self {
            Target::Peeled(oid) => oid.is_null(),
            Target::Symbolic(_) => false,
        }
    }

    /// Interpret this owned Target as shared Target
    pub fn borrow(&self) -> crate::Target<'_> {
        match self {
            Target::Peeled(oid) => crate::Target::Peeled(&oid),
            Target::Symbolic(name) => crate::Target::Symbolic(name.0.as_bstr()),
        }
    }

    /// Create an instance that signals that a reference should exist if this value is used in a [`Change`][crate::transaction::Change].
    pub fn must_exist() -> Self {
        Target::Peeled(ObjectId::null_sha1())
    }
}

impl<'a> From<crate::Target<'a>> for Target {
    fn from(src: crate::Target<'a>) -> Self {
        match src {
            crate::Target::Peeled(oid) => Target::Peeled(oid.to_owned()),
            crate::Target::Symbolic(name) => Target::Symbolic(FullName(name.to_owned())),
        }
    }
}

impl<'a> PartialEq<crate::Target<'a>> for Target {
    fn eq(&self, other: &crate::Target<'a>) -> bool {
        match (self, other) {
            (Target::Peeled(lhs), crate::Target::Peeled(rhs)) => lhs == rhs,
            (Target::Symbolic(lhs), crate::Target::Symbolic(rhs)) => lhs.as_ref() == *rhs,
            _ => false,
        }
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Target::Peeled(oid) => oid.fmt(f),
            Target::Symbolic(name) => write!(f, "ref: {}", name.as_ref()),
        }
    }
}
