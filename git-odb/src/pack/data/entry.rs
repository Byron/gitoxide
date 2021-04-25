use git_hash::SIZE_OF_SHA1_DIGEST as SHA1_SIZE;
use std::io;

const _TYPE_EXT1: u8 = 0;
const COMMIT: u8 = 1;
const TREE: u8 = 2;
const BLOB: u8 = 3;
const TAG: u8 = 4;
const _TYPE_EXT2: u8 = 5;
const OFS_DELTA: u8 = 6;
const REF_DELTA: u8 = 7;

/// An representing an full- or delta-object within a pack
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry {
    /// The entry's header
    pub header: Header,
    /// The decompressed size of the object in bytes
    pub decompressed_size: u64,
    /// absolute offset to compressed object data in the pack, just behind the entry's header
    pub data_offset: u64,
}

/// Access
impl Entry {
    /// Compute the pack offset to the base entry of the object represented by this entry.
    pub fn base_pack_offset(&self, distance: u64) -> u64 {
        let pack_offset = self.data_offset - self.header_size() as u64;
        pack_offset.checked_sub(distance).expect("in-bound distance of deltas")
    }
    /// The pack offset at which this entry starts
    pub fn pack_offset(&self) -> u64 {
        self.data_offset - self.header_size() as u64
    }
    /// The amount of bytes used to describe this entry in the pack. The header starts at [`Self::pack_offset()`]
    pub fn header_size(&self) -> usize {
        self.header
            .to_write(self.decompressed_size, io::sink())
            .expect("io::sink() to never fail")
    }
}

/// Decoding
impl Entry {
    /// Decode an entry from the given entry data `d`, providing the `pack_offset` to allow tracking the start of the entry data section.
    ///
    /// # Panics
    ///
    /// If we cannot understand the header, garbage data is likely to trigger this.
    pub fn from_bytes(d: &[u8], pack_offset: u64) -> Entry {
        let (type_id, size, mut consumed) = parse_header_info(d);

        use self::Header::*;
        let object = match type_id {
            OFS_DELTA => {
                let (distance, leb_bytes) = leb64decode(&d[consumed..]);
                let delta = OfsDelta {
                    base_distance: distance,
                };
                consumed += leb_bytes;
                delta
            }
            REF_DELTA => {
                let delta = RefDelta {
                    base_id: git_hash::ObjectId::from_20_bytes(&d[consumed..consumed + SHA1_SIZE]),
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
            decompressed_size: size,
            data_offset: pack_offset + consumed as u64,
        }
    }

    /// Instantiate an `Entry` from the reader `r`, providing the `pack_offset` to allow tracking the start of the entry data section.
    pub fn from_read(mut r: impl io::Read, pack_offset: u64) -> Result<Entry, io::Error> {
        let (type_id, size, mut consumed) = streaming_parse_header_info(&mut r)?;

        use self::Header::*;
        let object = match type_id {
            OFS_DELTA => {
                let (distance, leb_bytes) = streaming_leb64decode(&mut r)?;
                let delta = OfsDelta {
                    base_distance: distance,
                };
                consumed += leb_bytes;
                delta
            }
            REF_DELTA => {
                let mut buf = [0u8; SHA1_SIZE];
                r.read_exact(&mut buf)?;
                let delta = RefDelta {
                    base_id: git_hash::ObjectId::new_sha1(buf),
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
            decompressed_size: size,
            data_offset: pack_offset + consumed as u64,
        })
    }
}

/// The header portion of a pack data entry, identifying the kind of stored object.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub enum Header {
    /// The object is a commit
    Commit,
    /// The object is a tree
    Tree,
    /// The object is a blob
    Blob,
    /// The object is a tag
    Tag,
    /// Describes a delta-object which needs to be applied to a base. The base object is identified by the `base_id` field
    /// which is found within the parent repository.
    /// Most commonly used for **thin-packs** when receiving pack files from the server to refer to objects that are not
    /// part of the pack but expected to be present in the receivers repository.
    ///
    /// # Note
    /// This could also be an object within this pack if the LSB encoded offset would be larger than 20 bytes, which is unlikely to
    /// happen.
    ///
    /// **The naming** is exactly the same as the canonical implementation uses, namely **REF_DELTA**.
    RefDelta { base_id: git_hash::ObjectId },
    /// Describes a delta-object present in this pack which acts as base for this object.
    /// The base object is measured as a distance from this objects
    /// pack offset, so that `base_pack_offset = this_objects_pack_offset - base_distance`
    ///
    /// # Note
    ///
    /// **The naming** is exactly the same as the canonical implementation uses, namely **OFS_DELTA**.
    OfsDelta { base_distance: u64 },
}

impl Header {
    /// Subtract `distance` from `pack_offset` safely without the chance for overflow or no-ops if `distance` is 0.
    pub fn verified_base_pack_offset(pack_offset: u64, distance: u64) -> Option<u64> {
        if distance == 0 {
            return None;
        }
        pack_offset.checked_sub(distance)
    }
    /// Convert the header's object kind into [`git_object::Kind`] if possible
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
    /// Convert this header's object kind into the packs internal representation
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
    /// Return's true if this is a delta object, i.e. not a full object.
    pub fn is_delta(&self) -> bool {
        matches!(self, Header::OfsDelta { .. } | Header::RefDelta { .. })
    }
    /// Return's true if this is a base object, i.e. not a delta object.
    pub fn is_base(&self) -> bool {
        !self.is_delta()
    }
}

impl Header {
    /// Encode this header along the given `decompressed_size_in_bytes` into the `out` write stream for use within a data pack.
    ///
    /// Returns the amount of bytes written to `out`.
    /// `decompressed_size_in_bytes` is the full size in bytes of the object that this header represents
    pub fn to_write(&self, decompressed_size_in_bytes: u64, mut out: impl io::Write) -> io::Result<usize> {
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
            OfsDelta { mut base_distance } => {
                let mut buf = [0u8; 10];
                let mut bytes_written = 1;
                buf[buf.len() - 1] = base_distance as u8 & 0b0111_1111;
                for out in buf.iter_mut().rev().skip(1) {
                    base_distance >>= 7;
                    if base_distance == 0 {
                        break;
                    }
                    base_distance -= 1;
                    *out = 0b1000_0000 | (base_distance as u8 & 0b0111_1111);
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
