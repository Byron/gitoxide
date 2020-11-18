use crate::pack::data;
use byteorder::{BigEndian, ByteOrder};

pub(crate) const N32_SIZE: usize = std::mem::size_of::<u32>();

/// Returned by [`header()`]
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Could not open pack file at '{path}'")]
    Io {
        source: std::io::Error,
        path: std::path::PathBuf,
    },
    #[error("{0}")]
    Corrupt(String),
    #[error("Unsupported pack version: {0}")]
    UnsupportedVersion(u32),
}

/// Parses the first 12 bytes of a pack file, returning the pack version as well as the number of objects contained in the pack.
pub fn header(data: &[u8; 12]) -> Result<(data::Kind, u32), Error> {
    let mut ofs = 0;
    if &data[ofs..ofs + b"PACK".len()] != b"PACK" {
        return Err(Error::Corrupt("Pack data type not recognized".into()));
    }
    ofs += N32_SIZE;
    let kind = match BigEndian::read_u32(&data[ofs..ofs + N32_SIZE]) {
        2 => data::Kind::V2,
        3 => data::Kind::V3,
        v => return Err(Error::UnsupportedVersion(v)),
    };
    ofs += N32_SIZE;
    let num_objects = BigEndian::read_u32(&data[ofs..ofs + N32_SIZE]);

    Ok((kind, num_objects))
}
