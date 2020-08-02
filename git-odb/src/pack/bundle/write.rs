use crate::pack;
use git_features::progress::Progress;
use std::{io, path::Path};
use tempfile::NamedTempFile;

mod error {
    use crate::pack;
    use quick_error::quick_error;
    use std::io;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Io(err: io::Error) {
                display("An IO error occurred when reading the pack or creating a temporary file")
                from()
                source(err)
            }
            PackIter(err: pack::data::iter::Error) {
                display("Pack iteration failed")
                from()
                source(err)
            }
            PeristError(err: tempfile::PersistError) {
                display("Could not move a temporary file into its desired place")
                from()
                source(err)
            }
            IndexWrite(err: pack::index::write::Error) {
                display("The index file could not be written")
                from()
                source(err)
            }
        }
    }
}
use error::Error;

mod types {
    use crate::pack;
    use git_object::owned;

    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Outcome {
        index: pack::index::write::Outcome,
        pack_kind: pack::data::Kind,
        pack_hash: owned::Id,
    }

    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub enum MemoryMode {
        /// Base + deltas in memory compressed
        InMemory,
        InMemoryDecompressed,
        /// Deltas in memory compressed
        ResolveBases,
        /// Bases in memory compressed
        ResolveDeltas,
        ResolveBasesAndDeltas,
    }
}
pub use types::*;

impl pack::Bundle {
    /// If `directory` is `None`, the output will be written to a sink
    pub fn write_to_directory<P>(
        pack: impl io::Read,
        _pack_size: Option<u64>,
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
        let pack_entries_iter = pack::data::Iter::new_from_header(io::BufReader::new(pack), iteration_mode)?;

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
