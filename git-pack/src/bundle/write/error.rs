use std::io;

use gix_tempfile::handle::Writable;

/// The error returned by [`Bundle::write_to_directory()`][crate::Bundle::write_to_directory()]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("An IO error occurred when reading the pack or creating a temporary file")]
    Io(#[from] io::Error),
    #[error(transparent)]
    PackIter(#[from] crate::data::input::Error),
    #[error("Could not move a temporary file into its desired place")]
    Persist(#[from] gix_tempfile::handle::persist::Error<Writable>),
    #[error(transparent)]
    IndexWrite(#[from] crate::index::write::Error),
}
