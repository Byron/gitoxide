///
pub mod entry;
///
pub mod header;

/// Returned by [`File::decode_header()`][crate::data::File::decode_header()],
/// [`File::decode_entry()`][crate::data::File::decode_entry()] and .
/// [`File::decompress_entry()`][crate::data::File::decompress_entry()]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Failed to decompress pack entry")]
    ZlibInflate(#[from] git_features::zlib::inflate::Error),
    #[error("A delta chain could not be followed as the ref base with id {0} could not be found")]
    DeltaBaseUnresolved(git_hash::ObjectId),
}
