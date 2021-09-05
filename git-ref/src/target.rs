use std::fmt;

use bstr::BStr;
use git_hash::{oid, ObjectId};

use crate::{FullName, Kind, Target, TargetRef};
use std::convert::TryFrom;

impl<'a> TargetRef<'a> {
    /// Returns the kind of the target the ref is pointing to.
    pub fn kind(&self) -> Kind {
        match self {
            TargetRef::Symbolic(_) => Kind::Symbolic,
            TargetRef::Peeled(_) => Kind::Peeled,
        }
    }
    /// Interpret this target as object id which maybe `None` if it is symbolic.
    pub fn as_id(&self) -> Option<&oid> {
        match self {
            TargetRef::Symbolic(_) => None,
            TargetRef::Peeled(oid) => Some(oid),
        }
    }
    /// Interpret this target as name of the reference it points to which maybe `None` if it an object id.
    pub fn as_name(&self) -> Option<&BStr> {
        match self {
            TargetRef::Symbolic(path) => Some(path.as_bstr()),
            TargetRef::Peeled(_) => None,
        }
    }
    /// Convert this instance into an owned version, without consuming it.
    pub fn into_owned(self) -> crate::Target {
        self.into()
    }
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
    pub fn to_ref(&self) -> crate::TargetRef<'_> {
        match self {
            Target::Peeled(oid) => crate::TargetRef::Peeled(oid),
            Target::Symbolic(name) => crate::TargetRef::Symbolic(name.to_ref()),
        }
    }

    /// Interpret this target as object id which maybe `None` if it is symbolic.
    pub fn as_id(&self) -> Option<&oid> {
        match self {
            Target::Symbolic(_) => None,
            Target::Peeled(oid) => Some(oid),
        }
    }
    /// Return the contained object id or panic
    pub fn into_id(self) -> ObjectId {
        match self {
            Target::Symbolic(_) => panic!("BUG: expected peeled reference target but found symbolic one"),
            Target::Peeled(oid) => oid,
        }
    }

    /// Return the contained object id if the target is peeled or itself if it is not.
    pub fn try_into_id(self) -> Result<ObjectId, Self> {
        match self {
            Target::Symbolic(_) => Err(self),
            Target::Peeled(oid) => Ok(oid),
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

impl<'a> From<crate::TargetRef<'a>> for Target {
    fn from(src: crate::TargetRef<'a>) -> Self {
        match src {
            crate::TargetRef::Peeled(oid) => Target::Peeled(oid.to_owned()),
            crate::TargetRef::Symbolic(name) => Target::Symbolic(name.to_owned()),
        }
    }
}

impl<'a> PartialEq<crate::TargetRef<'a>> for Target {
    fn eq(&self, other: &crate::TargetRef<'a>) -> bool {
        match (self, other) {
            (Target::Peeled(lhs), crate::TargetRef::Peeled(rhs)) => lhs == rhs,
            (Target::Symbolic(lhs), crate::TargetRef::Symbolic(rhs)) => lhs.as_bstr() == rhs.as_bstr(),
            _ => false,
        }
    }
}

impl From<ObjectId> for Target {
    fn from(id: ObjectId) -> Self {
        Target::Peeled(id)
    }
}

impl TryFrom<Target> for ObjectId {
    type Error = Target;

    fn try_from(value: Target) -> Result<Self, Self::Error> {
        match value {
            Target::Peeled(id) => Ok(id),
            Target::Symbolic(_) => Err(value),
        }
    }
}

impl From<FullName> for Target {
    fn from(name: FullName) -> Self {
        Target::Symbolic(name)
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
