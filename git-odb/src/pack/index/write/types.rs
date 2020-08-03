use crate::pack;
use git_features::{parallel, progress::Progress};
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

#[derive(Clone)]
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

#[derive(Clone)]
pub(crate) struct Entry {
    pub pack_offset: u64,
    pub entry_len: usize,
    pub kind: ObjectKind,
    pub crc32: u32,
}

pub(crate) struct CacheEntry {
    cache: Cache,
    /// When it reaches zero, the cache can be freed
    child_count: u32,
}

pub(crate) struct TreeEntry {
    pub pack_offset: u64,
    pub entry_len: usize,
    pub kind: ObjectKind,
    pub crc32: u32,
    pub cache: Cache,
}

pub(crate) enum Bytes {
    Owned(Cache),
    Borrowed(Cache),
}

/// Note that every operation in the CacheEntry must be fast, as these happen behind a lock
impl CacheEntry {
    pub fn new(cache: Cache) -> Self {
        CacheEntry { child_count: 0, cache }
    }
    pub fn increment_child_count(&mut self) {
        self.child_count += 1;
    }
    pub fn cache_decr(&mut self) -> Bytes {
        self.child_count -= 1;
        self.cache()
    }

    pub fn is_borrowed(child_count: u32) -> bool {
        child_count != 0
    }

    pub fn cache(&mut self) -> Bytes {
        let cache = std::mem::replace(&mut self.cache, Cache::Unset);
        if Self::is_borrowed(self.child_count) {
            Bytes::Borrowed(cache)
        } else {
            Bytes::Owned(cache)
        }
    }
    pub fn set_decompressed(&mut self, bytes: Vec<u8>) {
        assert_ne!(self.child_count, 0, "Do not return decompressed bytes once nobody is interested in the data anymore, i.e. from `Bytes::Owned(…)`");
        self.cache = Cache::Decompressed(bytes);
    }
}

pub type EntrySlice = std::ops::Range<u64>;

/// The function an entry into all of its bytes written to &mut Vec<u8> which is big enough and returns to true if bytes
/// were written, false otherwise. The latter should never have to happen, but is an escape hatch if something goes very wrong
/// when reading the pack entry.
/// It will only be called after the iterator stopped returning elements.
pub enum Mode<F>
where
    F: Fn(EntrySlice, &mut Vec<u8>) -> Option<()>,
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
    F: Fn(EntrySlice, &mut Vec<u8>) -> Option<()>,
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

impl Mode<fn(EntrySlice, &mut Vec<u8>) -> Option<()>> {
    pub fn in_memory() -> Self {
        Self::InMemory
    }
    pub fn in_memory_decompressed() -> Self {
        Self::InMemoryDecompressed
    }
}

pub(crate) struct Reducer<'a, P> {
    pub(crate) items: Vec<(u64, owned::Id, u32)>,
    progress: &'a parking_lot::Mutex<P>,
    start: std::time::Instant,
}

impl<'a, P> Reducer<'a, P>
where
    P: Progress,
{
    pub fn new(num_objects: u32, progress: &'a parking_lot::Mutex<P>) -> Self {
        progress.lock().init(Some(num_objects), Some("objects"));
        Reducer {
            items: Vec::with_capacity(num_objects as usize),
            progress,
            start: std::time::Instant::now(),
        }
    }
}

impl<'a, P> parallel::Reducer for Reducer<'a, P>
where
    P: Progress,
{
    type Input = Result<Vec<(u64, owned::Id, u32)>, pack::index::write::Error>;
    type Output = Vec<(u64, owned::Id, u32)>;
    type Error = pack::index::write::Error;

    fn feed(&mut self, input: Self::Input) -> Result<(), Self::Error> {
        let input = input?;
        self.progress.lock().inc_by(input.len() as u32);
        self.items.extend(input.into_iter());
        Ok(())
    }

    fn finalize(self) -> Result<Self::Output, Self::Error> {
        self.progress
            .lock()
            .show_throughput(self.start, self.items.len() as u32, "objects");
        Ok(self.items)
    }
}
