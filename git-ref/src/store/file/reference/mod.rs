use crate::{file::Reference, mutable, mutable::FullName, Kind, Target};
use bstr::BString;
use git_hash::{oid, ObjectId};
use std::path::{Path, PathBuf};

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

    /// Transform this reference into an owned `Target`
    pub fn into_target(self) -> mutable::Target {
        match self.state {
            State::ValidatedPath(path) => mutable::Target::Symbolic(FullName(path)),
            State::Id(oid) => mutable::Target::Peeled(oid),
        }
    }

    /// Return the full name of this reference as path.
    ///
    /// Use [`name()`][Reference::name()] if consistent component separators are required across platforms.
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    /// Return our relative path while consuming this instance
    pub fn into_relative_path(self) -> PathBuf {
        self.relative_path
    }

    /// Return the full validated name of the reference
    pub fn name(&self) -> FullName {
        use os_str_bytes::OsStrBytes;
        let name = self.relative_path.as_path().to_raw_bytes();
        #[cfg(windows)]
        let name = name.replace(b"\\", b"/");
        FullName(name.to_vec().into())
    }
}

mod logiter;

///
pub mod peel;

///
pub mod decode;
