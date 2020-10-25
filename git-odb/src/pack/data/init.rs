use crate::pack::data;
use filebuffer::FileBuffer;
use git_object::SHA1_SIZE;
use std::{convert::TryFrom, convert::TryInto, path::Path};

/// Instantiation
impl data::File {
    /// Try opening a data file at the given `path`.
    pub fn at(path: impl AsRef<Path>) -> Result<data::File, data::parse::Error> {
        data::File::try_from(path.as_ref())
    }
}

impl TryFrom<&Path> for data::File {
    type Error = data::parse::Error;

    /// Try opening a data file at the given `path`.
    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        use data::parse::N32_SIZE;

        let data = FileBuffer::open(path).map_err(|e| data::parse::Error::Io {
            source: e,
            path: path.to_owned(),
        })?;
        let pack_len = data.len();
        if pack_len < N32_SIZE * 3 + SHA1_SIZE {
            return Err(data::parse::Error::Corrupt(format!(
                "Pack data of size {} is too small for even an empty pack",
                pack_len
            )));
        }
        let (kind, num_objects) =
            data::parse::header(&data[..12].try_into().expect("enough data after previous check"))?;
        Ok(data::File {
            data,
            path: path.to_owned(),
            kind,
            num_objects,
        })
    }
}
