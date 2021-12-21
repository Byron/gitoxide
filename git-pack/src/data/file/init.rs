use std::{convert::TryInto, path::Path};

use filebuffer::FileBuffer;

use crate::data;

/// Instantiation
impl data::File {
    /// Try opening a data file at the given `path`, and assume hashes are stored with the given `hash_len`.
    ///
    /// The `hash_kind` is a way to read (and write) the same file format with different hashes, as the hash kind
    /// isn't stored within the file format itself.
    pub fn at(path: impl AsRef<Path>, hash_kind: git_hash::Kind) -> Result<data::File, data::header::decode::Error> {
        Self::at_inner(path.as_ref(), hash_kind.len_in_bytes())
    }

    fn at_inner(path: &Path, hash_len: usize) -> Result<data::File, data::header::decode::Error> {
        use crate::data::header::N32_SIZE;

        let data = FileBuffer::open(path).map_err(|e| data::header::decode::Error::Io {
            source: e,
            path: path.to_owned(),
        })?;
        let pack_len = data.len();
        if pack_len < N32_SIZE * 3 + hash_len {
            return Err(data::header::decode::Error::Corrupt(format!(
                "Pack data of size {} is too small for even an empty pack with shortest hash",
                pack_len
            )));
        }
        let (kind, num_objects) =
            data::header::decode(&data[..12].try_into().expect("enough data after previous check"))?;
        Ok(data::File {
            data,
            path: path.to_owned(),
            id: git_features::hash::crc32(path.as_os_str().to_string_lossy().as_bytes()),
            version: kind,
            num_objects,
            hash_len,
        })
    }
}
