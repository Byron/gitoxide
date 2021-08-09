use crate::{Kind, Target};
use bstr::BStr;
use git_hash::oid;

impl<'a> Target<'a> {
    /// Returns the kind of the target the ref is pointing to.
    pub fn kind(&self) -> Kind {
        match self {
            Target::Symbolic(_) => Kind::Symbolic,
            Target::Peeled(_) => Kind::Peeled,
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
            Target::Symbolic(path) => Some(path),
            Target::Peeled(_) => None,
        }
    }
    /// Convert this instance into an owned version, without consuming it.
    pub fn to_owned(self) -> crate::mutable::Target {
        self.into()
    }
}
