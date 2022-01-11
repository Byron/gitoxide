use bstr::{BStr, ByteSlice};
use filetime::FileTime;
use git_hash::Kind;
use std::ops::Range;

use crate::util::{from_be_u32, split_at_byte_exclusive, split_at_pos};
use crate::{entry, extension, Entry, State, Version};

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

impl State {
    pub fn from_bytes(
        data: &[u8],
        timestamp: FileTime,
        object_hash: git_hash::Kind,
    ) -> Result<(Self, git_hash::ObjectId), Error> {
        let (version, num_entries, post_header_data) = header::decode(data, object_hash)?;
        let start_of_extensions = extension::end_of_index_entry::decode(data, object_hash);

        let path_backing_buffer_size = entries::estimate_path_storage_requirements_in_bytes(
            num_entries,
            data.len(),
            start_of_extensions,
            object_hash,
            version,
        );
        let (entries, ext, data) = match start_of_extensions {
            Some(offset) => {
                let (entries_res, (ext, data)) = git_features::parallel::join(
                    // TODO load all extensions in thread, then get IEOT, then possibly multi-threaded entry parsing
                    || {
                        entries::load_all(
                            post_header_data,
                            num_entries,
                            path_backing_buffer_size,
                            object_hash,
                            version,
                        )
                    },
                    || extension::decode::all(&data[offset..], object_hash),
                );
                (entries_res?.0, ext, data)
            }
            None => {
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
