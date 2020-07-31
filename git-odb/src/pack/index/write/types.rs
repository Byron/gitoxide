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

pub(crate) enum ObjectKind {
    Base(git_object::Kind),
    OfsDelta(u64),
}

impl ObjectKind {
    pub fn is_base(&self) -> bool {
        match self {
            ObjectKind::Base(_) => true,
            ObjectKind::OfsDelta(_) => false,
        }
    }
    pub fn to_kind(&self) -> Option<git_object::Kind> {
        match self {
            ObjectKind::Base(kind) => Some(*kind),
            ObjectKind::OfsDelta(_) => None,
        }
    }
}

pub(crate) struct Entry {
    pub pack_offset: u64,
    pub entry_len: u64,
    pub kind: ObjectKind,
    pub crc32: u32,
}

pub(crate) struct CacheEntry {
    cache: Cache,
    /// When it reaches zero, the cache can be freed
    child_count: u32,
}

pub(crate) enum Bytes {
    Owned(Cache),
    Borrowed(Cache),
}

/// Note that every operation in the CacheEntry must be fast, as these happen behind a lock
impl CacheEntry {
    pub fn new(cache: Cache) -> Self {
        CacheEntry {
            child_count: 0,
            cache: cache,
        }
    }
    pub fn increment_child_count(&mut self) {
        self.child_count += 1;
    }
    pub fn _decr(&mut self) -> Bytes {
        self.child_count -= 1;
        self.cache()
    }

    pub fn cache(&mut self) -> Bytes {
        let cache = std::mem::replace(&mut self.cache, Cache::Unset);
        if self.child_count == 0 {
            Bytes::Owned(cache)
        } else {
            Bytes::Borrowed(cache)
        }
    }
    pub fn set_decompressed(&mut self, bytes: Vec<u8>) {
        assert_ne!(self.child_count, 0, "Do not return decompressed bytes once nobody is interested in the data anymore, i.e. from `Bytes::Owned(…)`");
        self.cache = Cache::Decompressed(bytes);
    }
}

pub struct ResolveContext {
    pack_offset: u64,
    /// The size of the bytes of the entry directly from `pack_offset`, allowing `&pack[pack_offset..pack_offset+entry_size]`
    entry_size: u64,
}

/// The function an entry into all of its bytes written to &mut Vec<u8> which is big enough and returns to true if bytes
/// were written, false otherwise. The latter should never have to happen, but is an escape hatch if something goes very wrong
/// when reading the pack entry.
/// It will only be called after the iterator stopped returning elements.
pub enum Mode<F>
where
    F: Fn(ResolveContext, &mut Vec<u8>) -> bool,
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
    F: Fn(ResolveContext, &mut Vec<u8>) -> bool,
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

impl Mode<fn(ResolveContext, &mut Vec<u8>) -> bool> {
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
    type Input = Result<Vec<(u64, owned::Id)>, pack::index::write::Error>;
    type Output = Vec<(u64, owned::Id, u32)>;
    type Error = pack::index::write::Error;

    fn feed(&mut self, input: Self::Input) -> Result<(), Self::Error> {
        let input = input?;
        self.items
            .extend(input.into_iter().map(|(pack_offset, id)| (pack_offset, id, 0)));
        Ok(())
    }

    fn finalize(self) -> Result<Self::Output, Self::Error> {
        Ok(self.items)
    }
}
