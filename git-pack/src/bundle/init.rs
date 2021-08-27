use crate::Bundle;
use std::{
    convert::TryFrom,
    path::{Path, PathBuf},
};

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
    /// Also available via [`Bundle::try_from()`].
    pub fn at(path: impl AsRef<Path>) -> Result<Self, Error> {
        Self::try_from(path.as_ref())
    }
}

impl TryFrom<&Path> for Bundle {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| Error::InvalidPath(path.to_owned()))?;
        Ok(match ext {
            "idx" => Self {
                index: crate::index::File::at(path)?,
                pack: crate::data::File::at(path.with_extension("pack"))?,
            },
            "pack" => Self {
                pack: crate::data::File::at(path)?,
                index: crate::index::File::at(path.with_extension("idx"))?,
            },
            _ => return Err(Error::InvalidPath(path.to_owned())),
        })
    }
}
