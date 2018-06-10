use failure::{Error, ResultExt};

use byteorder::{BigEndian, ByteOrder};
use std::{mem::size_of, path::Path};
use filebuffer::FileBuffer;

use object::SHA1_SIZE;
use object;

const N32_SIZE: usize = size_of::<u32>();

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Kind {
    V2,
    V3,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Entry<'a> {
    pub oid: object::Id,
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

        let obj_begin = &self.data[offset as usize..];
        let (object, compressed) = parsed::Object::from_bytes(obj_begin);

        Entry {
            oid: [0; 20],
            object,
            compressed,
        }
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

    const _TYPE_EXT1: u8 = 0;
    const _COMMIT: u8 = 1;
    const _TREE: u8 = 2;
    const _BLOB: u8 = 3;
    const _TAG: u8 = 4;
    const _TYPE_EXT2: u8 = 5;
    const _OFS_DELTA: u8 = 6;
    const _REF_DELTA: u8 = 7;

    #[derive(PartialEq, Eq, Debug, Hash, Clone)]
    pub enum Object {
        Commit,
        Tree,
        Blob,
        Tag,
        RefDelta { oid: object::Id },
        OfsDelta { offset: u64 },
    }

    impl Object {
        pub fn from_bytes(d: &[u8]) -> (Object, &[u8]) {
            let c = d[0];
            let type_id = (c >> 4) & 0b0000_0111;

            let mut count = 1;
            let mut size = (c & 15) as u64;
            let mut shift = 4;

            for b in &d[1..] {
                count += 1;
                size |= ((b & 0b0111_1111) as u64) << shift;
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

            let d = &d[count..];
            unimplemented!()
        }
    }
}
