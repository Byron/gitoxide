use crate::{file::Reference, Kind, Target};
use bstr::BString;
use git_hash::{oid, ObjectId};

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub(in crate::file) enum State {
    Id(ObjectId),
    ValidatedPath(BString),
}

impl State {
    fn as_id(&self) -> Option<&oid> {
        match self {
            State::Id(id) => Some(id),
            State::ValidatedPath(_) => None,
        }
    }
}

impl<'a> Reference<'a> {
    /// Return the kind of ref.
    pub fn kind(&self) -> Kind {
        match self.state {
            State::ValidatedPath(_) => Kind::Symbolic,
            State::Id(_) => Kind::Peeled,
        }
    }
    /// Return the target to which this instance is pointing.
    pub fn target(&'a self) -> Target<'a> {
        match self.state {
            State::ValidatedPath(ref path) => Target::Symbolic(path.as_ref()),
            State::Id(ref oid) => Target::Peeled(oid.as_ref()),
        }
    }
}

///
pub mod peel;

///
pub mod decode;
