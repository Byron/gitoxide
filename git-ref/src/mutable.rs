use crate::Kind;
use bstr::{BStr, BString, ByteSlice};
use git_hash::{oid, ObjectId};
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

impl<'a> From<crate::FullName<'a>> for FullName {
    fn from(value: crate::FullName<'a>) -> Self {
        FullName(value.as_bstr().into())
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
    /// Returns the kind of the target the ref is pointing to.
    pub fn kind(&self) -> Kind {
        match self {
            Target::Symbolic(_) => Kind::Symbolic,
            Target::Peeled(_) => Kind::Peeled,
        }
    }

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
            Target::Peeled(oid) => crate::Target::Peeled(oid),
            Target::Symbolic(name) => crate::Target::Symbolic(name.0.as_bstr()),
        }
    }

    /// Interpret this target as object id which maybe `None` if it is symbolic.
    pub fn as_id(&self) -> Option<&oid> {
        match self {
            Target::Symbolic(_) => None,
            Target::Peeled(oid) => Some(oid),
        }
    }
    /// Interpret this target as name of the reference it points to which maybe `None` if it an object id.
    pub fn as_name(&self) -> Option<&BStr> {
        match self {
            Target::Symbolic(name) => Some(name.as_bstr()),
            Target::Peeled(_) => None,
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
            (Target::Symbolic(lhs), crate::Target::Symbolic(rhs)) => lhs.as_bstr() == *rhs,
            _ => false,
        }
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Target::Peeled(oid) => oid.fmt(f),
            Target::Symbolic(name) => write!(f, "ref: {}", name.as_bstr()),
        }
    }
}
