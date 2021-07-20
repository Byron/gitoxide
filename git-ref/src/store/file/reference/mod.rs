use crate::{file::Reference, mutable, mutable::FullName, Kind, Target};
use std::path::{Path, PathBuf};

impl<'a> Reference<'a> {
    /// Return the kind of ref.
    pub fn kind(&self) -> Kind {
        self.target.kind()
    }

    /// Return the target to which this instance is pointing.
    pub fn target(&self) -> Target<'_> {
        self.target.borrow()
    }

    /// Transform this reference into an owned `Target`
    pub fn into_target(self) -> mutable::Target {
        self.target
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
        let name = {
            use bstr::ByteSlice;
            name.replace(b"\\", b"/")
        };
        FullName(name.to_vec().into())
    }
}

mod logiter;

///
pub mod peel;

///
pub mod decode;
