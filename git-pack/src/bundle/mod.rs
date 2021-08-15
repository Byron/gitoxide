use std::{
    convert::TryFrom,
    path::{Path, PathBuf},
};

mod find;
///
pub mod write;

mod verify {
    use std::sync::{atomic::AtomicBool, Arc};

    use git_features::progress::Progress;

    impl super::Bundle {
        /// Similar to [`crate::index::File::verify_integrity()`] but more convenient to call as the presence of the
        /// pack file is a given.
        pub fn verify_integrity<C, P>(
            &self,
            verify_mode: crate::index::verify::Mode,
            traversal: crate::index::traverse::Algorithm,
            make_pack_lookup_cache: impl Fn() -> C + Send + Sync,
            thread_limit: Option<usize>,
            progress: Option<P>,
            should_interrupt: Arc<AtomicBool>,
        ) -> Result<
            (git_hash::ObjectId, Option<crate::index::traverse::Outcome>, Option<P>),
            crate::index::traverse::Error<crate::index::verify::Error>,
        >
        where
            P: Progress,
            C: crate::cache::DecodeEntry,
        {
            self.index.verify_integrity(
                Some((&self.pack, verify_mode, traversal, make_pack_lookup_cache)),
                thread_limit,
                progress,
                should_interrupt,
            )
        }
    }
}

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

/// A way to uniquely identify the location of an object within a pack bundle
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Location {
    /// The id of the pack containing the object //TODO: this should  probably at least by a typedef or even an opaque type
    pub pack_id: u32,
    /// The index at which the object can be found in the index file corresponding to the `pack_id`.
    pub index_file_id: u32,
    /// The size of the entry of disk so that the range of bytes of the entry is `pack_offset..pack_offset + entry_size`.
    pub entry_size: usize,
    /// The start of the entry in the pack identified by `pack_id`.
    pub pack_offset: u64,
}

impl Location {
    /// Compute a range suitable for lookup in pack data using the [`entry_slice()`][crate::data::File::entry_slice()] method.
    pub fn entry_range(&self, pack_offset: u64) -> crate::data::EntryRange {
        pack_offset..pack_offset + self.entry_size as u64
    }
}

/// A bundle of pack data and the corresponding pack index
pub struct Bundle {
    /// The pack file corresponding to `index`
    pub pack: crate::data::File,
    /// The index file corresponding to `pack`
    pub index: crate::index::File,
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
