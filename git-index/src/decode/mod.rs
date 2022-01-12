use filetime::FileTime;

use crate::{extension, State};

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
            min_extension_block_in_bytes_for_threading: _,
        }: Options,
    ) -> Result<(Self, git_hash::ObjectId), Error> {
        let (version, num_entries, post_header_data) = header::decode(data, object_hash)?;
        let start_of_extensions = extension::end_of_index_entry::decode(data, object_hash);

        let num_threads = git_features::parallel::num_threads(thread_limit);
        let path_backing_buffer_size = entries::estimate_path_storage_requirements_in_bytes(
            num_entries,
            data.len(),
            start_of_extensions,
            object_hash,
            version,
        );

        let (entries, ext, data) = match start_of_extensions {
            Some(offset) if num_threads > 1 => {
                let start_of_extensions = &data[offset..];
                let index_offsets_table = extension::index_entry_offset_table::find(start_of_extensions, object_hash);
                let (entries_res, (ext, data)) = git_features::parallel::threads(|_scope| {
                    match index_offsets_table {
                        Some(entry_offsets) => {
                            dbg!(entry_offsets);
                            todo!("threaded entry loading if its worth it")
                        }
                        None => {
                            // TODO load all extensions in scoped, then get IEOT, then possibly multi-threaded entry parsing
                            (
                                entries::load_all(
                                    post_header_data,
                                    num_entries,
                                    path_backing_buffer_size,
                                    object_hash,
                                    version,
                                ),
                                extension::decode::all(start_of_extensions, object_hash),
                            )
                        }
                    }
                })
                .unwrap(); // this unwrap is for panics - if these happened we are done anyway.
                (entries_res?.0, ext, data)
            }
            None | Some(_) => {
                let (entries, data) = entries::load_all(
                    post_header_data,
                    num_entries,
                    path_backing_buffer_size,
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
        let entries::Outcome {
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
