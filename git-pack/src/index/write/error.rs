use std::io;

/// Returned by [`crate::index::File::write_data_iter_to_stream()`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("An IO error occurred when reading the pack or creating a temporary file")]
    Io(#[from] io::Error),
    #[error("A pack entry could not be extracted")]
    PackEntryDecode(#[from] crate::data::input::Error),
    #[error("Indices of type {} cannot be written, only {} are supported", *.0 as usize, crate::index::Version::default() as usize)]
    Unsupported(crate::index::Version),
    #[error("Ref delta objects are not supported as there is no way to look them up. Resolve them beforehand.")]
    IteratorInvariantNoRefDelta,
    #[error("The iterator failed to set a trailing hash over all prior pack entries in the last provided entry")]
    IteratorInvariantTrailer,
    #[error("Only u32::MAX objects can be stored in a pack, found {0}")]
    IteratorInvariantTooManyObjects(usize),
    #[error("{pack_offset} is not a valid offset for pack offset {distance}")]
    IteratorInvariantBaseOffset { pack_offset: u64, distance: u64 },
    #[error(transparent)]
    Tree(#[from] crate::cache::delta::Error),
    #[error(transparent)]
    TreeTraversal(#[from] crate::cache::delta::traverse::Error),
}
