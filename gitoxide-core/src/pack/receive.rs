use std::{
    io,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
};

use git_repository::{
    hash::ObjectId,
    objs::bstr::{BString, ByteSlice},
    odb::pack,
    protocol,
    protocol::{
        fetch::{Action, Arguments, LsRefsAction, Ref, Response},
        transport,
        transport::client::Capabilities,
    },
};

use crate::{remote::refs::JsonRef, OutputFormat};

pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=3;

pub struct Context<W> {
    pub thread_limit: Option<usize>,
    pub format: OutputFormat,
    pub should_interrupt: Arc<AtomicBool>,
    pub out: W,
}

struct CloneDelegate<W> {
    ctx: Context<W>,
    directory: Option<PathBuf>,
    refs_directory: Option<PathBuf>,
    ref_filter: Option<&'static [&'static str]>,
}
static FILTER: &[&str] = &["HEAD", "refs/tags", "refs/heads"];

impl<W> protocol::fetch::DelegateBlocking for CloneDelegate<W> {
    fn prepare_ls_refs(
        &mut self,
        server: &Capabilities,
        arguments: &mut Vec<BString>,
        _features: &mut Vec<(&str, Option<&str>)>,
    ) -> io::Result<LsRefsAction> {
        if server.contains("ls-refs") {
            arguments.extend(FILTER.iter().map(|r| format!("ref-prefix {}", r).into()));
        }
        Ok(LsRefsAction::Continue)
    }

    fn prepare_fetch(
        &mut self,
        version: transport::Protocol,
        _server: &Capabilities,
        _features: &mut Vec<(&str, Option<&str>)>,
        _refs: &[Ref],
    ) -> io::Result<Action> {
        if version == transport::Protocol::V1 {
            self.ref_filter = Some(FILTER);
        }
        Ok(Action::Continue)
    }

    fn negotiate(
        &mut self,
        refs: &[Ref],
        arguments: &mut Arguments,
        _previous_response: Option<&Response>,
    ) -> io::Result<Action> {
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
        Ok(Action::Cancel)
    }
}

#[cfg(feature = "blocking-client")]
mod blocking_io {
    use std::{io, io::BufRead, path::PathBuf};

    use git_repository::{
        protocol,
        protocol::fetch::{Ref, Response},
        Progress,
    };

    use super::{receive_pack_blocking, CloneDelegate, Context};
    use crate::net;

    impl<W: io::Write> protocol::fetch::Delegate for CloneDelegate<W> {
        fn receive_pack(
            &mut self,
            input: impl BufRead,
            progress: impl Progress,
            refs: &[Ref],
            _previous_response: &Response,
        ) -> io::Result<()> {
            receive_pack_blocking(
                self.directory.take(),
                self.refs_directory.take(),
                &mut self.ctx,
                input,
                progress,
                &refs,
            )
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
        let delegate = CloneDelegate {
            ctx,
            directory,
            refs_directory,
            ref_filter: None,
        };
        protocol::fetch(
            transport,
            delegate,
            protocol::credentials::helper,
            progress,
            protocol::FetchConnection::TerminateOnSuccessfulCompletion,
        )?;
        Ok(())
    }
}

#[cfg(feature = "blocking-client")]
pub use blocking_io::receive;

#[cfg(feature = "async-client")]
mod async_io {
    use std::{io, io::BufRead, path::PathBuf};

    use async_trait::async_trait;
    use futures_io::AsyncBufRead;
    use git_repository::{
        objs::bstr::{BString, ByteSlice},
        odb::pack,
        protocol,
        protocol::fetch::{Ref, Response},
        Progress,
    };

    use super::{print, receive_pack_blocking, write_raw_refs, CloneDelegate, Context};
    use crate::{net, OutputFormat};

    #[async_trait(?Send)]
    impl<W: io::Write + Send + 'static> protocol::fetch::Delegate for CloneDelegate<W> {
        async fn receive_pack(
            &mut self,
            input: impl AsyncBufRead + Unpin + 'async_trait,
            progress: impl Progress,
            refs: &[Ref],
            _previous_response: &Response,
        ) -> io::Result<()> {
            receive_pack_blocking(
                self.directory.take(),
                self.refs_directory.take(),
                &mut self.ctx,
                futures_lite::io::BlockOn::new(input),
                progress,
                &refs,
            )
        }
    }

    pub async fn receive<P: Progress, W: io::Write + Send + 'static>(
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
        blocking::unblock(move || {
            futures_lite::future::block_on(protocol::fetch(
                transport,
                delegate,
                protocol::credentials::helper,
                progress,
                protocol::FetchConnection::TerminateOnSuccessfulCompletion,
            ))
        })
        .await?;
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

fn write_raw_refs(refs: &[Ref], directory: PathBuf) -> std::io::Result<()> {
    let assure_dir_exists = |path: &BString| {
        assert!(!path.starts_with_str("/"), "no ref start with a /, they are relative");
        let path = directory.join(path.to_path().expect("UTF-8 conversion succeeds"));
        std::fs::create_dir_all(path.parent().expect("multi-component path")).map(|_| path)
    };
    for r in refs {
        let (path, content) = match r {
            Ref::Symbolic { path, target, .. } => (assure_dir_exists(path)?, format!("ref: {}", target)),
            Ref::Peeled { path, tag: object, .. } | Ref::Direct { path, object } => {
                (assure_dir_exists(path)?, object.to_string())
            }
        };
        std::fs::write(path, content.as_bytes())?;
    }
    Ok(())
}

fn receive_pack_blocking<W: io::Write>(
    mut directory: Option<PathBuf>,
    mut refs_directory: Option<PathBuf>,
    ctx: &mut Context<W>,
    input: impl io::BufRead,
    progress: impl git_repository::Progress,
    refs: &[Ref],
) -> io::Result<()> {
    let options = pack::bundle::write::Options {
        thread_limit: ctx.thread_limit,
        index_kind: pack::index::Version::V2,
        iteration_mode: pack::data::input::Mode::Verify,
    };
    let outcome = pack::bundle::Bundle::write_to_directory(
        input,
        directory.take(),
        progress,
        &ctx.should_interrupt,
        None,
        options,
    )
    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    if let Some(directory) = refs_directory.take() {
        write_raw_refs(refs, directory)?;
    }

    match ctx.format {
        OutputFormat::Human => drop(print(&mut ctx.out, outcome, refs)),
        #[cfg(feature = "serde1")]
        OutputFormat::Json => {
            serde_json::to_writer_pretty(&mut ctx.out, &JsonOutcome::from_outcome_and_refs(outcome, refs))?
        }
    };
    Ok(())
}
