use crate::pack;
use git_object::owned;
use std::io;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Outcome {
    index: pack::index::write::Outcome,
    pack_kind: pack::data::Kind,
    pack_hash: owned::Id,
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum MemoryMode {
    /// Base + deltas in memory compressed
    InMemory,
    InMemoryDecompressed,
    /// Deltas in memory compressed
    ResolveBases,
    /// Bases in memory compressed
    ResolveDeltas,
    ResolveBasesAndDeltas,
}

impl MemoryMode {
    pub(crate) fn into_write_mode<F>(self, f: F) -> pack::index::write::Mode<F>
    where
        F: Fn(pack::index::write::EntrySlice, &mut Vec<u8>) -> Option<()>,
    {
        use MemoryMode::*;
        match self {
            InMemory => pack::index::write::Mode::InMemory,
            InMemoryDecompressed => pack::index::write::Mode::InMemoryDecompressed,
            ResolveBases => pack::index::write::Mode::ResolveBases(f),
            ResolveDeltas => pack::index::write::Mode::ResolveDeltas(f),
            ResolveBasesAndDeltas => pack::index::write::Mode::ResolveBasesAndDeltas(f),
        }
    }
}

pub(crate) struct PassThrough<R, W> {
    pub reader: R,
    pub writer: W,
}

impl<R, W> io::Read for PassThrough<R, W>
where
    R: io::Read,
    W: io::Write,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.reader.read(buf)?;
        self.writer.write(&buf[..bytes_read])?;
        Ok(bytes_read)
    }
}
