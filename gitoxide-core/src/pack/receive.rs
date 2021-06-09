use crate::{remote::refs::JsonRef, OutputFormat};
use git_repository::{
    hash::ObjectId,
    object::bstr::{BString, ByteSlice},
    odb::pack,
    protocol,
    protocol::{
        fetch::{Action, Arguments, Ref, Response},
        transport,
        transport::client::Capabilities,
    },
};
use std::{io, path::PathBuf};

pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=3;

pub struct Context<W: io::Write> {
    pub thread_limit: Option<usize>,
    pub format: OutputFormat,
    pub out: W,
}

struct CloneDelegate<W: io::Write> {
    ctx: Context<W>,
    directory: Option<PathBuf>,
    refs_directory: Option<PathBuf>,
    ref_filter: Option<&'static [&'static str]>,
}
static FILTER: &[&str] = &["HEAD", "refs/tags", "refs/heads"];

impl<W: io::Write> protocol::fetch::DelegateWithoutIO for CloneDelegate<W> {
    fn prepare_ls_refs(
        &mut self,
        server: &Capabilities,
        arguments: &mut Vec<BString>,
        _features: &mut Vec<(&str, Option<&str>)>,
    ) {
        if server.contains("ls-refs") {
            arguments.extend(FILTER.iter().map(|r| format!("ref-prefix {}", r).into()));
        }
    }

    fn prepare_fetch(
        &mut self,
        version: transport::Protocol,
        _server: &Capabilities,
        _features: &mut Vec<(&str, Option<&str>)>,
        _refs: &[Ref],
    ) -> Action {
        if version == transport::Protocol::V1 {
            self.ref_filter = Some(&FILTER);
        }
        Action::Continue
    }

    fn negotiate(&mut self, refs: &[Ref], arguments: &mut Arguments, _previous: Option<&Response>) -> Action {
        for r in refs {
            let (path, id) = r.unpack();
            match self.ref_filter {
                Some(ref_prefixes) => {
                    if ref_prefixes.iter().any(|prefix| path.starts_with_str(prefix)) {
                        arguments.want(id);
                    }
                }
                None => arguments.want(id),
            }
        }
        Action::Close
    }
}

#[cfg(feature = "blocking-client")]
mod blocking_io {
    #[cfg(feature = "serde1")]
    use super::JsonOutcome;
    use super::{CloneDelegate, Context};
    use crate::{net, pack::receive::print, OutputFormat};
    use git_repository::{
        object::bstr::{BString, ByteSlice},
        odb::pack,
        protocol,
        protocol::fetch::{Ref, Response},
        Progress,
    };
    use std::{io, io::BufRead, path::PathBuf};

    impl<W: io::Write> protocol::fetch::Delegate for CloneDelegate<W> {
        fn receive_pack(
            &mut self,
            input: impl BufRead,
            progress: impl Progress,
            refs: &[Ref],
            _previous: &Response,
        ) -> io::Result<()> {
            let options = pack::bundle::write::Options {
                thread_limit: self.ctx.thread_limit,
                index_kind: pack::index::Version::V2,
                iteration_mode: pack::data::input::Mode::Verify,
            };
            let outcome = pack::bundle::Bundle::write_to_directory(input, self.directory.take(), progress, options)
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

            if let Some(directory) = self.refs_directory.take() {
                let assure_dir = |path: &BString| {
                    assert!(!path.starts_with_str("/"), "no ref start with a /, they are relative");
                    let path = directory.join(path.to_path_lossy());
                    std::fs::create_dir_all(path.parent().expect("multi-component path")).map(|_| path)
                };
                for r in refs {
                    let (path, content) = match r {
                        Ref::Symbolic { path, target, .. } => (assure_dir(path)?, format!("ref: {}", target)),
                        Ref::Peeled { path, tag: object, .. } | Ref::Direct { path, object } => {
                            (assure_dir(path)?, object.to_string())
                        }
                    };
                    std::fs::write(path, content.as_bytes())?;
                }
            }

            match self.ctx.format {
                OutputFormat::Human => drop(print(&mut self.ctx.out, outcome, refs)),
                #[cfg(feature = "serde1")]
                OutputFormat::Json => {
                    serde_json::to_writer_pretty(&mut self.ctx.out, &JsonOutcome::from_outcome_and_refs(outcome, refs))?
                }
            };
            Ok(())
        }
    }

    pub fn receive<P: Progress, W: io::Write>(
        protocol: Option<net::Protocol>,
        url: &str,
        directory: Option<PathBuf>,
        refs_directory: Option<PathBuf>,
        progress: P,
        ctx: Context<W>,
    ) -> anyhow::Result<()> {
        let transport = net::connect(url.as_bytes(), protocol.unwrap_or_default().into())?;
        let mut delegate = CloneDelegate {
            ctx,
            directory,
            refs_directory,
            ref_filter: None,
        };
        protocol::fetch(transport, &mut delegate, protocol::credentials::helper, progress)?;
        Ok(())
    }
}
#[cfg(feature = "blocking-client")]
pub use blocking_io::receive;

#[cfg(feature = "async-client")]
mod async_io {
    #[cfg(feature = "serde1")]
    use super::JsonOutcome;
    use super::{CloneDelegate, Context};
    use crate::{net, pack::receive::print, OutputFormat};
    use async_trait::async_trait;
    use futures_io::AsyncBufRead;
    use git_repository::{
        object::bstr::{BString, ByteSlice},
        odb::pack,
        protocol,
        protocol::fetch::{Ref, Response},
        Progress,
    };
    use std::{io, io::BufRead, path::PathBuf};

    #[async_trait(?Send)]
    impl<W: io::Write> protocol::fetch::Delegate for CloneDelegate<W> {
        async fn receive_pack(
            &mut self,
            input: impl AsyncBufRead + Unpin + 'async_trait,
            progress: impl Progress,
            refs: &[Ref],
            _previous: &Response,
        ) -> io::Result<()> {
            let options = pack::bundle::write::Options {
                thread_limit: self.ctx.thread_limit,
                index_kind: pack::index::Version::V2,
                iteration_mode: pack::data::input::Mode::Verify,
            };
            // TODO: unblock
            let outcome = pack::bundle::Bundle::write_to_directory(
                futures_lite::io::BlockOn::new(input),
                self.directory.take(),
                progress,
                options,
            )
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

            if let Some(directory) = self.refs_directory.take() {
                let refs = refs.to_owned();
                blocking::unblock(move || -> io::Result<()> {
                    let assure_dir_exists = |path: &BString| {
                        assert!(!path.starts_with_str("/"), "no ref start with a /, they are relative");
                        let path = directory.join(path.to_path_lossy());
                        std::fs::create_dir_all(path.parent().expect("multi-component path")).map(|_| path)
                    };
                    for r in refs {
                        let (path, content) = match r {
                            Ref::Symbolic { path, target, .. } => {
                                (assure_dir_exists(&path)?, format!("ref: {}", target))
                            }
                            Ref::Peeled { path, tag: object, .. } | Ref::Direct { path, object } => {
                                (assure_dir_exists(&path)?, object.to_string())
                            }
                        };
                        std::fs::write(path, content.as_bytes())?;
                    }
                    Ok(())
                })
                .await?;
            }

            // TODO: unblock
            match self.ctx.format {
                OutputFormat::Human => drop(print(&mut self.ctx.out, outcome, refs)),
                #[cfg(feature = "serde1")]
                OutputFormat::Json => {
                    serde_json::to_writer_pretty(&mut self.ctx.out, &JsonOutcome::from_outcome_and_refs(outcome, refs))?
                }
            };
            Ok(())
        }
    }

    pub async fn receive<P: Progress, W: io::Write>(
        protocol: Option<net::Protocol>,
        url: &str,
        directory: Option<PathBuf>,
        refs_directory: Option<PathBuf>,
        progress: P,
        ctx: Context<W>,
    ) -> anyhow::Result<()> {
        let transport = net::connect(url.as_bytes(), protocol.unwrap_or_default().into()).await?;
        let mut delegate = CloneDelegate {
            ctx,
            directory,
            refs_directory,
            ref_filter: None,
        };
        protocol::fetch(transport, &mut delegate, protocol::credentials::helper, progress).await?;
        Ok(())
    }
}
#[cfg(feature = "async-client")]
pub use self::async_io::receive;

#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct JsonBundleWriteOutcome {
    pub index_kind: pack::index::Version,
    pub index_hash: String,

    pub data_hash: String,
    pub num_objects: u32,
}

impl From<pack::index::write::Outcome> for JsonBundleWriteOutcome {
    fn from(v: pack::index::write::Outcome) -> Self {
        JsonBundleWriteOutcome {
            index_kind: v.index_kind,
            num_objects: v.num_objects,
            data_hash: v.data_hash.to_string(),
            index_hash: v.index_hash.to_string(),
        }
    }
}

#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct JsonOutcome {
    pub index: JsonBundleWriteOutcome,
    pub pack_kind: pack::data::Version,

    pub index_path: Option<PathBuf>,
    pub data_path: Option<PathBuf>,

    pub refs: Vec<JsonRef>,
}

impl JsonOutcome {
    pub fn from_outcome_and_refs(v: pack::bundle::write::Outcome, refs: &[Ref]) -> Self {
        JsonOutcome {
            index: v.index.into(),
            pack_kind: v.pack_kind,
            index_path: v.index_path,
            data_path: v.data_path,
            refs: refs.iter().cloned().map(Into::into).collect(),
        }
    }
}

fn print_hash_and_path(out: &mut impl io::Write, name: &str, id: ObjectId, path: Option<PathBuf>) -> io::Result<()> {
    match path {
        Some(path) => writeln!(out, "{}: {} ({})", name, id, path.display()),
        None => writeln!(out, "{}: {}", name, id),
    }
}

fn print(out: &mut impl io::Write, res: pack::bundle::write::Outcome, refs: &[Ref]) -> io::Result<()> {
    print_hash_and_path(out, "index", res.index.index_hash, res.index_path)?;
    print_hash_and_path(out, "pack", res.index.data_hash, res.data_path)?;
    writeln!(out)?;
    crate::remote::refs::print(out, refs)?;
    Ok(())
}
