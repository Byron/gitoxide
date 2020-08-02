use crate::pack;
use git_features::{progress, progress::Progress};
use std::{io, path::Path};
use tempfile::NamedTempFile;

mod error;
use error::Error;

mod types;
use types::PassThrough;
pub use types::{MemoryMode, Outcome};

impl pack::Bundle {
    /// If `directory` is `None`, the output will be written to a sink
    pub fn write_to_directory<P>(
        pack: impl io::Read,
        pack_size: Option<u64>,
        iteration_mode: pack::data::iter::Mode,
        thread_limit: Option<usize>,
        _memory_mode: MemoryMode,
        index_kind: pack::index::Kind,
        directory: Option<impl AsRef<Path>>,
        mut progress: P,
    ) -> Result<pack::index::write::Outcome, Error>
    where
        P: Progress,
        <<P as Progress>::SubProgress as Progress>::SubProgress: Send,
    {
        let mut read_progress = progress.add_child("read pack");
        read_progress.init(pack_size.map(|s| s as u32), Some("bytes"));
        let mut pack = PassThrough {
            inner_read: progress::Read {
                read: pack,
                progress: read_progress,
            },
            inner_write: io::sink(),
        };
        let pack_entries_iter = pack::data::Iter::new_from_header(io::BufReader::new(&mut pack), iteration_mode)?;

        match directory {
            Some(directory) => {
                let directory = directory.as_ref();
                let mut index_file = io::BufWriter::with_capacity(4096 * 8, NamedTempFile::new_in(directory)?);
                let memory_mode = pack::index::write::Mode::in_memory_decompressed();
                let outcome = pack::index::File::write_data_iter_to_stream(
                    index_kind,
                    memory_mode,
                    pack_entries_iter,
                    thread_limit,
                    progress.add_child("create index file"),
                    &mut index_file,
                )?;

                let index_path = directory.join(format!("{}.idx", outcome.index_hash.to_sha1_hex_string()));
                index_file
                    .into_inner()
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?
                    .persist(index_path)?;
            }
            None => {
                unimplemented!("no output directory");
            }
        }

        // Consider thin packs resolution in data pack itself as Iterator - input iter::Entry, output resolved Entries
        // These can then be written to an output stream (Write) which can also be in the data pack.
        // This method just coordinates the pieces
        unimplemented!("pack writing and thin pack resolution")
    }
}
