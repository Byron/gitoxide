pub mod refs {
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
    mod async_io {
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
            let transport = net::connect(url.as_bytes(), protocol.unwrap_or_default().into()).await?;
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
                                &delegate.refs.into_iter().map(JsonRef::from).collect::<Vec<_>>(),
                            )?,
                        }
                        Ok(())
                    })
                },
            )
            .await
        }
    }
    #[cfg(feature = "async-client")]
    pub use self::async_io::list;

    #[cfg(feature = "blocking-client")]
    mod blocking_io {
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
            let transport = net::connect(url.as_bytes(), protocol.unwrap_or_default().into())?;
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
    }
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
            target: Option<String>,
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
                    target: target.map(|t| t.to_string()),
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
                Ref::Direct { path, object } => writeln!(&mut out, "{} {}", object.to_sha1_hex_string(), path),
                Ref::Peeled { path, object, tag } => {
                    writeln!(&mut out, "{} {} tag:{}", object.to_sha1_hex_string(), path, tag)
                }
                Ref::Symbolic { path, target, object } => writeln!(
                    &mut out,
                    "{} {} symref-target:{}",
                    object.to_sha1_hex_string(),
                    path,
                    target.clone().unwrap_or_else(|| "(null)".into())
                ),
            }?;
        }
        Ok(())
    }
}
