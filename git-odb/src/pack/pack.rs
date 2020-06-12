use byteorder::{BigEndian, ByteOrder};
use filebuffer::FileBuffer;
use std::{mem::size_of, path::Path};

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

use crate::object::SHA1_SIZE;

const N32_SIZE: usize = size_of::<u32>();

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Kind {
    V2,
    V3,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Entry {
    pub object: parsed::Object,
    pub decompressed_size: u64,
    /// absolute offset to compressed object data in the pack
    pub offset: u64,
}

pub struct File {
    data: FileBuffer,
    kind: Kind,
    size: u32,
}

impl File {
    pub fn kind(&self) -> Kind {
        self.kind.clone()
    }
    pub fn size(&self) -> u32 {
        self.size
    }

    fn assure_v2(&self) {
        assert!(
            if let Kind::V2 = self.kind.clone() {
                true
            } else {
                false
            },
            "Only V2 is implemented"
        );
    }

    pub fn entry(&self, offset: u64) -> Entry {
        self.assure_v2();
        assert!(offset <= usize::max_value() as u64);
        assert!(offset as usize <= self.data.len(), "offset out of bounds");

        let obj_begin = &self.data[offset as usize..];
        let (object, decompressed_size, consumed_bytes) = parsed::Object::from_bytes(obj_begin);
        Entry {
            object,
            decompressed_size,
            offset: offset + consumed_bytes,
        }
    }

    pub fn at(path: impl AsRef<Path>) -> Result<Self, Error> {
        let data =
            FileBuffer::open(path.as_ref()).map_err(|e| Error::Io(e, path.as_ref().to_owned()))?;
        let pack_len = data.len();
        if pack_len < N32_SIZE * 3 + SHA1_SIZE {
            return Err(Error::Corrupt(format!(
                "Pack file of size {} is too small for even an empty pack",
                pack_len
            )));
        }
        let mut ofs = 0;
        if &data[ofs..ofs + N32_SIZE] != b"PACK" {
            return Err(Error::Corrupt("Pack file type not recognized".into()));
        }
        ofs += N32_SIZE;
        let kind = match BigEndian::read_u32(&data[ofs..ofs + N32_SIZE]) {
            2 => Kind::V2,
            3 => Kind::V3,
            v => return Err(Error::UnsupportedVersion(v)),
        };
        ofs += N32_SIZE;
        let size = BigEndian::read_u32(&data[ofs..ofs + N32_SIZE]);

        Ok(File { data, kind, size })
    }
}

pub mod parsed {
    use crate::object;

    use crate::object::SHA1_SIZE;
    use std::mem;

    const _TYPE_EXT1: u8 = 0;
    const COMMIT: u8 = 1;
    const TREE: u8 = 2;
    const BLOB: u8 = 3;
    const TAG: u8 = 4;
    const _TYPE_EXT2: u8 = 5;
    const OFS_DELTA: u8 = 6;
    const REF_DELTA: u8 = 7;

    #[derive(PartialEq, Eq, Debug, Hash, Clone)]
    pub enum Object {
        Commit,
        Tree,
        Blob,
        Tag,
        RefDelta { oid: object::Id },
        OfsDelta { offset: u64 },
    }

    #[inline]
    fn leb64decode(
        d: &[u8],
        initial_result: Option<u64>,
        initial_shift: Option<usize>,
    ) -> (u64, usize) {
        let mut count = 0;
        let mut result = initial_result.unwrap_or(0);
        let mut shift = initial_shift.unwrap_or(0);

        for b in d {
            count += 1;
            result |= ((b & 0b0111_1111) as u64) << shift;
            shift += 7;
            if b & 0b1000_0000 == 0 {
                assert!(
                    shift + 1 - b.leading_zeros() as usize <= mem::size_of::<u64>() * 8,
                    "overflow, expected {} byte(s), got {} bits",
                    mem::size_of::<u64>(),
                    shift + 1 - b.leading_zeros() as usize
                );
                break;
            }
        }
        (result, count)
    }

    impl Object {
        pub fn from_bytes(d: &[u8]) -> (Object, u64, u64) {
            let c = d[0];
            let type_id = (c >> 4) & 0b0000_0111;
            let (size, leb_bytes) = leb64decode(&d[1..], Some((c & 15) as u64), Some(4));
            let mut consumed = 1 + leb_bytes;

            use self::Object::*;
            let object = match type_id {
                OFS_DELTA => {
                    let (offset, leb_bytes) = leb64decode(d, None, None);
                    let o = OfsDelta { offset };
                    consumed += leb_bytes;
                    o
                }
                REF_DELTA => {
                    let o = RefDelta {
                        oid: object::id_from_20_bytes(&d[..SHA1_SIZE]),
                    };
                    consumed += SHA1_SIZE;
                    o
                }
                BLOB => Blob,
                TREE => Tree,
                COMMIT => Commit,
                TAG => Tag,
                _ => panic!("We currently don't support any V3 features or extensions"),
            };
            (object, size, consumed as u64)
        }
    }
}
