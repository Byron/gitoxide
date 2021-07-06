use crate::transaction::FullName;
use crate::{file::Reference, Kind, Target};
use bstr::BString;
use git_hash::{oid, ObjectId};
use std::path::Path;

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

    /// Return the full name of this reference as path
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    /// Return the full validated name of the reference
    pub fn name(&self) -> FullName {
        use os_str_bytes::OsStrBytes;
        let name = self.relative_path.as_path().to_raw_bytes();
        FullName(name.to_vec().into())
    }
}

mod iter;

///
pub mod peel;

///
pub mod decode;
