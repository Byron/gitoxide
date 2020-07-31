use crate::pack;
use git_features::parallel;
use git_object::owned;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Outcome {
    pub index_kind: pack::index::Kind,
    pub index_hash: owned::Id,
    pub pack_hash: owned::Id,
    pub num_objects: u32,
}

pub(crate) enum Cache {
    Unset,
    Decompressed(Vec<u8>),
    /// compressed bytes + decompressed size
    Compressed(Vec<u8>, usize),
}

pub(crate) struct Entry {
    pub is_base: bool,
    pub pack_offset: u64,
    pub crc32: u32,
}

pub(crate) struct CacheEntry {
    pub _cache: Cache,
    /// When it reaches zero, the cache can be freed
    pub child_count: u32,
}

impl CacheEntry {
    pub fn _decr(&mut self) {
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
    pub(crate) fn base_cache(&self, compressed: Vec<u8>, decompressed: Vec<u8>) -> Cache {
        match self {
            Mode::ResolveDeltas(_) | Mode::InMemory => Cache::Compressed(compressed, decompressed.len()),
            Mode::InMemoryDecompressed => Cache::Decompressed(decompressed),
            Mode::ResolveBases(_) | Mode::ResolveBasesAndDeltas(_) => Cache::Unset,
        }
    }
    pub(crate) fn delta_cache(&self, compressed: Vec<u8>, decompressed: Vec<u8>) -> Cache {
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

pub(crate) struct Reducer {
    pub(crate) items: Vec<(u64, owned::Id, u32)>,
}

impl Reducer {
    pub fn new(num_objects: u32) -> Self {
        Reducer {
            items: Vec::with_capacity(num_objects as usize),
        }
    }
}

impl parallel::Reducer for Reducer {
    type Input = Vec<(u64, owned::Id)>;
    type Output = Vec<(u64, owned::Id, u32)>;
    type Error = pack::index::write::Error;

    fn feed(&mut self, input: Self::Input) -> Result<(), Self::Error> {
        self.items
            .extend(input.into_iter().map(|(pack_offset, id)| (pack_offset, id, 0)));
        Ok(())
    }

    fn finalize(self) -> Result<Self::Output, Self::Error> {
        Ok(self.items)
    }
}
