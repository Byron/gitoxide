use crate::{hash, loose, pack, pack::index::V2_SIGNATURE};
use byteorder::{BigEndian, ByteOrder};
use git_object::owned;
use quick_error::quick_error;
use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            display("An IO error occurred when reading the pack or creating a temporary file")
            from()
            source(err)
        }
        PackEntryDecode(err: pack::data::iter::Error) {
            display("A pack entry could not be extracted")
            from()
            source(err)
        }
        Unsupported(kind: pack::index::Kind) {
            display("Indices of type {} cannot be written, only {} are supported", *kind as usize, pack::index::Kind::default() as usize)
        }
        RefDelta {
            display("Ref delta objects are not supported as there is no way to look them up. Resolve them beforehand.")
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Outcome {
    pub index_kind: pack::index::Kind,
    pub index_hash: owned::Id,
    pub pack_hash: owned::Id,
    pub num_objects: u32,
}

struct Entry {
    _id: owned::Id,
    _pack_offset: u64,
    _crc32: u32,
    /// Only set if we don't have a resolve function, so we keep all bases in memory
    _base_bytes: Option<Vec<u8>>,
}

/// Various ways of writing an index file from pack entries
impl pack::index::File {
    /// Note that neither in-pack nor out-of-pack Ref Deltas are supported here, these must have been resolved beforehand.
    pub fn write_to_stream(
        entries: impl Iterator<Item = Result<pack::data::iter::Entry, pack::data::iter::Error>>,
        out: impl io::Write,
        kind: pack::index::Kind,
    ) -> Result<Outcome, Error> {
        use io::Write;

        let mut out = hash::Write::new(out, kind.hash());
        if kind != pack::index::Kind::default() {
            return Err(Error::Unsupported(kind));
        }
        let mut num_objects = 0;
        // This array starts our sorted by pack-offset
        let mut index_entries = Vec::with_capacity(entries.size_hint().0);
        for entry in entries {
            let pack::data::iter::Entry {
                header,
                pack_offset,
                header_size: _,
                compressed: _,
                decompressed,
                trailer: _,
            } = entry?;
            use pack::data::Header::*;
            num_objects += 1;
            let mut hash_write = hash::Write::new(io::sink(), kind.hash());
            let decompressed = match header {
                Blob | Tree | Commit | Tag => {
                    loose::object::header::encode(
                        header.to_kind().expect("non-delta kind"),
                        decompressed.len() as u64,
                        &mut hash_write,
                    )?;
                    hash_write.hash.update(&decompressed);
                    Some(decompressed)
                }
                RefDelta { .. } => return Err(Error::RefDelta),
                OfsDelta { pack_offset: _ } => {
                    unimplemented!("apply a single delta to an offset we must have seen already")
                }
            };

            index_entries.push(Entry {
                _id: owned::Id::from(hash_write.hash.digest()),
                _pack_offset: pack_offset,
                _crc32: 0, // TBD
                _base_bytes: decompressed,
            })
        }

        out.write_all(V2_SIGNATURE)?;

        let mut buf = [0u8; 4];
        BigEndian::write_u32(&mut buf, kind as u32);
        out.write_all(&buf)?;

        let _index_hash = out.hash.digest();
        Ok(Outcome {
            index_kind: kind,
            index_hash: owned::Id::from([0u8; 20]),
            pack_hash: owned::Id::from([0u8; 20]),
            num_objects,
        })
    }
}
