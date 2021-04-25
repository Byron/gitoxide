use crate::pack::data;
use byteorder::{BigEndian, ByteOrder};

pub(crate) const N32_SIZE: usize = std::mem::size_of::<u32>();

/// Parses the first 12 bytes of a pack file, returning the pack version as well as the number of objects contained in the pack.
pub fn decode(data: &[u8; 12]) -> Result<(data::Version, u32), decode::Error> {
    let mut ofs = 0;
    if &data[ofs..ofs + b"PACK".len()] != b"PACK" {
        return Err(decode::Error::Corrupt("Pack data type not recognized".into()));
    }
    ofs += N32_SIZE;
    let kind = match BigEndian::read_u32(&data[ofs..ofs + N32_SIZE]) {
        2 => data::Version::V2,
        3 => data::Version::V3,
        v => return Err(decode::Error::UnsupportedVersion(v)),
    };
    ofs += N32_SIZE;
    let num_objects = BigEndian::read_u32(&data[ofs..ofs + N32_SIZE]);

    Ok((kind, num_objects))
}

/// Write a pack data header at `version` with `num_objects` and return a buffer.
pub fn encode(version: data::Version, num_objects: u32) -> [u8; 12] {
    use data::Version::*;
    let mut buf = [0u8; 12];
    buf[..4].copy_from_slice(b"PACK");
    buf[4..8].copy_from_slice(
        &match version {
            V2 => 2u32,
            V3 => 3,
        }
        .to_be_bytes()[..],
    );
    buf[8..].copy_from_slice(&num_objects.to_be_bytes()[..]);
    buf
}

///
pub mod decode {
    /// Returned by [`decode()`][super::decode()].
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
}
