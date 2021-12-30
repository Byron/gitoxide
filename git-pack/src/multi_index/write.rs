#![allow(missing_docs, unused)]

use crate::multi_index;
use git_features::progress::Progress;
use std::io::Write;
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
        let mut out = git_features::hash::Write::new(out, object_hash);
        let (index_paths_sorted, index_filenames_sorted) = {
            index_paths.sort();
            let file_names = index_paths
                .iter()
                .map(|p| PathBuf::from(p.file_name().expect("file name present")))
                .collect::<Vec<_>>();
            (index_paths, file_names)
        };
        let mut cf = git_chunk::file::Index::for_writing();
        cf.plan_chunk(
            multi_index::chunk::index_names::ID,
            multi_index::chunk::index_names::storage_size(&index_filenames_sorted),
        );
        let mut chunk_write = cf.into_write(&mut out)?;
        while let Some(chunk_to_write) = chunk_write.next_chunk() {
            match chunk_to_write {
                multi_index::chunk::index_names::ID => {
                    multi_index::chunk::index_names::write(&index_filenames_sorted, &mut chunk_write)?
                }
                unknown => unreachable!("BUG: forgot to implement chunk {:?}", std::str::from_utf8(&unknown)),
            }
        }

        // write trailing checksum
        let multi_index_checksum: git_hash::ObjectId = out.hash.digest().into();
        let mut out = out.inner;
        out.write_all(multi_index_checksum.as_slice())?;

        Ok(Outcome {
            multi_index_checksum,
            progress,
        })
    }
}
