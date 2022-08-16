use std::io;

use git_repository::{
    protocol,
    protocol::fetch::{Ref, Response},
    Progress,
};

#[cfg(feature = "serde1")]
use super::JsonRef;
use super::{print, Context, LsRemotes};
use crate::{net, OutputFormat};

impl protocol::fetch::Delegate for LsRemotes {
    fn receive_pack(
        &mut self,
        _input: impl io::BufRead,
        _progress: impl Progress,
        _refs: &[Ref],
        _previous_response: &Response,
    ) -> io::Result<()> {
        unreachable!("not called for ls-refs")
    }
}

pub fn list(
    protocol: Option<net::Protocol>,
    url: &str,
    progress: impl Progress,
    ctx: Context<impl io::Write>,
) -> anyhow::Result<()> {
    let transport = net::connect(url.into(), protocol.unwrap_or_default().into())?;
    let mut delegate = LsRemotes::default();
    protocol::fetch(
        transport,
        &mut delegate,
        protocol::credentials::helper,
        progress,
        protocol::FetchConnection::TerminateOnSuccessfulCompletion,
    )?;

    match ctx.format {
        OutputFormat::Human => drop(print(ctx.out, &delegate.refs)),
        #[cfg(feature = "serde1")]
        OutputFormat::Json => serde_json::to_writer_pretty(
            ctx.out,
            &delegate.refs.into_iter().map(JsonRef::from).collect::<Vec<_>>(),
        )?,
    };
    Ok(())
}
