use crate::{hash, pack, pack::index::V2_SIGNATURE};
use byteorder::{BigEndian, WriteBytesExt};
use git_features::{parallel, parallel::in_parallel_if, progress::Progress};
use git_object::owned;
use quick_error::quick_error;
use smallvec::alloc::collections::BTreeMap;
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
        IteratorInvariantNonEmpty {
            display("Is there ever a need to create empty indices? If so, please post a PR.")
        }
        IteratorInvariantBasesPresent {
            display("Did not find a single base")
        }
        IteratorInvariantBasesBeforeDeltasNeedThem(delta_pack_offset: u64, base_pack_offset: u64) {
            display("The delta at pack offset {} could not find its base at {} - it should have been seen already", delta_pack_offset, base_pack_offset)
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
    Unset,
    Decompressed(Vec<u8>),
    /// compressed bytes + decompressed size
    Compressed(Vec<u8>, usize),
}

struct Entry {
    is_base: bool,
    _pack_offset: u64,
    _id: Option<owned::Id>,
    _crc32: u32,
}

struct CacheEntry {
    _cache: Cache,
    /// When it reaches zero, the cache can be freed
    child_count: u32,
}

impl CacheEntry {
    fn _decr(&mut self) {
        self.child_count -= 1;
        if self.child_count == 0 {
            self._cache = Cache::Unset;
        }
    }
}

/// The function resolves pack_offset: u64 into compressed bytes to &mut Vec<u8> and returns (object kind, decompressed size)
/// And it will be called after the iterator stopped returning elements.
pub enum Mode<F>
where
    F: Fn(u64, &mut Vec<u8>) -> Option<(pack::data::Header, u64)>,
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
    F: Fn(u64, &mut Vec<u8>) -> Option<(pack::data::Header, u64)>,
{
    fn base_cache(&self, compressed: Vec<u8>, decompressed: Vec<u8>) -> Cache {
        match self {
            Mode::ResolveDeltas(_) | Mode::InMemory => Cache::Compressed(compressed, decompressed.len()),
            Mode::InMemoryDecompressed => Cache::Decompressed(decompressed),
            Mode::ResolveBases(_) | Mode::ResolveBasesAndDeltas(_) => Cache::Unset,
        }
    }
    fn delta_cache(&self, compressed: Vec<u8>, decompressed: Vec<u8>) -> Cache {
        match self {
            Mode::ResolveBases(_) | Mode::InMemory => Cache::Compressed(compressed, decompressed.len()),
            Mode::InMemoryDecompressed => Cache::Decompressed(decompressed),
            Mode::ResolveDeltas(_) | Mode::ResolveBasesAndDeltas(_) => Cache::Unset,
        }
    }
}

impl Mode<fn(u64, &mut Vec<u8>) -> Option<(pack::data::Header, u64)>> {
    pub fn in_memory() -> Self {
        Self::InMemory
    }
    pub fn in_memory_decompressed() -> Self {
        Self::InMemoryDecompressed
    }
}

struct Reducer;

impl parallel::Reducer for Reducer {
    type Input = Vec<owned::Id>;
    type Output = ();
    type Error = Error;

    fn feed(&mut self, _input: Self::Input) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn finalize(self) -> Result<Self::Output, Self::Error> {
        unimplemented!()
    }
}

/// Various ways of writing an index file from pack entries
impl pack::index::File {
    /// Note that neither in-pack nor out-of-pack Ref Deltas are supported here, these must have been resolved beforehand.
    pub fn write_data_iter_to_stream<F>(
        kind: pack::index::Kind,
        mode: Mode<F>,
        entries: impl Iterator<Item = Result<pack::data::iter::Entry, pack::data::iter::Error>>,
        thread_limit: Option<usize>,
        _progress: impl Progress,
        out: impl io::Write,
    ) -> Result<Outcome, Error>
    where
        F: for<'r> Fn(u64, &'r mut Vec<u8>) -> Option<(pack::data::Header, u64)>,
    {
        use io::Write;

        let mut out = hash::Write::new(out, kind.hash());
        if kind != pack::index::Kind::default() {
            return Err(Error::Unsupported(kind));
        }
        let mut num_objects = 0;
        let mut bytes_to_process = 0u64;
        // This array starts out sorted by pack-offset
        let mut index_entries = Vec::with_capacity(entries.size_hint().0);
        if index_entries.capacity() == 0 {
            return Err(Error::IteratorInvariantNonEmpty);
        }
        let mut last_seen_trailer = None;
        let mut last_base_index = None;
        // TODO: This should soon become a dashmap (in fast mode, or a Mutex protected shared map) as this will be edited
        // by threads to remove now unused caches. Probably also a good moment to switch to parking lot mutexes everywhere.
        let mut cache_by_offset = BTreeMap::<_, CacheEntry>::new();
        for (eid, entry) in entries.enumerate() {
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
            bytes_to_process += decompressed.len() as u64;
            let (cache, _is_base) = match header {
                Blob | Tree | Commit | Tag => {
                    last_base_index = Some(eid);
                    (mode.base_cache(compressed, decompressed), true)
                }
                RefDelta { .. } => return Err(Error::RefDelta),
                OfsDelta {
                    pack_offset: base_pack_offset,
                } => {
                    cache_by_offset
                        .get_mut(&base_pack_offset)
                        .ok_or_else(|| {
                            Error::IteratorInvariantBasesBeforeDeltasNeedThem(pack_offset, base_pack_offset)
                        })?
                        .child_count += 1;
                    (mode.delta_cache(compressed, decompressed), false)
                }
            };

            cache_by_offset.insert(
                pack_offset,
                CacheEntry {
                    child_count: 0,
                    _cache: cache,
                },
            );
            index_entries.push(Entry {
                is_base: _is_base,
                _pack_offset: pack_offset,
                _id: None,
                _crc32: 0, // TBD, but can be done right here, needs header encoding
            });
            last_seen_trailer = trailer;
        }

        // Prevent us from trying to find bases for resolution past the point where they are
        let last_base_index = last_base_index.ok_or(Error::IteratorInvariantBasesPresent)?;
        in_parallel_if(
            || bytes_to_process > 5_000_000,
            index_entries.iter().take(last_base_index).filter(|e| e.is_base), // TODO: chunking
            thread_limit,
            |_| (),
            |_base_pack_offset, _state| Vec::new(),
            Reducer,
        )?;

        // Write header
        out.write_all(V2_SIGNATURE)?;
        out.write_u32::<BigEndian>(kind as u32)?;

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
