use crate::pack::data;
use filebuffer::FileBuffer;
use git_hash::SIZE_OF_SHA1_DIGEST as SHA1_SIZE;
use std::{convert::TryFrom, convert::TryInto, path::Path};

/// Instantiation
impl data::File {
    /// Try opening a data file at the given `path`.
    pub fn at(path: impl AsRef<Path>) -> Result<data::File, data::header::decode::Error> {
        data::File::try_from(path.as_ref())
    }
}

impl TryFrom<&Path> for data::File {
    type Error = data::header::decode::Error;

    /// Try opening a data file at the given `path`.
    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        use data::header::N32_SIZE;

        let data = FileBuffer::open(path).map_err(|e| data::header::decode::Error::Io {
            source: e,
            path: path.to_owned(),
        })?;
        let pack_len = data.len();
        if pack_len < N32_SIZE * 3 + SHA1_SIZE {
            return Err(data::header::decode::Error::Corrupt(format!(
                "Pack data of size {} is too small for even an empty pack",
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
        })
    }
}
