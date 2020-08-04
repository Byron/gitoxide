use crate::pack;
use git_features::{parallel, progress::Progress};
use git_object::owned;
use std::io;

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
}

impl Default for Cache {
    fn default() -> Self {
        Cache::Unset
    }
}

#[derive(Clone)]
pub(crate) enum ObjectKind {
    Base(git_object::Kind),
    OfsDelta,
}

impl ObjectKind {
    pub fn to_kind(&self) -> Option<git_object::Kind> {
        match self {
            ObjectKind::Base(kind) => Some(*kind),
            ObjectKind::OfsDelta => None,
        }
    }
}

pub(crate) struct TreeEntry {
    pub id: owned::Id,
    pub pack_offset: u64,
    pub entry_len: usize,
    pub kind: ObjectKind,
    pub crc32: u32,
    pub cache: Cache,
}

impl Default for TreeEntry {
    fn default() -> Self {
        TreeEntry {
            id: owned::Id::null(),
            pack_offset: 0,
            entry_len: 0,
            kind: ObjectKind::OfsDelta,
            crc32: 0,
            cache: Cache::Unset,
        }
    }
}

pub type EntrySlice = std::ops::Range<u64>;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Mode {
    /// Base + deltas in memory, decompressed
    InMemory,
    /// Bases in memory, decompressed
    ResolveDeltas,
    ResolveBasesAndDeltas,
}

impl Mode {
    pub(crate) fn base_cache(&self, decompressed: Vec<u8>) -> Cache {
        match self {
            Mode::InMemory | Mode::ResolveDeltas => Cache::Decompressed(decompressed),
            Mode::ResolveBasesAndDeltas => Cache::Unset,
        }
    }
    pub(crate) fn delta_cache(&self, decompressed: Vec<u8>) -> Cache {
        match self {
            Mode::InMemory => Cache::Decompressed(decompressed),
            Mode::ResolveDeltas | Mode::ResolveBasesAndDeltas => Cache::Unset,
        }
    }
    pub(crate) fn is_in_memory(&self) -> bool {
        match self {
            Mode::InMemory => true,
            Mode::ResolveDeltas | Mode::ResolveBasesAndDeltas => false,
        }
    }
}

pub type ResolverFn = fn(EntrySlice, &mut Vec<u8>) -> Option<()>;

impl Mode {
    pub fn noop_resolver() -> io::Result<ResolverFn> {
        fn noop(_: EntrySlice, _: &mut Vec<u8>) -> Option<()> {
            None
        };
        Ok(noop)
    }
}

pub(crate) struct Reducer<'a, P> {
    item_count: usize,
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
            item_count: 0,
            progress,
            start: std::time::Instant::now(),
        }
    }
}

impl<'a, P> parallel::Reducer for Reducer<'a, P>
where
    P: Progress,
{
    type Input = Result<usize, pack::index::write::Error>;
    type Output = ();
    type Error = pack::index::write::Error;

    fn feed(&mut self, input: Self::Input) -> Result<(), Self::Error> {
        let input = input?;
        self.item_count += input;
        self.progress.lock().set(self.item_count as u32);
        Ok(())
    }

    fn finalize(self) -> Result<Self::Output, Self::Error> {
        self.progress
            .lock()
            .show_throughput(self.start, self.item_count as u32, "objects");
        Ok(())
    }
}
