use crate::pack;
use git_features::{progress, progress::Progress};
use std::{
    io,
    path::{Path, PathBuf},
};
use tempfile::NamedTempFile;

mod error;
use error::Error;

mod types;
use filebuffer::FileBuffer;
use types::PassThrough;
pub use types::{MemoryMode, Outcome};

impl pack::Bundle {
    /// If `directory` is `None`, the output will be written to a sink
    pub fn write_to_directory<P>(
        pack: impl io::Read,
        pack_size: Option<u64>,
        iteration_mode: pack::data::iter::Mode,
        thread_limit: Option<usize>,
        memory_mode: MemoryMode,
        index_kind: pack::index::Kind,
        directory: Option<impl AsRef<Path>>,
        mut progress: P,
    ) -> Result<Outcome, Error>
    where
        P: Progress,
        <<P as Progress>::SubProgress as Progress>::SubProgress: Send,
    {
        let mut read_progress = progress.add_child("read pack");
        read_progress.init(pack_size.map(|s| s as u32), Some("bytes"));
        let pack = progress::Read {
            reader: pack,
            progress: read_progress,
        };
        let is_in_memory_completely = memory_mode.is_in_memory();
        let indexing_progress = progress.add_child("create index file");

        let data_file = match directory.as_ref() {
            Some(directory) => Some(NamedTempFile::new_in(directory.as_ref())?),
            None if is_in_memory_completely => None,
            None => Some(NamedTempFile::new()?),
        };
        let data_path: Option<PathBuf> = data_file.as_ref().map(|f| f.as_ref().into());
        let mut pack = PassThrough {
            reader: pack,
            writer: data_file,
        };
        let pack_entries_iter = pack::data::Iter::new_from_header(io::BufReader::new(&mut pack), iteration_mode)?;
        let pack_kind = pack_entries_iter.kind();

        let outcome = match directory {
            Some(directory) => {
                let directory = directory.as_ref();
                let mut index_file = io::BufWriter::with_capacity(4096 * 8, NamedTempFile::new_in(directory)?);

                let outcome = if is_in_memory_completely {
                    pack::index::File::write_data_iter_to_stream(
                        index_kind,
                        memory_mode.into_write_mode(|_, _| None),
                        pack_entries_iter,
                        thread_limit,
                        indexing_progress,
                        &mut index_file,
                    )
                } else {
                    pack::index::File::write_data_iter_to_stream(
                        index_kind,
                        memory_mode.into_write_mode(new_pack_file_resolver(data_path)?),
                        pack_entries_iter,
                        thread_limit,
                        indexing_progress,
                        &mut index_file,
                    )
                }?;

                let data_file = pack.writer.expect("data file to always be set in write mode");
                let index_path = directory.join(format!("{}.idx", outcome.index_hash.to_sha1_hex_string()));
                let data_path = directory.join(format!("{}.pack", outcome.pack_hash.to_sha1_hex_string()));

                data_file.persist(&data_path)?;
                index_file
                    .into_inner()
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?
                    .persist(index_path)
                    .map_err(|err| {
                        progress.info(format!(
                            "pack file at {} is retained despite failing to move the index file into place. You can use plumbing to make it usable.",
                            data_path.display()
                        ));
                        err
                    })?;
                outcome
            }
            None => {
                if is_in_memory_completely {
                    pack::index::File::write_data_iter_to_stream(
                        index_kind,
                        memory_mode.into_write_mode(|_, _| None),
                        pack_entries_iter,
                        thread_limit,
                        indexing_progress,
                        io::sink(),
                    )
                } else {
                    pack::index::File::write_data_iter_to_stream(
                        index_kind,
                        memory_mode.into_write_mode(new_pack_file_resolver(data_path)?),
                        pack_entries_iter,
                        thread_limit,
                        indexing_progress,
                        io::sink(),
                    )
                }?
            }
        };

        Ok(Outcome {
            index: outcome,
            pack_kind,
        })
    }
}

fn new_pack_file_resolver(
    data_path: Option<PathBuf>,
) -> io::Result<impl Fn(pack::index::write::EntrySlice, &mut Vec<u8>) -> Option<()> + Send + Sync> {
    let data_path = data_path.expect("data path to be present if not in memory and there is no directory");
    let data_map = parking_lot::Mutex::new(None);
    let on_demand_pack_data_lookup = move |range: std::ops::Range<u64>, out: &mut Vec<u8>| -> Option<()> {
        let mut guard = data_map.lock();
        let possibly_map = guard.get_or_insert_with(|| FileBuffer::open(&data_path));
        possibly_map
            .as_ref()
            .ok()
            .map(|mapped_file| out.copy_from_slice(&mapped_file[range.start as usize..range.end as usize]))
    };
    Ok(on_demand_pack_data_lookup)
}
