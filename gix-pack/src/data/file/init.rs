use std::{convert::TryInto, path::Path};

use crate::data;

/// Instantiation
impl data::File {
    /// Try opening a data file at the given `path`.
    ///
    /// The `object_hash` is a way to read (and write) the same file format with different hashes, as the hash kind
    /// isn't stored within the file format itself.
    pub fn at(path: impl AsRef<Path>, object_hash: gix_hash::Kind) -> Result<data::File, data::header::decode::Error> {
        Self::at_inner(path.as_ref(), object_hash)
    }

    fn at_inner(path: &Path, object_hash: gix_hash::Kind) -> Result<data::File, data::header::decode::Error> {
        use crate::data::header::N32_SIZE;
        let hash_len = object_hash.len_in_bytes();

        let data = crate::mmap::read_only(path).map_err(|e| data::header::decode::Error::Io {
            source: e,
            path: path.to_owned(),
        })?;
        let pack_len = data.len();
        if pack_len < N32_SIZE * 3 + hash_len {
            return Err(data::header::decode::Error::Corrupt(format!(
                "Pack data of size {pack_len} is too small for even an empty pack with shortest hash"
            )));
        }
        let (kind, num_objects) =
            data::header::decode(&data[..12].try_into().expect("enough data after previous check"))?;
        Ok(data::File {
            data,
            path: path.to_owned(),
            id: gix_features::hash::crc32(path.as_os_str().to_string_lossy().as_bytes()),
            version: kind,
            num_objects,
            hash_len,
            object_hash,
        })
    }
}
