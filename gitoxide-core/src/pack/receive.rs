use crate::{OutputFormat, Protocol};
use git_features::progress::Progress;
use std::{io, path::PathBuf};

pub struct Context<W: io::Write> {
    pub thread_limit: Option<usize>,
    pub format: OutputFormat,
    pub out: W,
}

pub fn receive<P, W: io::Write>(
    _protocol: Option<Protocol>,
    _url: &str,
    _directory: Option<PathBuf>,
    _progress: P,
    _ctx: Context<W>,
) -> anyhow::Result<()>
where
    P: Progress,
    <P as Progress>::SubProgress: Send + 'static,
    <<P as Progress>::SubProgress as Progress>::SubProgress: Send,
{
    unimplemented!("pack-receive")
}
