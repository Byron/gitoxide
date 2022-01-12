use filetime::FileTime;

use crate::{extension, Entry, State, Version};

mod entries;
pub mod header;

mod error {
    use quick_error::quick_error;

    use crate::decode;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Header(err: decode::header::Error) {
                display("The header could not be decoded")
                source(err)
                from()
            }
            Entry(index: u32) {
                display("Could not parse entry at index {}", index)
            }
            UnexpectedTrailerLength { expected: usize, actual: usize } {
                display("Index trailer should have been {} bytes long, but was {}", expected, actual)
            }
        }
    }
}
pub use error::Error;

#[derive(Default)]
pub struct Options {
    pub object_hash: git_hash::Kind,
    /// If Some(_), we are allowed to use more than one thread. If Some(N), use no more than N threads. If Some(0)|None, use as many threads
    /// as there are physical cores.
    ///
    /// This applies to loading extensions in parallel to entries if the common EOIE extension is available.
    /// It also allows to use multiple threads for loading entries if the IEOT extension is present.
    pub thread_limit: Option<usize>,
    /// The minimum size in bytes to load extensions in their own thread, assuming there is enough `num_threads` available.
    pub min_extension_block_in_bytes_for_threading: usize,
}

impl State {
    pub fn from_bytes(
        data: &[u8],
        timestamp: FileTime,
        Options {
            object_hash,
            thread_limit,
            min_extension_block_in_bytes_for_threading,
        }: Options,
    ) -> Result<(Self, git_hash::ObjectId), Error> {
        let (version, num_entries, post_header_data) = header::decode(data, object_hash)?;
        let start_of_extensions = extension::end_of_index_entry::decode(data, object_hash);

        let mut num_threads = git_features::parallel::num_threads(thread_limit);
        let path_backing_buffer_size = entries::estimate_path_storage_requirements_in_bytes(
            num_entries,
            data.len(),
            start_of_extensions,
            object_hash,
            version,
        );

        let (entries, ext, data) = match start_of_extensions {
            Some(offset) if num_threads > 1 => {
                let extensions_data = &data[offset..];
                let index_offsets_table = extension::index_entry_offset_table::find(extensions_data, object_hash);
                let (entries_res, (ext, data)) = git_features::parallel::threads(|scope| {
                    let extension_loading =
                        (extensions_data.len() > min_extension_block_in_bytes_for_threading).then({
                            num_threads -= 1;
                            || scope.spawn(|_| extension::decode::all(extensions_data, object_hash))
                        });
                    let entries_res = match index_offsets_table {
                        Some(entry_offsets) => {
                            let chunk_size = (entry_offsets.len() as f32 / num_threads as f32).ceil() as usize;
                            let num_chunks = entry_offsets.chunks(chunk_size).count();
                            let mut threads = Vec::with_capacity(num_chunks);
                            for (id, chunks) in entry_offsets.chunks(chunk_size).enumerate() {
                                let chunks = chunks.to_vec();
                                threads.push(scope.spawn(move |_| {
                                    let num_entries = chunks.iter().map(|c| c.num_entries).sum::<u32>() as usize;
                                    let mut entries = Vec::with_capacity(num_entries);
                                    let path_backing_buffer_size = entries::estimate_path_storage_requirements_in_bytes(
                                        num_entries as u32,
                                        data.len() / num_chunks,
                                        start_of_extensions.map(|ofs| ofs / num_chunks),
                                        object_hash,
                                        version,
                                    );
                                    let mut path_backing = Vec::with_capacity(path_backing_buffer_size);
                                    let mut is_sparse = false;
                                    for offset in chunks {
                                        let (
                                            entries::Outcome {
                                                is_sparse: chunk_is_sparse,
                                            },
                                            _data,
                                        ) = entries::load_chunk(
                                            &data[offset.from_beginning_of_file as usize..],
                                            &mut entries,
                                            &mut path_backing,
                                            offset.num_entries,
                                            object_hash,
                                            version,
                                        )?;
                                        is_sparse |= chunk_is_sparse;
                                    }
                                    Ok::<_, Error>((
                                        id,
                                        EntriesOutcome {
                                            entries,
                                            path_backing,
                                            is_sparse,
                                        },
                                    ))
                                }));
                            }
                            todo!("combined thread results in order ")
                        }
                        None => load_entries(
                            post_header_data,
                            path_backing_buffer_size,
                            num_entries,
                            object_hash,
                            version,
                        ),
                    };
                    let ext_res = extension_loading
                        .map(|thread| thread.join().unwrap())
                        .unwrap_or_else(|| extension::decode::all(extensions_data, object_hash));
                    (entries_res, ext_res)
                })
                .unwrap(); // this unwrap is for panics - if these happened we are done anyway.
                (entries_res?.0, ext, data)
            }
            None | Some(_) => {
                let (entries, data) = load_entries(
                    post_header_data,
                    path_backing_buffer_size,
                    num_entries,
                    object_hash,
                    version,
                )?;
                let (ext, data) = extension::decode::all(data, object_hash);
                (entries, ext, data)
            }
        };

        if data.len() != object_hash.len_in_bytes() {
            return Err(Error::UnexpectedTrailerLength {
                expected: object_hash.len_in_bytes(),
                actual: data.len(),
            });
        }

        let checksum = git_hash::ObjectId::from(data);
        let EntriesOutcome {
            entries,
            path_backing,
            is_sparse,
        } = entries;
        let extension::decode::Outcome { cache_tree } = ext;

        Ok((
            State {
                timestamp,
                version,
                cache_tree,
                entries,
                path_backing,
                is_sparse,
            },
            checksum,
        ))
    }
}

struct EntriesOutcome {
    pub entries: Vec<Entry>,
    pub path_backing: Vec<u8>,
    pub is_sparse: bool,
}

fn load_entries(
    post_header_data: &[u8],
    path_backing_buffer_size: usize,
    num_entries: u32,
    object_hash: git_hash::Kind,
    version: Version,
) -> Result<(EntriesOutcome, &[u8]), Error> {
    let mut entries = Vec::with_capacity(num_entries as usize);
    let mut path_backing = Vec::with_capacity(path_backing_buffer_size);
    entries::load_chunk(
        post_header_data,
        &mut entries,
        &mut path_backing,
        num_entries,
        object_hash,
        version,
    )
    .map(|(entries::Outcome { is_sparse }, data): (entries::Outcome, &[u8])| {
        (
            EntriesOutcome {
                entries,
                path_backing,
                is_sparse,
            },
            data,
        )
    })
}
