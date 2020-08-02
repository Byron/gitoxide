mod from_pack {
    use git_features::progress::Progress;
    use git_odb::pack;
    use std::{io, path::PathBuf};

    pub enum IterationMode {
        AsIs,
        Verify,
        Restore,
    }

    pub enum MemoryMode {
        InMemory,
        InMemoryDecompressed,
        ResolveBases,
        ResolveDeltas,
        ResolveBasesAndDeltas,
    }

    pub struct Context {
        thread_limit: Option<usize>,
        iteration_mode: IterationMode,
        memory_mode: MemoryMode,
        index_kind: pack::index::Kind,
    }

    pub enum ReadOrSeek<R, S>
    where
        R: io::Read,
        S: io::Seek + io::Read,
    {
        Read(R),
        Seek(S),
    }

    pub fn from_pack<P, R, S>(
        pack: ReadOrSeek<R, S>,
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
        unimplemented!("todo pass through to bundle")
    }
}
pub use from_pack::from_pack;
