use std::io;

use git_tempfile::handle::Writable;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("An IO error occurred when reading the pack or creating a temporary file")]
    Io(#[from] io::Error),
    #[error(transparent)]
    PackIter(#[from] crate::data::input::Error),
    #[error("Could not move a temporary file into its desired place")]
    Perist(#[from] git_tempfile::handle::persist::Error<Writable>),
    #[error(transparent)]
    IndexWrite(#[from] crate::index::write::Error),
}
