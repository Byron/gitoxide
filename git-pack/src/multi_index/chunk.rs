/// Information for the chunk about index names
pub mod index_names {
    use std::path::{Path, PathBuf};

    use git_object::bstr::{BString, ByteSlice};
    use os_str_bytes::OsStrBytes;

    /// The ID used for the index-names chunk.
    pub const ID: git_chunk::Id = *b"PNAM";

    ///
    pub mod decode {
        use git_object::bstr::BString;

        /// The error returned by [from_bytes()][super::from_bytes()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error("The pack names were not ordered alphabetically.")]
            NotOrderedAlphabetically,
            #[error("Each pack path name must be terminated with a null byte")]
            MissingNullByte,
            #[error("Couldn't turn path '{path}' into OS path due to encoding issues")]
            PathEncoding { path: BString },
            #[error("non-padding bytes found after all paths were read.")]
            UnknownTrailerBytes,
        }
    }

    /// Parse null-separated index names from the given `chunk` of bytes and the expected number of packs and indices.
    /// Ignore padding bytes which are typically \0.
    pub fn from_bytes(mut chunk: &[u8], num_packs: u32) -> Result<Vec<PathBuf>, decode::Error> {
        let mut out = Vec::new();
        for _ in 0..num_packs {
            let null_byte_pos = chunk.find_byte(b'\0').ok_or(decode::Error::MissingNullByte)?;

            let path = &chunk[..null_byte_pos];
            let path = Path::from_raw_bytes(path)
                .map_err(|_| decode::Error::PathEncoding {
                    path: BString::from(path),
                })?
                .into_owned();

            if let Some(previous) = out.last() {
                if previous >= &path {
                    return Err(decode::Error::NotOrderedAlphabetically);
                }
            }
            out.push(path);

            chunk = &chunk[null_byte_pos + 1..];
        }

        if !chunk.is_empty() && !chunk.iter().all(|b| *b == 0) {
            return Err(decode::Error::UnknownTrailerBytes);
        }
        // NOTE: git writes garbage into this chunk, usually extra \0 bytes, which we simply ignore. If we were strict
        // about it we couldn't read this chunk data at all.
        Ok(out)
    }

    /// Calculate the size on disk for our chunk with the given index paths. Note that these are expected to have been processed already
    /// to actually be file names.
    pub fn storage_size(_paths: impl IntoIterator<Item = impl AsRef<Path>>) -> u64 {
        todo!("path computation with padding")
    }

    /// Write all `paths` in order to `out`.
    pub fn write(_paths: impl IntoIterator<Item = impl AsRef<Path>>, _out: impl std::io::Write) -> std::io::Result<()> {
        todo!("write path names")
    }
}

/// Information for the chunk with the fanout table
pub mod fanout {
    use std::convert::TryInto;

    /// The id uniquely identifying the fanout table.
    pub const ID: git_chunk::Id = *b"OIDF";

    /// Decode the fanout table contained in `chunk`, or return `None` if it didn't have the expected size.
    pub fn from_bytes(chunk: &[u8]) -> Option<[u32; 256]> {
        if chunk.len() != 4 * 256 {
            return None;
        }
        let mut out = [0; 256];
        for (c, f) in chunk.chunks(4).zip(out.iter_mut()) {
            *f = u32::from_be_bytes(c.try_into().unwrap());
        }
        out.into()
    }
}

/// Information about the oid lookup table.
pub mod lookup {
    use std::ops::Range;

    /// The id uniquely identifying the oid lookup table.
    pub const ID: git_chunk::Id = *b"OIDL";

    /// Return true if the size of the `offset` range seems to match for a `hash` of the given kind and the amount of objects.
    pub fn is_valid(offset: &Range<usize>, hash: git_hash::Kind, num_objects: u32) -> bool {
        (offset.end - offset.start) / hash.len_in_bytes() == num_objects as usize
    }
}

/// Information about the offsets table.
pub mod offsets {
    use std::ops::Range;

    /// The id uniquely identifying the offsets table.
    pub const ID: git_chunk::Id = *b"OOFF";

    /// Returns true if the `offset` range seems to match the size required for `num_objects`.
    pub fn is_valid(offset: &Range<usize>, num_objects: u32) -> bool {
        let entry_size = 4 /* pack-id */ + 4 /* pack-offset */;
        ((offset.end - offset.start) / num_objects as usize) == entry_size
    }
}

/// Information about the large offsets table.
pub mod large_offsets {
    use std::ops::Range;

    /// The id uniquely identifying the large offsets table (with 64 bit offsets)
    pub const ID: git_chunk::Id = *b"LOFF";

    /// Returns true if the `offsets` range seems to be properly aligned for the data we expect.
    pub fn is_valid(offset: &Range<usize>) -> bool {
        (offset.end - offset.start) % 8 == 0
    }
}
