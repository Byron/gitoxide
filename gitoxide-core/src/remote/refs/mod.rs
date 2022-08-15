use git_repository::{
    protocol,
    protocol::{
        fetch::{Action, Arguments, Ref, Response},
        transport,
    },
};

use crate::OutputFormat;

pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=2;

use std::io;

#[derive(Default)]
struct LsRemotes {
    refs: Vec<Ref>,
}

impl protocol::fetch::DelegateBlocking for LsRemotes {
    fn prepare_fetch(
        &mut self,
        _version: transport::Protocol,
        _server: &transport::client::Capabilities,
        _features: &mut Vec<(&str, Option<&str>)>,
        refs: &[Ref],
    ) -> io::Result<Action> {
        self.refs = refs.into();
        Ok(Action::Cancel)
    }

    fn negotiate(
        &mut self,
        _refs: &[Ref],
        _arguments: &mut Arguments,
        _previous_response: Option<&Response>,
    ) -> io::Result<Action> {
        unreachable!("not to be called due to Action::Close in `prepare_fetch`")
    }
}

#[cfg(feature = "async-client")]
mod async_io;
#[cfg(feature = "async-client")]
pub use self::async_io::list;

#[cfg(feature = "blocking-client")]
mod blocking_io;
#[cfg(feature = "blocking-client")]
pub use blocking_io::list;

pub struct Context<W: io::Write> {
    pub thread_limit: Option<usize>,
    pub format: OutputFormat,
    pub out: W,
}

#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum JsonRef {
    Peeled {
        path: String,
        tag: String,
        object: String,
    },
    Direct {
        path: String,
        object: String,
    },
    Symbolic {
        path: String,
        target: String,
        object: String,
    },
}

impl From<Ref> for JsonRef {
    fn from(value: Ref) -> Self {
        match value {
            Ref::Direct { path, object } => JsonRef::Direct {
                path: path.to_string(),
                object: object.to_string(),
            },
            Ref::Symbolic { path, target, object } => JsonRef::Symbolic {
                path: path.to_string(),
                target: target.to_string(),
                object: object.to_string(),
            },
            Ref::Peeled { path, tag, object } => JsonRef::Peeled {
                path: path.to_string(),
                tag: tag.to_string(),
                object: object.to_string(),
            },
        }
    }
}

pub(crate) fn print(mut out: impl io::Write, refs: &[Ref]) -> io::Result<()> {
    for r in refs {
        match r {
            Ref::Direct { path, object } => writeln!(&mut out, "{} {}", object.to_hex(), path),
            Ref::Peeled { path, object, tag } => {
                writeln!(&mut out, "{} {} tag:{}", object.to_hex(), path, tag)
            }
            Ref::Symbolic { path, target, object } => {
                writeln!(&mut out, "{} {} symref-target:{}", object.to_hex(), path, target)
            }
        }?;
    }
    Ok(())
}
