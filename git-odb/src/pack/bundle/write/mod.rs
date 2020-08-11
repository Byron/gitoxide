use filebuffer::FileBuffer;

use crate::pack;
use git_features::{interruptible, progress, progress::Progress};
use std::{
    io,
    path::{Path, PathBuf},
    sync::Arc,
};
use tempfile::NamedTempFile;

mod error;
use error::Error;

mod types;
pub use types::Outcome;
use types::PassThrough;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Options {
    pub thread_limit: Option<usize>,
    pub iteration_mode: pack::data::iter::Mode,
    pub index_kind: pack::index::Kind,
}

impl pack::Bundle {
    /// If `directory` is `None`, the output will be written to a sink
    pub fn write_to_directory<P>(
        pack: impl io::Read + Send + 'static,
        pack_size: Option<u64>,
        directory: Option<impl AsRef<Path>>,
        mut progress: P,
        Options {
            thread_limit,
            iteration_mode,
            index_kind,
        }: Options,
    ) -> Result<Outcome, Error>
    where
        P: Progress,
        <P as Progress>::SubProgress: std::marker::Send + 'static,
        <<P as Progress>::SubProgress as Progress>::SubProgress: Send,
    {
        let mut read_progress = progress.add_child("read pack");
        read_progress.init(pack_size.map(|s| s as usize), Some(progress::bytes()));
        let pack = progress::Read {
            reader: pack,
            progress: progress::ThroughputOnDrop::new(read_progress),
        };
        let indexing_progress = progress.add_child("create index file");

        let data_file = Arc::new(parking_lot::Mutex::new(match directory.as_ref() {
            Some(directory) => NamedTempFile::new_in(directory.as_ref())?,
            None => NamedTempFile::new()?,
        }));
        let data_path: PathBuf = data_file.lock().path().into();
        let pack = PassThrough {
            reader: interruptible::Read { inner: pack },
            writer: Some(data_file.clone()),
        };
        let eight_pages = 4096 * 8;
        let buffered_pack = io::BufReader::with_capacity(eight_pages, pack);
        let pack_entries_iter = pack::data::Iter::new_from_header(
            buffered_pack,
            iteration_mode,
            pack::data::iter::CompressedBytesMode::CRC32,
        )?;
        let pack_kind = pack_entries_iter.kind();
        let num_objects = pack_entries_iter.size_hint().0;
        let pack_entries_iter =
            git_features::parallel::EagerIterIf::new(|| num_objects > 25_000, pack_entries_iter, 5_000, 5);

        let (outcome, data_path, index_path) = match directory {
            Some(directory) => {
                let directory = directory.as_ref();
                let mut index_file = NamedTempFile::new_in(directory)?;

                let outcome = pack::index::File::write_data_iter_to_stream(
                    index_kind,
                    move || new_pack_file_resolver(data_path),
                    pack_entries_iter,
                    thread_limit,
                    indexing_progress,
                    &mut index_file,
                )?;

                let data_path = directory.join(format!("{}.pack", outcome.data_hash.to_sha1_hex_string()));
                let index_path = data_path.with_extension("idx");

                Arc::try_unwrap(data_file)
                    .expect("only one handle left after pack was consumed")
                    .into_inner()
                    .persist(&data_path)?;
                index_file
                    .persist(&index_path)
                    .map_err(|err| {
                        progress.info(format!(
                            "pack file at {} is retained despite failing to move the index file into place. You can use plumbing to make it usable.",
                            data_path.display()
                        ));
                        err
                    })?;
                (outcome, Some(data_path), Some(index_path))
            }
            None => (
                pack::index::File::write_data_iter_to_stream(
                    index_kind,
                    move || new_pack_file_resolver(data_path),
                    pack_entries_iter,
                    thread_limit,
                    indexing_progress,
                    io::sink(),
                )?,
                None,
                None,
            ),
        };

        Ok(Outcome {
            index: outcome,
            pack_kind,
            data_path,
            index_path,
        })
    }
}

fn new_pack_file_resolver(
    data_path: PathBuf,
) -> io::Result<impl Fn(pack::data::EntrySlice, &mut Vec<u8>) -> Option<()> + Send + Sync> {
    let mapped_file = FileBuffer::open(data_path)?;
    let pack_data_lookup = move |range: std::ops::Range<u64>, out: &mut Vec<u8>| -> Option<()> {
        mapped_file
            .get(range.start as usize..range.end as usize)
            .map(|pack_entry| out.copy_from_slice(pack_entry))
    };
    Ok(pack_data_lookup)
}
