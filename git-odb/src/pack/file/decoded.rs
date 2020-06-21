use git_object as object;
use object::SHA1_SIZE;
use std::mem;

const _TYPE_EXT1: u8 = 0;
const COMMIT: u8 = 1;
const TREE: u8 = 2;
const BLOB: u8 = 3;
const TAG: u8 = 4;
const _TYPE_EXT2: u8 = 5;
const OFS_DELTA: u8 = 6;
const REF_DELTA: u8 = 7;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
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
    /// The offset from the place this header is located at, pointing to the objects base
    OfsDelta {
        offset: u64,
    },
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
            debug_assert!(
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

impl Header {
    pub fn from_bytes(d: &[u8]) -> (Header, u64, u64) {
        let c = d[0];
        let type_id = (c >> 4) & 0b0000_0111;
        let (size, leb_bytes) = leb64decode(&d[1..], Some((c & 0b0000_1111) as u64), Some(4));
        let mut consumed = 1 + leb_bytes;

        use self::Header::*;
        let object = match type_id {
            OFS_DELTA => {
                let (offset, leb_bytes) = leb64decode(&d[consumed..], None, None);
                let delta = OfsDelta { offset };
                consumed += leb_bytes;
                delta
            }
            REF_DELTA => {
                let delta = RefDelta {
                    oid: object::id_from_20_bytes(&d[consumed..consumed + SHA1_SIZE]),
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
