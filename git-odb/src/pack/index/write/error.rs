use crate::pack;
use std::io;

/// Returned by [`pack::index::File::write_data_iter_to_stream()`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("An IO error occurred when reading the pack or creating a temporary file")]
    Io(#[from] io::Error),
    #[error("A pack entry could not be extracted")]
    PackEntryDecode(#[from] pack::data::iter::Error),
    #[error("Indices of type {} cannot be written, only {} are supported", *.0 as usize, pack::index::Kind::default() as usize)]
    Unsupported(pack::index::Kind),
    #[error("Ref delta objects are not supported as there is no way to look them up. Resolve them beforehand.")]
    IteratorInvariantNoRefDelta,
    #[error("The iterator failed to set a trailing hash over all prior pack entries in the last provided entry")]
    IteratorInvariantTrailer,
    #[error("Did not encounter a single base")]
    IteratorInvariantBasesPresent,
    #[error("Only u32::MAX objects can be stored in a pack, found {0}")]
    IteratorInvariantTooManyObjects(usize),
    #[error("{pack_offset} is not a valid offset for pack offset {distance}")]
    IteratorInvariantBaseOffset { pack_offset: u64, distance: u64 },
    #[error(transparent)]
    Tree(#[from] pack::tree::Error),
    #[error(transparent)]
    TreeTraversal(#[from] pack::tree::traverse::Error),
}
