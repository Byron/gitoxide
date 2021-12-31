#![allow(missing_docs, unused)]

use crate::multi_index;
use byteorder::{BigEndian, WriteBytesExt};
use git_features::progress::Progress;
use std::convert::TryInto;
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
    pub(crate) const SIGNATURE: &'static [u8] = b"MIDX";
    pub(crate) const HEADER_LEN: usize = 4 /*signature*/ +
        1 /*version*/ +
        1 /*object id version*/ +
        1 /*num chunks */ +
        1 /*num base files */ +
        4 /*num pack files*/;

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

        let bytes_written = Self::write_header(
            &mut out,
            cf.num_chunks().try_into().expect("BUG: wrote more than 256 chunks"),
            index_paths_sorted.len() as u32,
            object_hash,
        )?;
        let mut chunk_write = cf.into_write(&mut out, bytes_written)?;
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

    fn write_header(
        mut out: impl std::io::Write,
        num_chunks: u8,
        num_indices: u32,
        object_hash: git_hash::Kind,
    ) -> std::io::Result<usize> {
        out.write_all(Self::SIGNATURE)?;
        out.write_all(&[crate::multi_index::Version::V1 as u8])?;
        out.write_all(&[object_hash as u8])?;
        out.write_all(&[num_chunks])?;
        out.write_all(&[0])?; /* unused number of base files */
        out.write_u32::<BigEndian>(num_indices)?;

        Ok(Self::HEADER_LEN)
    }
}
