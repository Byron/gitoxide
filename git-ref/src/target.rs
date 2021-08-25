use bstr::BStr;
use git_hash::oid;

use crate::{Kind, TargetRef};

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
            TargetRef::Symbolic(path) => Some(path),
            TargetRef::Peeled(_) => None,
        }
    }
    /// Convert this instance into an owned version, without consuming it.
    pub fn into_owned(self) -> crate::Target {
        self.into()
    }
}
