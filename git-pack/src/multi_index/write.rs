#![allow(missing_docs, unused)]
use crate::multi_index;
use git_features::progress::Progress;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;

mod error {
    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Io(#[from] std::io::Error),
        #[error("Interrupted")]
        Interrupted,
    }
}
pub use error::Error;

pub struct Options {
    pub object_hash: git_hash::Kind,
}

pub struct Outcome<P> {
    /// The calculated multi-index checksum of the file at `multi_index_path`.
    pub multi_index_checksum: git_hash::ObjectId,
    /// The input progress
    pub progress: P,
}

impl multi_index::File {
    pub fn write_from_index_paths<P>(
        mut index_paths: Vec<PathBuf>,
        out: impl std::io::Write,
        progress: P,
        should_interrupt: &AtomicBool,
        Options { object_hash }: Options,
    ) -> Result<Outcome<P>, Error>
    where
        P: Progress,
    {
        let out = git_features::hash::Write::new(out, object_hash);
        let index_paths_sorted = {
            index_paths.sort();
            index_paths
        };
        todo!()
    }
}
