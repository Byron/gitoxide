use git_object as object;
use object::SHA1_SIZE;

const _TYPE_EXT1: u8 = 0;
const COMMIT: u8 = 1;
const TREE: u8 = 2;
const BLOB: u8 = 3;
const TAG: u8 = 4;
const _TYPE_EXT2: u8 = 5;
const OFS_DELTA: u8 = 6;
const REF_DELTA: u8 = 7;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry {
    pub header: Header,
    /// The decompressed size of the object in bytes
    pub decompressed_size: u64,
    /// absolute offset to compressed object data in the pack
    pub data_offset: u64,
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Header {
    Commit,
    Tree,
    Blob,
    Tag,
    /// An object within this pack if the LSB encoded offset would be larger than 20 bytes
    /// Alternatively an object stored in the repository, if this is a thin pack
    RefDelta {
        oid: object::Id,
    },
    /// The offset into the pack at which to find the base object header
    OfsDelta {
        pack_offset: u64,
    },
}

impl Header {
    pub fn to_kind(&self) -> Option<object::Kind> {
        use object::Kind::*;
        Some(match self {
            Header::Tree => Tree,
            Header::Blob => Blob,
            Header::Commit => Commit,
            Header::Tag => Tag,
            Header::RefDelta { .. } | Header::OfsDelta { .. } => return None,
        })
    }
    pub fn is_delta(&self) -> bool {
        match self {
            Header::OfsDelta { .. } | Header::RefDelta { .. } => true,
            _ => false,
        }
    }
}

#[inline]
fn leb64decode(d: &[u8]) -> (u64, usize) {
    let mut i = 0;
    let mut c = d[i];
    i += 1;
    let mut value = c as u64 & 0x7f;
    while c & 0x80 != 0 {
        c = d[i];
        i += 1;
        value += 1;
        value = (value << 7) + (c as u64 & 0x7f)
    }
    (value, i)
}

/// Parses the header of a pack-entry, yielding object type id, decompressed object size, and consumed bytes
fn parse_header_info(data: &[u8]) -> (u8, u64, usize) {
    let mut c = data[0];
    let mut i = 1;
    let type_id = (c >> 4) & 0b0000_0111;
    let mut size = c as u64 & 0b0000_1111;
    let mut s = 4;
    while c & 0b1000_0000 != 0 {
        c = data[i];
        i += 1;
        size += ((c & 0b0111_1111) as u64) << s;
        s += 7
    }
    (type_id, size, i)
}

impl Header {
    pub fn from_bytes(d: &[u8], pack_offset: u64) -> (Header, u64, u64) {
        let (type_id, size, mut consumed) = parse_header_info(d);

        use self::Header::*;
        let object = match type_id {
            OFS_DELTA => {
                let (offset, leb_bytes) = leb64decode(&d[consumed..]);
                let delta = OfsDelta {
                    pack_offset: pack_offset - offset,
                };
                consumed += leb_bytes;
                delta
            }
            REF_DELTA => {
                let delta = RefDelta {
                    oid: object::Id::from_20_bytes(&d[consumed..consumed + SHA1_SIZE]),
                };
                consumed += SHA1_SIZE;
                delta
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
