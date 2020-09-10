use crate::{OutputFormat, Protocol};
use git_features::progress::Progress;
use git_protocol::fetch::{Action, Arguments, Ref, Response};
use std::{io, io::BufRead, path::PathBuf};

pub struct Context<W: io::Write> {
    pub thread_limit: Option<usize>,
    pub format: OutputFormat,
    pub out: W,
}

struct CloneDelegate<W: io::Write> {
    ctx: Context<W>,
    directory: Option<PathBuf>,
}

impl<W: io::Write> git_protocol::fetch::Delegate for CloneDelegate<W> {
    fn negotiate(&mut self, refs: &[Ref], arguments: &mut Arguments, _previous: Option<&Response>) -> Action {
        for r in refs {
            arguments.want(r.unpack_common().1.to_borrowed());
        }
        Action::Continue
    }

    fn receive_pack<P>(&mut self, input: impl BufRead, progress: P, refs: &[Ref], previous: &Response) -> io::Result<()>
    where
        P: Progress,
        <P as Progress>::SubProgress: Send + 'static,
        <<P as Progress>::SubProgress as Progress>::SubProgress: Send + 'static,
    {
        let options = git_odb::pack::bundle::write::Options {
            thread_limit: self.ctx.thread_limit,
            index_kind: git_odb::pack::index::Kind::V2,
            iteration_mode: git_odb::pack::data::iter::Mode::Verify,
        };
        let outcome =
            git_odb::pack::bundle::Bundle::write_to_directory(input, None, self.directory.take(), progress, options)
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        writeln!(self.ctx.out, "{:?}", outcome)?;
        Ok(())
    }
}

pub fn receive<P, W: io::Write>(
    protocol: Option<Protocol>,
    url: &str,
    directory: Option<PathBuf>,
    progress: P,
    ctx: Context<W>,
) -> anyhow::Result<()>
where
    P: Progress,
    <P as Progress>::SubProgress: Send + 'static,
    <<P as Progress>::SubProgress as Progress>::SubProgress: Send,
{
    let transport = git_protocol::git_transport::client::connect(url.as_bytes(), protocol.unwrap_or_default().into())?;
    let mut delegate = CloneDelegate { ctx, directory };
    git_protocol::fetch(transport, &mut delegate, git_protocol::credentials::helper, progress)?;
    Ok(())
}
