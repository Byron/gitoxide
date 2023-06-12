use std::path::{Path, PathBuf};

use crate::Bundle;

/// Returned by [`Bundle::at()`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("An 'idx' extension is expected of an index file: '{0}'")]
    InvalidPath(PathBuf),
    #[error(transparent)]
    Pack(#[from] crate::data::header::decode::Error),
    #[error(transparent)]
    Index(#[from] crate::index::init::Error),
}

/// Initialization
impl Bundle {
    /// Create a `Bundle` from `path`, which is either a pack file _(*.pack)_ or an index file _(*.idx)_.
    ///
    /// The corresponding complementary file is expected to be present.
    ///
    /// The `object_hash` is a way to read (and write) the same file format with different hashes, as the hash kind
    /// isn't stored within the file format itself.
    pub fn at(path: impl AsRef<Path>, object_hash: gix_hash::Kind) -> Result<Self, Error> {
        Self::at_inner(path.as_ref(), object_hash)
    }

    fn at_inner(path: &Path, object_hash: gix_hash::Kind) -> Result<Self, Error> {
        let ext = path
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .ok_or_else(|| Error::InvalidPath(path.to_owned()))?;
        Ok(match ext {
            "idx" => Self {
                index: crate::index::File::at(path, object_hash)?,
                pack: crate::data::File::at(path.with_extension("pack"), object_hash)?,
            },
            "pack" => Self {
                pack: crate::data::File::at(path, object_hash)?,
                index: crate::index::File::at(path.with_extension("idx"), object_hash)?,
            },
            _ => return Err(Error::InvalidPath(path.to_owned())),
        })
    }
}
