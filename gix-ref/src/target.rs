use std::{convert::TryFrom, fmt};

use gix_hash::{oid, ObjectId};

use crate::{FullName, FullNameRef, Kind, Target, TargetRef};

impl<'a> TargetRef<'a> {
    /// Returns the kind of the target the ref is pointing to.
    pub fn kind(&self) -> Kind {
        match self {
            TargetRef::Symbolic(_) => Kind::Symbolic,
            TargetRef::Peeled(_) => Kind::Peeled,
        }
    }
    /// Interpret this target as object id which maybe `None` if it is symbolic.
    pub fn try_id(&self) -> Option<&oid> {
        match self {
            TargetRef::Symbolic(_) => None,
            TargetRef::Peeled(oid) => Some(oid),
        }
    }
    /// Interpret this target as object id or **panic** if it is symbolic.
    pub fn id(&self) -> &oid {
        match self {
            TargetRef::Symbolic(_) => panic!("BUG: tries to obtain object id from symbolic target"),
            TargetRef::Peeled(oid) => oid,
        }
    }
    /// Interpret this target as name of the reference it points to which maybe `None` if it an object id.
    pub fn try_name(&self) -> Option<&FullNameRef> {
        match self {
            TargetRef::Symbolic(name) => Some(name),
            TargetRef::Peeled(_) => None,
        }
    }
    /// Convert this instance into an owned version, without consuming it.
    pub fn into_owned(self) -> Target {
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
    pub fn to_ref(&self) -> TargetRef<'_> {
        match self {
            Target::Peeled(oid) => TargetRef::Peeled(oid),
            Target::Symbolic(name) => TargetRef::Symbolic(name.as_ref()),
        }
    }

    /// Interpret this target as object id which maybe `None` if it is symbolic.
    pub fn try_id(&self) -> Option<&oid> {
        match self {
            Target::Symbolic(_) => None,
            Target::Peeled(oid) => Some(oid),
        }
    }
    /// Interpret this target as object id or panic if it is symbolic.
    pub fn id(&self) -> &oid {
        match self {
            Target::Symbolic(_) => panic!("BUG: tries to obtain object id from symbolic target"),
            Target::Peeled(oid) => oid,
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
    pub fn try_name(&self) -> Option<&FullNameRef> {
        match self {
            Target::Symbolic(name) => Some(name.as_ref()),
            Target::Peeled(_) => None,
        }
    }
}

impl<'a> From<TargetRef<'a>> for Target {
    fn from(src: TargetRef<'a>) -> Self {
        match src {
            TargetRef::Peeled(oid) => Target::Peeled(oid.to_owned()),
            TargetRef::Symbolic(name) => Target::Symbolic(name.to_owned()),
        }
    }
}

impl<'a> PartialEq<TargetRef<'a>> for Target {
    fn eq(&self, other: &TargetRef<'a>) -> bool {
        match (self, other) {
            (Target::Peeled(lhs), TargetRef::Peeled(rhs)) => lhs == rhs,
            (Target::Symbolic(lhs), TargetRef::Symbolic(rhs)) => lhs.as_bstr() == rhs.as_bstr(),
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
