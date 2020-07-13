use byteorder::{BigEndian, ByteOrder};
use filebuffer::FileBuffer;
use git_object::SHA1_SIZE;
use quick_error::quick_error;
use std::{convert::TryFrom, mem::size_of, path::Path};

pub mod decode;

pub mod decoded;

pub mod verify;

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

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub enum Kind {
    V2,
    V3,
}

pub struct File {
    data: FileBuffer,
    path: std::path::PathBuf,
    kind: Kind,
    num_objects: u32,
}

/// Instantiation and basic file information
impl File {
    pub fn kind(&self) -> Kind {
        self.kind.clone()
    }
    pub fn num_objects(&self) -> u32 {
        self.num_objects
    }
    pub fn data_len(&self) -> usize {
        self.data.len()
    }
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn at(path: impl AsRef<Path>) -> Result<File, Error> {
        File::try_from(path.as_ref())
    }
}

impl TryFrom<&Path> for File {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let data = FileBuffer::open(path).map_err(|e| Error::Io(e, path.to_owned()))?;
        let pack_len = data.len();
        if pack_len < N32_SIZE * 3 + SHA1_SIZE {
            return Err(Error::Corrupt(format!(
                "Pack file of size {} is too small for even an empty pack",
                pack_len
            )));
        }
        let mut ofs = 0;
        if &data[ofs..ofs + b"PACK".len()] != b"PACK" {
            return Err(Error::Corrupt("Pack file type not recognized".into()));
        }
        ofs += N32_SIZE;
        let kind = match BigEndian::read_u32(&data[ofs..ofs + N32_SIZE]) {
            2 => Kind::V2,
            3 => Kind::V3,
            v => return Err(Error::UnsupportedVersion(v)),
        };
        ofs += N32_SIZE;
        let num_objects = BigEndian::read_u32(&data[ofs..ofs + N32_SIZE]);

        Ok(File {
            data,
            path: path.to_owned(),
            kind,
            num_objects,
        })
    }
}
