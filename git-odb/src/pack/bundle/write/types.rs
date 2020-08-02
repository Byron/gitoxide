use crate::pack;
use std::io;
use tempfile::NamedTempFile;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Outcome {
    pub index: pack::index::write::Outcome,
    pub pack_kind: pack::data::Kind,
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

    pub(crate) fn is_in_memory(&self) -> bool {
        use MemoryMode::*;
        match self {
            InMemory | InMemoryDecompressed => true,
            ResolveBases | ResolveDeltas | ResolveBasesAndDeltas => false,
        }
    }
}

pub(crate) struct PassThrough<R> {
    pub reader: R,
    pub writer: Option<NamedTempFile>,
}

impl<R> io::Read for PassThrough<R>
where
    R: io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.reader.read(buf)?;
        if let Some(writer) = self.writer.as_mut() {
            use io::Write;
            writer.write(&buf[..bytes_read])?;
        }
        Ok(bytes_read)
    }
}
