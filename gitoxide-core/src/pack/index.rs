use git_features::progress::Progress;
use git_odb::pack;
use std::{io, path::PathBuf};

pub enum IterationMode {
    AsIs,
    Verify,
    Restore,
}

impl From<IterationMode> for pack::data::iter::Mode {
    fn from(v: IterationMode) -> Self {
        use pack::data::iter::Mode::*;
        match v {
            IterationMode::AsIs => AsIs,
            IterationMode::Verify => Verify,
            IterationMode::Restore => Restore,
        }
    }
}

pub enum MemoryMode {
    InMemory,
    InMemoryDecompressed,
    ResolveBases,
    ResolveDeltas,
    ResolveBasesAndDeltas,
}

impl From<MemoryMode> for pack::bundle::write::MemoryMode {
    fn from(v: MemoryMode) -> Self {
        use pack::bundle::write::MemoryMode::*;
        match v {
            MemoryMode::InMemory => InMemory,
            MemoryMode::InMemoryDecompressed => InMemoryDecompressed,
            MemoryMode::ResolveBases => ResolveBases,
            MemoryMode::ResolveDeltas => ResolveDeltas,
            MemoryMode::ResolveBasesAndDeltas => ResolveBasesAndDeltas,
        }
    }
}

pub struct Context {
    thread_limit: Option<usize>,
    iteration_mode: IterationMode,
    memory_mode: MemoryMode,
    index_kind: pack::index::Kind,
}

impl From<Context> for pack::bundle::write::Options {
    fn from(
        Context {
            thread_limit,
            iteration_mode,
            memory_mode,
            index_kind,
        }: Context,
    ) -> Self {
        pack::bundle::write::Options {
            thread_limit,
            iteration_mode: iteration_mode.into(),
            memory_mode: memory_mode.into(),
            index_kind,
        }
    }
}

pub enum ReadOrSeek<R, S>
where
    R: io::Read,
    S: io::Seek + io::Read,
{
    Read(R),
    Seek(S),
}

impl<R, S> ReadOrSeek<R, S>
where
    R: io::Read,
    S: io::Seek + io::Read,
{
    pub fn inner_stream_len(s: &mut S) -> io::Result<u64> {
        use io::SeekFrom;
        let old_pos = s.seek(SeekFrom::Current(0))?;
        let len = s.seek(SeekFrom::End(0))?;
        if old_pos != len {
            s.seek(SeekFrom::Start(old_pos))?;
        }
        Ok(len)
    }

    pub fn stream_len(&mut self) -> Option<io::Result<u64>> {
        match self {
            ReadOrSeek::Read(_) => None,
            ReadOrSeek::Seek(s) => Some(Self::inner_stream_len(s)),
        }
    }
}
impl<R, S> io::Read for ReadOrSeek<R, S>
where
    R: io::Read,
    S: io::Seek + io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            ReadOrSeek::Read(v) => v.read(buf),
            ReadOrSeek::Seek(v) => v.read(buf),
        }
    }
}

pub fn from_pack<P, R, S>(
    mut pack: ReadOrSeek<R, S>,
    directory: Option<PathBuf>,
    progress: P,
    context: Context,
) -> anyhow::Result<()>
where
    R: io::Read,
    S: io::Seek + io::Read,
    P: Progress,
    <<P as Progress>::SubProgress as Progress>::SubProgress: Send,
{
    use anyhow::Context;
    let pack_len = pack.stream_len().transpose()?;
    pack::Bundle::write_to_directory(pack, pack_len, directory, progress, context.into())
        .with_context(|| "Failed to write pack and index")
        .map(|_| ())
}
