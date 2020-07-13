use byteorder::{BigEndian, ByteOrder};
use filebuffer::FileBuffer;
use git_object::{self as object, SHA1_SIZE};
use quick_error::quick_error;
use std::{convert::TryFrom, convert::TryInto, mem::size_of, path::Path};

pub mod read;

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
        ZlibInflate(err: crate::zlib::Error, msg: &'static str) {
            display("{}", msg)
            cause(err)
        }
        DeltaBaseUnresolved(id: object::Id) {
            display("A delta chain could not be applied as the ref base with id {} could not be found", id)
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

    /// Currently only done during pack verification - finding the right size is only possible by decompressing
    /// the pack entry beforehand, or by using the (to be sorted) offsets stored in an index file.
    pub fn entry_crc32(&self, pack_offset: u64, size: usize) -> u32 {
        let pack_offset: usize = pack_offset.try_into().expect("pack_size fits into usize");
        git_features::hash::crc32(&self.data[pack_offset..pack_offset + size])
    }

    fn assure_v2(&self) {
        assert!(
            if let Kind::V2 = self.kind.clone() { true } else { false },
            "Only V2 is implemented"
        );
    }

    pub fn entry(&self, offset: u64) -> decoded::Entry {
        self.assure_v2();
        let pack_offset: usize = offset.try_into().expect("offset representable by machine");
        assert!(pack_offset <= self.data.len(), "offset out of bounds");

        let object_data = &self.data[pack_offset..];
        let (object, decompressed_size, consumed_bytes) = decoded::Header::from_bytes(object_data, offset);
        decoded::Entry {
            header: object,
            decompressed_size,
            data_offset: offset + consumed_bytes,
        }
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
