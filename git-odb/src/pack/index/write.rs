use crate::{hash, loose, pack, pack::index::V2_SIGNATURE};
use byteorder::{BigEndian, ByteOrder};
use git_features::progress::Progress;
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
        IteratorInvariantTrailer {
            display("The iterator failed to set a trailing hash over all prior pack entries in the last provided entry")
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

enum Cache {
    UseResolve,
    Decompressed(Vec<u8>),
    /// compressed bytes + decompressed size
    Compressed(Vec<u8>, usize),
}

struct Entry {
    _id: owned::Id,
    _pack_offset: u64,
    _crc32: u32,
    _cache: Cache,
    /// When it reaches zero, the cache can be freed
    _child_count: u32,
}

/// The function resolves pack_offset: u64 into compressed bytes to &mut Vec<u8> and returns (object kind, decompressed size)
/// And it will be called after the iterator stopped returning elements.
pub enum Mode<F>
where
    F: Fn(u64, &mut Vec<u8>) -> (pack::data::Header, u64),
{
    /// Base + deltas in memory compressed
    InMemory,
    InMemoryDecompressed,
    /// Deltas in memory compressed
    ResolveBases(F),
    /// Bases in memory compressed
    ResolveDeltas(F),
    ResolveBasesAndDeltas(F),
}

impl<F> Mode<F>
where
    F: Fn(u64, &mut Vec<u8>) -> (pack::data::Header, u64),
{
    fn base_cache(&self, compressed: Vec<u8>, decompressed: Vec<u8>) -> Cache {
        match self {
            Mode::ResolveDeltas(_) | Mode::InMemory => Cache::Compressed(compressed, decompressed.len()),
            Mode::InMemoryDecompressed => Cache::Decompressed(decompressed),
            Mode::ResolveBases(_) | Mode::ResolveBasesAndDeltas(_) => Cache::UseResolve,
        }
    }
    fn delta_cache(&self, compressed: Vec<u8>, decompressed: Vec<u8>) -> Cache {
        match self {
            Mode::ResolveBases(_) | Mode::InMemory => Cache::Compressed(compressed, decompressed.len()),
            Mode::InMemoryDecompressed => Cache::Decompressed(decompressed),
            Mode::ResolveDeltas(_) | Mode::ResolveBasesAndDeltas(_) => Cache::UseResolve,
        }
    }
}

impl Mode<fn(u64, &mut Vec<u8>) -> (pack::data::Header, u64)> {
    pub fn in_memory() -> Self {
        Self::InMemory
    }
}

/// Various ways of writing an index file from pack entries
impl pack::index::File {
    /// Note that neither in-pack nor out-of-pack Ref Deltas are supported here, these must have been resolved beforehand.
    pub fn write_data_iter_to_stream<F>(
        kind: pack::index::Kind,
        mode: Mode<F>,
        entries: impl Iterator<Item = Result<pack::data::iter::Entry, pack::data::iter::Error>>,
        _progress: impl Progress,
        out: impl io::Write,
    ) -> Result<Outcome, Error>
    where
        F: for<'r> Fn(u64, &'r mut Vec<u8>) -> (pack::data::Header, u64),
    {
        use io::Write;

        let mut out = hash::Write::new(out, kind.hash());
        if kind != pack::index::Kind::default() {
            return Err(Error::Unsupported(kind));
        }
        let mut num_objects = 0;
        // This array starts out sorted by pack-offset
        let mut index_entries = Vec::with_capacity(entries.size_hint().0);
        let mut last_seen_trailer = None;
        for entry in entries {
            let pack::data::iter::Entry {
                header,
                pack_offset,
                header_size: _,
                compressed,
                decompressed,
                trailer,
            } = entry?;
            use pack::data::Header::*;
            num_objects += 1;
            let mut hash_write = hash::Write::new(io::sink(), kind.hash());
            let cache = match header {
                Blob | Tree | Commit | Tag => {
                    loose::object::header::encode(
                        header.to_kind().expect("non-delta kind"),
                        decompressed.len() as u64,
                        &mut hash_write,
                    )?;
                    hash_write.hash.update(&decompressed);
                    mode.base_cache(compressed, decompressed)
                }
                RefDelta { .. } => return Err(Error::RefDelta),
                OfsDelta { pack_offset: _ } => {
                    mode.delta_cache(compressed, decompressed)
                    // TODO find base and increment child count
                    // unimplemented!("apply a single delta to an offset we must have seen already")
                }
            };

            index_entries.push(Entry {
                _id: owned::Id::from(hash_write.hash.digest()),
                _pack_offset: pack_offset,
                _crc32: 0, // TBD, but can be done right here, needs header encoding
                _cache: cache,
                _child_count: 0,
            });
            last_seen_trailer = trailer;
        }

        out.write_all(V2_SIGNATURE)?;

        // Write header
        let mut buf = [0u8; 4];
        BigEndian::write_u32(&mut buf, kind as u32);
        out.write_all(&buf)?;

        // todo: write fanout

        let _index_hash = out.hash.digest();
        Ok(Outcome {
            index_kind: kind,
            index_hash: owned::Id::from([0u8; 20]),
            pack_hash: last_seen_trailer.ok_or(Error::IteratorInvariantTrailer)?,
            num_objects,
        })
    }
}
