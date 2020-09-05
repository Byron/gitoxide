pub mod refs {
    use crate::{OutputFormat, Protocol};
    use git_features::progress::Progress;
    use git_protocol::{
        fetch::{Action, Ref},
        git_transport,
    };
    pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=2;
    use std::io;

    #[derive(Default)]
    struct LsRemotes {
        refs: Vec<Ref>,
    }

    impl git_protocol::fetch::Delegate for LsRemotes {
        fn prepare_fetch(
            &mut self,
            _version: git_transport::Protocol,
            _server: &git_transport::client::Capabilities,
            _features: &mut Vec<(&str, Option<&str>)>,
            refs: &[Ref],
        ) -> Action {
            self.refs = refs.into();
            Action::Close
        }
    }

    pub struct Context<W: io::Write> {
        pub thread_limit: Option<usize>,
        pub format: OutputFormat,
        pub out: W,
    }

    pub fn list<P, W: io::Write>(
        protocol: Option<Protocol>,
        url: &str,
        progress: P,
        ctx: Context<W>,
    ) -> anyhow::Result<()>
    where
        P: Progress,
        <P as Progress>::SubProgress: Send + 'static,
        <<P as Progress>::SubProgress as Progress>::SubProgress: Send,
    {
        let transport = git_transport::client::connect(url.as_bytes(), protocol.unwrap_or_default().into())?;
        let mut delegate = LsRemotes::default();
        git_protocol::fetch(transport, &mut delegate, git_protocol::credentials::helper, progress)?;

        match ctx.format {
            OutputFormat::Human => drop(print(ctx.out, &delegate.refs)),
            #[cfg(feature = "serde1")]
            OutputFormat::Json => serde_json::to_writer_pretty(ctx.out, &delegate.refs)?,
        };
        Ok(())
    }

    fn print(mut out: impl io::Write, refs: &[Ref]) -> io::Result<()> {
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
                    target
                ),
                Ref::SymbolicForLookup { .. } => unreachable!("Bug: these should be resolved already"),
            }?;
        }
        Ok(())
    }
}
