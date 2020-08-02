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

pub(crate) struct PassThrough<R, W> {
    pub inner_read: R,
    pub inner_write: W,
}

impl<R, W> io::Read for PassThrough<R, W>
where
    R: io::Read,
    W: io::Write,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.inner_read.read(buf)?;
        self.inner_write.write(&buf[..bytes_read])?;
        Ok(bytes_read)
    }
}
