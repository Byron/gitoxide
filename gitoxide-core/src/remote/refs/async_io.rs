use std::io;

use async_trait::async_trait;
use futures_io::AsyncBufRead;
use git_repository::{
    protocol,
    protocol::fetch::{Ref, Response},
    Progress,
};

use super::{Context, LsRemotes};
use crate::{net, remote::refs::print, OutputFormat};

#[async_trait(?Send)]
impl protocol::fetch::Delegate for LsRemotes {
    async fn receive_pack(
        &mut self,
        input: impl AsyncBufRead + Unpin + 'async_trait,
        progress: impl Progress,
        refs: &[Ref],
        previous_response: &Response,
    ) -> io::Result<()> {
        unreachable!("not called for ls-refs")
    }
}

pub async fn list(
    protocol: Option<net::Protocol>,
    url: &str,
    progress: impl Progress,
    ctx: Context<impl io::Write + Send + 'static>,
) -> anyhow::Result<()> {
    let url = url.to_owned();
    let transport = net::connect(url.as_str().into(), protocol.unwrap_or_default().into()).await?;
    blocking::unblock(
        // `blocking` really needs a way to unblock futures, which is what it does internally anyway.
        // Both fetch() needs unblocking as it executes blocking code within the future, and the other
        // block does blocking IO because it's primarily a blocking codebase.
        move || {
            futures_lite::future::block_on(async move {
                let mut delegate = LsRemotes::default();
                protocol::fetch(
                    transport,
                    &mut delegate,
                    protocol::credentials::helper,
                    progress,
                    protocol::FetchConnection::TerminateOnSuccessfulCompletion,
                )
                .await?;

                match ctx.format {
                    OutputFormat::Human => drop(print(ctx.out, &delegate.refs)),
                    #[cfg(feature = "serde1")]
                    OutputFormat::Json => serde_json::to_writer_pretty(
                        ctx.out,
                        &delegate.refs.into_iter().map(super::JsonRef::from).collect::<Vec<_>>(),
                    )?,
                }
                Ok(())
            })
        },
    )
    .await
}
