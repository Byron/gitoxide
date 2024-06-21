use std::io;

use gix_features::decode::{leb64, leb64_from_read};

use super::{BLOB, COMMIT, OFS_DELTA, REF_DELTA, TAG, TREE};
use crate::data;

/// The error returned by [data::Entry::from_bytes()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
#[error("Object type {type_id} is unsupported")]
pub struct Error {
    pub type_id: u8,
}

/// Decoding
impl data::Entry {
    /// Decode an entry from the given entry data `d`, providing the `pack_offset` to allow tracking the start of the entry data section.
    ///
    /// # Panics
    ///
    /// If we cannot understand the header, garbage data is likely to trigger this.
    pub fn from_bytes(d: &[u8], pack_offset: data::Offset, hash_len: usize) -> Result<data::Entry, Error> {
        let (type_id, size, mut consumed) = parse_header_info(d);

        use crate::data::entry::Header::*;
        let object = match type_id {
            OFS_DELTA => {
                let (distance, leb_bytes) = leb64(&d[consumed..]);
                let delta = OfsDelta {
                    base_distance: distance,
                };
                consumed += leb_bytes;
                delta
            }
            REF_DELTA => {
                let delta = RefDelta {
                    base_id: gix_hash::ObjectId::from_bytes_or_panic(&d[consumed..][..hash_len]),
                };
                consumed += hash_len;
                delta
            }
            BLOB => Blob,
            TREE => Tree,
            COMMIT => Commit,
            TAG => Tag,
            other => return Err(Error { type_id: other }),
        };
        Ok(data::Entry {
            header: object,
            decompressed_size: size,
            data_offset: pack_offset + consumed as u64,
        })
    }

    /// Instantiate an `Entry` from the reader `r`, providing the `pack_offset` to allow tracking the start of the entry data section.
    pub fn from_read(r: &mut dyn io::Read, pack_offset: data::Offset, hash_len: usize) -> io::Result<data::Entry> {
        let (type_id, size, mut consumed) = streaming_parse_header_info(r)?;

        use crate::data::entry::Header::*;
        let object = match type_id {
            OFS_DELTA => {
                let (distance, leb_bytes) = leb64_from_read(r)?;
                let delta = OfsDelta {
                    base_distance: distance,
                };
                consumed += leb_bytes;
                delta
            }
            REF_DELTA => {
                let mut buf = gix_hash::Kind::buf();
                let hash = &mut buf[..hash_len];
                r.read_exact(hash)?;
                #[allow(clippy::redundant_slicing)]
                let delta = RefDelta {
                    base_id: gix_hash::ObjectId::from_bytes_or_panic(&hash[..]),
                };
                consumed += hash_len;
                delta
            }
            BLOB => Blob,
            TREE => Tree,
            COMMIT => Commit,
            TAG => Tag,
            other => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Object type {other} is unsupported"),
                ))
            }
        };
        Ok(data::Entry {
            header: object,
            decompressed_size: size,
            data_offset: pack_offset + consumed as u64,
        })
    }
}

#[inline]
fn streaming_parse_header_info(read: &mut dyn io::Read) -> Result<(u8, u64, usize), io::Error> {
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
