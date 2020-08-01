use git_object::{owned, SHA1_SIZE};
use std::io;

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
    /// The amount of bytes used to encode the header
    pub header_size: u8,
    /// absolute offset to compressed object data in the pack, just behind the header
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
        base_id: owned::Id,
    },
    /// The offset into the pack at which to find the base object header
    OfsDelta {
        base_pack_offset: u64,
    },
}
impl Header {
    pub fn to_kind(&self) -> Option<git_object::Kind> {
        use git_object::Kind::*;
        Some(match self {
            Header::Tree => Tree,
            Header::Blob => Blob,
            Header::Commit => Commit,
            Header::Tag => Tag,
            Header::RefDelta { .. } | Header::OfsDelta { .. } => return None,
        })
    }
    pub fn to_type_id(&self) -> u8 {
        use Header::*;
        match self {
            Blob => BLOB,
            Tree => TREE,
            Commit => COMMIT,
            Tag => TAG,
            OfsDelta { .. } => OFS_DELTA,
            RefDelta { .. } => REF_DELTA,
        }
    }
    pub fn is_delta(&self) -> bool {
        match self {
            Header::OfsDelta { .. } | Header::RefDelta { .. } => true,
            _ => false,
        }
    }
}

impl Header {
    pub fn from_bytes(d: &[u8], pack_offset: u64) -> Entry {
        let (type_id, size, mut consumed) = parse_header_info(d);

        use self::Header::*;
        let object = match type_id {
            OFS_DELTA => {
                let (distance, leb_bytes) = leb64decode(&d[consumed..]);
                let delta = OfsDelta {
                    base_pack_offset: pack_offset - distance,
                };
                consumed += leb_bytes;
                delta
            }
            REF_DELTA => {
                let delta = RefDelta {
                    base_id: owned::Id::from_20_bytes(&d[consumed..consumed + SHA1_SIZE]),
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
        Entry {
            header: object,
            header_size: consumed as u8,
            decompressed_size: size,
            data_offset: 0,
        }
    }

    pub fn from_read(mut r: impl io::Read, pack_offset: u64) -> Result<Entry, io::Error> {
        let (type_id, size, mut consumed) = streaming_parse_header_info(&mut r)?;

        use self::Header::*;
        let object = match type_id {
            OFS_DELTA => {
                let (distance, leb_bytes) = streaming_leb64decode(&mut r)?;
                let delta = OfsDelta {
                    base_pack_offset: pack_offset.checked_sub(distance).ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::Other,
                            format!(
                                "Computing the absolute pack offset would underflow: {} - {}",
                                pack_offset, distance
                            ),
                        )
                    })?,
                };
                consumed += leb_bytes;
                delta
            }
            REF_DELTA => {
                let mut buf = [0u8; SHA1_SIZE];
                r.read_exact(&mut buf)?;
                let delta = RefDelta {
                    base_id: owned::Id::new_sha1(buf),
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
        Ok(Entry {
            header: object,
            header_size: consumed as u8,
            decompressed_size: size,
            data_offset: 0,
        })
    }

    pub fn to_write(
        &self,
        decompressed_size_in_bytes: u64,
        pack_offset: u64,
        mut out: impl io::Write,
    ) -> io::Result<usize> {
        let mut size = decompressed_size_in_bytes;
        let mut written = 1;
        let mut c: u8 = (self.to_type_id() << 4) | (size as u8 & 0b0000_1111);
        size >>= 4;
        while size != 0 {
            out.write_all(&[c | 0b1000_0000])?;
            written += 1;
            c = size as u8 & 0b0111_1111;
            size >>= 7;
        }
        out.write_all(&[c])?;

        use Header::*;
        match self {
            RefDelta { base_id: oid } => {
                out.write_all(oid.as_slice())?;
                written += oid.as_slice().len();
            }
            OfsDelta { base_pack_offset } => {
                let mut distance = pack_offset
                    .checked_sub(*base_pack_offset)
                    .expect("base entry to be before this entry");
                let mut buf = [0u8; 10];
                let mut bytes_written = 1;
                buf[buf.len() - 1] = distance as u8 & 0b0111_1111;
                for out in buf.iter_mut().rev().skip(1) {
                    distance >>= 7;
                    if distance == 0 {
                        break;
                    }
                    distance -= 1;
                    *out = 0b1000_0000 | (distance as u8 & 0b0111_1111);
                    bytes_written += 1;
                }
                out.write_all(&buf[buf.len() - bytes_written..])?;
                written += bytes_written;
            }
            Blob | Tree | Commit | Tag => {}
        }
        Ok(written)
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

#[inline]
fn streaming_leb64decode(mut r: impl io::Read) -> Result<(u64, usize), io::Error> {
    let mut b = [0u8; 1];
    let mut i = 0;
    r.read_exact(&mut b)?;
    i += 1;
    let mut value = b[0] as u64 & 0x7f;
    while b[0] & 0x80 != 0 {
        r.read_exact(&mut b)?;
        i += 1;
        value += 1;
        value = (value << 7) + (b[0] as u64 & 0x7f)
    }
    Ok((value, i))
}

/// Parses the header of a pack-entry, yielding object type id, decompressed object size, and consumed bytes
#[inline]
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

#[inline]
fn streaming_parse_header_info(mut read: impl io::Read) -> Result<(u8, u64, usize), io::Error> {
    let mut byte = [0u8; 1];
    read.read_exact(&mut byte)?;
    let mut c = byte[0];
    let mut i = 1;
    let type_id = (c >> 4) & 0b0000_0111;
    let mut size = c as u64 & 0b0000_1111;
    let mut s = 4;
    while c & 0b1000_0000 != 0 {
        read.read_exact(&mut byte)?;
        c = byte[0];
        i += 1;
        size += ((c & 0b0111_1111) as u64) << s;
        s += 7
    }
    Ok((type_id, size, i))
}
