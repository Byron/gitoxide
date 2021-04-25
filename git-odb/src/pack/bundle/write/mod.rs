use filebuffer::FileBuffer;

use crate::pack;
use git_features::{interrupt, progress, progress::Progress};
use std::{
    io,
    path::{Path, PathBuf},
    sync::Arc,
};
use tempfile::NamedTempFile;

mod error;
use error::Error;

mod types;
use types::PassThrough;
pub use types::{Options, Outcome};

impl pack::Bundle {
    /// Given a `pack` data stream into the `directory` if `Some` or discard it entirely if `None`.
    ///
    /// `progress` provides detailed progress information which can be discarded with [`git_features::progress::Discard`].
    /// `options` further configure how the task is performed.
    pub fn write_stream_to_directory(
        pack: impl io::BufRead,
        directory: Option<impl AsRef<Path>>,
        mut progress: impl Progress,
        options: Options,
    ) -> Result<Outcome, Error> {
        let mut read_progress = progress.add_child("read pack");
        read_progress.init(None, progress::bytes());
        let pack = progress::Read {
            reader: pack,
            progress: progress::ThroughputOnDrop::new(read_progress),
        };

        let data_file = Arc::new(parking_lot::Mutex::new(match directory.as_ref() {
            Some(directory) => NamedTempFile::new_in(directory.as_ref())?,
            None => NamedTempFile::new()?,
        }));
        let data_path: PathBuf = data_file.lock().path().into();
        let pack = PassThrough {
            reader: interrupt::Read { inner: pack },
            writer: Some(data_file.clone()),
        };
        // This buff-reader is required to assure we call 'read()' in order to fill the (extra) buffer. Otherwise all the counting
        // we do with the wrapped pack reader doesn't work as it does not expect anyone to call BufRead functions directly.
        // However, this is exactly what's happening in the ZipReader implementation that is eventually used.
        // The performance impact of this is probably negligible, compared to all the other work that is done anyway :D.
        let buffered_pack = io::BufReader::new(pack);
        let pack_entries_iter = pack::data::EntriesFromBytesIter::new_from_header(
            buffered_pack,
            options.iteration_mode,
            pack::data::input::CompressedBytesMode::Crc32,
        )?;
        let pack_kind = pack_entries_iter.kind();
        let (outcome, data_path, index_path) =
            pack::Bundle::inner_write(directory, progress, options, data_file, data_path, pack_entries_iter)?;

        Ok(Outcome {
            index: outcome,
            pack_kind,
            data_path,
            index_path,
        })
    }

    /// Equivalent to [`write_stream_to_directory()`][pack::Bundle::write_stream_to_directory()] but offloads reading of the pack into its own thread, hence the `Send + 'static'` bounds.
    pub fn write_to_directory_eagerly(
        pack: impl io::Read + Send + 'static,
        pack_size: Option<u64>,
        directory: Option<impl AsRef<Path>>,
        mut progress: impl Progress,
        options: Options,
    ) -> Result<Outcome, Error> {
        let mut read_progress = progress.add_child("read pack");
        read_progress.init(pack_size.map(|s| s as usize), progress::bytes());
        let pack = progress::Read {
            reader: pack,
            progress: progress::ThroughputOnDrop::new(read_progress),
        };

        let data_file = Arc::new(parking_lot::Mutex::new(match directory.as_ref() {
            Some(directory) => NamedTempFile::new_in(directory.as_ref())?,
            None => NamedTempFile::new()?,
        }));
        let data_path: PathBuf = data_file.lock().path().into();
        let pack = PassThrough {
            reader: interrupt::Read { inner: pack },
            writer: Some(data_file.clone()),
        };
        let eight_pages = 4096 * 8;
        let buffered_pack = io::BufReader::with_capacity(eight_pages, pack);
        let pack_entries_iter = pack::data::EntriesFromBytesIter::new_from_header(
            buffered_pack,
            options.iteration_mode,
            pack::data::input::CompressedBytesMode::Crc32,
        )?;
        let pack_kind = pack_entries_iter.kind();
        let num_objects = pack_entries_iter.size_hint().0;
        let pack_entries_iter =
            git_features::parallel::EagerIterIf::new(|| num_objects > 25_000, pack_entries_iter, 5_000, 5);

        let (outcome, data_path, index_path) =
            pack::Bundle::inner_write(directory, progress, options, data_file, data_path, pack_entries_iter)?;

        Ok(Outcome {
            index: outcome,
            pack_kind,
            data_path,
            index_path,
        })
    }

    fn inner_write(
        directory: Option<impl AsRef<Path>>,
        mut progress: impl Progress,
        Options {
            thread_limit,
            iteration_mode: _,
            index_kind,
        }: Options,
        data_file: Arc<parking_lot::Mutex<NamedTempFile>>,
        data_path: PathBuf,
        pack_entries_iter: impl Iterator<Item = Result<pack::data::input::Entry, pack::data::input::Error>>,
    ) -> Result<(pack::index::write::Outcome, Option<PathBuf>, Option<PathBuf>), Error> {
        let indexing_progress = progress.add_child("create index file");
        Ok(match directory {
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
        })
    }
}

fn new_pack_file_resolver(
    data_path: PathBuf,
) -> io::Result<impl Fn(pack::data::EntryRange, &mut Vec<u8>) -> Option<()> + Send + Sync> {
    let mapped_file = FileBuffer::open(data_path)?;
    let pack_data_lookup = move |range: std::ops::Range<u64>, out: &mut Vec<u8>| -> Option<()> {
        mapped_file
            .get(range.start as usize..range.end as usize)
            .map(|pack_entry| out.copy_from_slice(pack_entry))
    };
    Ok(pack_data_lookup)
}
