use crate::pack::data::output::entry;
use git_hash::ObjectId;

/// The way input objects are handled
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum ObjectExpansion {
    /// Don't do anything with the input objects except for transforming them into pack entries
    AsIs,
}

impl Default for ObjectExpansion {
    fn default() -> Self {
        ObjectExpansion::AsIs
    }
}

/// Configuration options for the pack generation functions provied in [this module][crate::pack::data::output].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Options {
    /// The amount of threads to use at most when resolving the pack. If `None`, all logical cores are used.
    pub thread_limit: Option<usize>,
    /// The amount of objects per chunk or unit of work to be sent to threads for processing
    /// TODO: could this become the window size?
    pub chunk_size: usize,
    /// The pack data version to produce
    pub version: crate::pack::data::Version,
    /// The way input objects are handled
    pub input_object_expansion: ObjectExpansion,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            thread_limit: None,
            chunk_size: 10,
            version: Default::default(),
            input_object_expansion: Default::default(),
        }
    }
}

/// The error returned by the pack generation function [`to_entry_iter()`][crate::pack::data::output::objects_to_entries_iter()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error<LocateErr>
where
    LocateErr: std::error::Error + 'static,
{
    #[error(transparent)]
    Locate(#[from] LocateErr),
    #[error("Object id {oid} wasn't found in object database")]
    NotFound { oid: ObjectId },
    #[error("Entry expected to have hash {expected}, but it had {actual}")]
    PackToPackCopyCrc32Mismatch { actual: u32, expected: u32 },
    #[error(transparent)]
    NewEntry(entry::Error),
}
