use crate::OutputFormat;
use git_features::progress::Progress;
use std::{io, path::PathBuf, str::FromStr};

#[derive(PartialEq, Debug)]
pub enum Protocol {
    V1,
    V2,
}

impl FromStr for Protocol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => Protocol::V1,
            "2" => Protocol::V2,
            _ => return Err(format!("Unsupported protocol version '{}', choose '1' or '2'", s)),
        })
    }
}

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
