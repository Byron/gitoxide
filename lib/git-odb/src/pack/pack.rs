use failure::{Error, ResultExt};

use byteorder::{BigEndian, ByteOrder};
use std::{mem::size_of, path::Path};
use filebuffer::FileBuffer;

use object::SHA1_SIZE;

const N32_SIZE: usize = size_of::<u32>();

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Kind {
    V2,
    V3,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Entry<'a> {
    pub object: parsed::Object,
    pub compressed: &'a [u8],
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

    pub fn entry<'a>(&'a self, offset: u64) -> Entry<'a> {
        self.assure_v2();
        assert!(offset <= usize::max_value() as u64);
        assert!(offset as usize <= self.data.len(), "offset out of bounds");

        let obj_begin = &self.data[offset as usize..];
        let (object, compressed) = parsed::Object::from_bytes(obj_begin);

        Entry { object, compressed }
    }

    pub fn at(path: &Path) -> Result<Self, Error> {
        let data = FileBuffer::open(path)
            .with_context(|_| format!("Could not map pack file at '{}'", path.display()))?;
        let pack_len = data.len();
        if pack_len < N32_SIZE * 3 + SHA1_SIZE {
            bail!(
                "Pack file of size {} is too small for even an empty pack",
                pack_len
            );
        }
        let mut ofs = 0;
        if &data[ofs..ofs + N32_SIZE] != b"PACK" {
            bail!("Pack file type not recognized")
        }
        ofs += N32_SIZE;
        let kind = match BigEndian::read_u32(&data[ofs..ofs + N32_SIZE]) {
            2 => Kind::V2,
            3 => Kind::V3,
            v @ _ => bail!("Unknown pack version: {}", v),
        };
        ofs += N32_SIZE;
        let size = BigEndian::read_u32(&data[ofs..ofs + N32_SIZE]);

        Ok(File { data, kind, size })
    }
}

pub mod parsed {
    use object;

    use std::mem;
    use object::SHA1_SIZE;

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
    ) -> (u64, &[u8]) {
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

        (result, &d[count..])
    }

    impl Object {
        pub fn from_bytes(d: &[u8]) -> (Object, &[u8]) {
            println!("original d.len == {}", d.len());
            let c = d[0];
            let type_id = (c >> 4) & 0b0000_0111;
            let (size, mut d) = leb64decode(&d[1..], Some((c & 15) as u64), Some(4));
            let size = size as usize;
            use self::Object::*;
            println!("type {} d[..{}] of {}", type_id, size, d.len());
            let object = match type_id {
                OFS_DELTA => {
                    let (offset, nd) = leb64decode(d, None, None);
                    let o = OfsDelta { offset };
                    d = nd;
                    o
                }
                REF_DELTA => {
                    let o = RefDelta {
                        oid: object::id_from_20_bytes(&d[..SHA1_SIZE]),
                    };
                    d = &d[SHA1_SIZE..];
                    o
                }
                BLOB => {
                    d = &d[..size];
                    Blob
                }
                TREE => {
                    d = &d[..size];
                    Tree
                }
                COMMIT => {
                    d = &d[..size];
                    Commit
                }
                TAG => {
                    d = &d[..size];
                    Tag
                }
                _ => panic!("We currently don't support any V3 features or extensions"),
            };
            (object, d)
        }
    }
}
