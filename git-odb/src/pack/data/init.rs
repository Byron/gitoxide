use crate::pack::data;
use byteorder::{BigEndian, ByteOrder};
use filebuffer::FileBuffer;
use git_object::SHA1_SIZE;
use quick_error::quick_error;
use std::{convert::TryFrom, mem::size_of, path::Path};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: std::io::Error, path: std::path::PathBuf) {
            display("Could not open pack file at '{}'", path.display())
            cause(err)
        }
        Corrupt(msg: String) {
            display("{}", msg)
        }
        UnsupportedVersion(version: u32) {
            display("Unsupported pack version: {}", version)
        }
    }
}

const N32_SIZE: usize = size_of::<u32>();

/// Instantiation
impl data::File {
    pub fn at(path: impl AsRef<Path>) -> Result<data::File, Error> {
        data::File::try_from(path.as_ref())
    }
}

impl TryFrom<&Path> for data::File {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let data = FileBuffer::open(path).map_err(|e| Error::Io(e, path.to_owned()))?;
        let pack_len = data.len();
        if pack_len < N32_SIZE * 3 + SHA1_SIZE {
            return Err(Error::Corrupt(format!(
                "Pack data of size {} is too small for even an empty pack",
                pack_len
            )));
        }
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

        Ok(data::File {
            data,
            path: path.to_owned(),
            kind,
            num_objects,
        })
    }
}
