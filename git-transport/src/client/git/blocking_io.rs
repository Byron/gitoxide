use std::{any::Any, error::Error, io::Write};

use bstr::BString;
use git_packetline::PacketLineRef;

use crate::{
    client::{self, capabilities, git, Capabilities, SetServiceResponse},
    Protocol, Service,
};

impl<R, W> client::TransportWithoutIO for git::Connection<R, W>
where
    R: std::io::Read,
    W: std::io::Write,
{
    fn request(
        &mut self,
        write_mode: client::WriteMode,
        on_into_read: client::MessageKind,
    ) -> Result<client::RequestWriter<'_>, client::Error> {
        Ok(client::RequestWriter::new_from_bufread(
            &mut self.writer,
            Box::new(self.line_provider.as_read_without_sidebands()),
            write_mode,
            on_into_read,
        ))
    }

    fn to_url(&self) -> String {
        self.custom_url.as_ref().map_or_else(
            || {
                let mut possibly_lossy_url = self.path.to_string();
                possibly_lossy_url.insert_str(0, "file://");
                possibly_lossy_url
            },
            |url| url.clone(),
        )
    }

    /// We implement this in a paranoid and safe way, not allowing downgrade to V1 which
    /// could send large amounts of refs in case we didn't want to support V1.
    fn supported_protocol_versions(&self) -> &[Protocol] {
        if self.desired_version == Protocol::V1 {
            // allow any version
            &[]
        } else {
            // only allow the specified one
            &self.supported_versions
        }
    }

    fn connection_persists_across_multiple_requests(&self) -> bool {
        true
    }

    fn configure(&mut self, _config: &dyn Any) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        Ok(())
    }
}

impl<R, W> client::Transport for git::Connection<R, W>
where
    R: std::io::Read,
    W: std::io::Write,
{
    fn handshake<'a>(
        &mut self,
        service: Service,
        extra_parameters: &'a [(&'a str, Option<&'a str>)],
    ) -> Result<SetServiceResponse<'_>, client::Error> {
        if self.mode == git::ConnectMode::Daemon {
            let mut line_writer = git_packetline::Writer::new(&mut self.writer).binary_mode();
            line_writer.write_all(&git::message::connect(
                service,
                self.desired_version,
                &self.path,
                self.virtual_host.as_ref(),
                extra_parameters,
            ))?;
            line_writer.flush()?;
        }

        let capabilities::recv::Outcome {
            capabilities,
            refs,
            protocol: actual_protocol,
        } = Capabilities::from_lines_with_version_detection(&mut self.line_provider, service)?;
        Ok(SetServiceResponse {
            actual_protocol,
            capabilities,
            refs,
        })
    }
}

impl<R, W> git::Connection<R, W>
where
    R: std::io::Read,
    W: std::io::Write,
{
    /// Create a connection from the given `read` and `write`, asking for `desired_version` as preferred protocol
    /// and the transfer of the repository at `repository_path`.
    ///
    /// `virtual_host` along with a port to which to connect to, while `mode` determines the kind of endpoint to connect to.
    pub fn new(
        read: R,
        write: W,
        desired_version: Protocol,
        repository_path: impl Into<BString>,
        virtual_host: Option<(impl Into<String>, Option<u16>)>,
        mode: git::ConnectMode,
    ) -> Self {
        git::Connection {
            writer: write,
            line_provider: git_packetline::StreamingPeekableIter::new(read, &[PacketLineRef::Flush]),
            path: repository_path.into(),
            virtual_host: virtual_host.map(|(h, p)| (h.into(), p)),
            desired_version,
            custom_url: None,
            supported_versions: [desired_version],
            mode,
        }
    }
    pub(crate) fn new_for_spawned_process(
        reader: R,
        writer: W,
        desired_version: Protocol,
        repository_path: impl Into<BString>,
    ) -> Self {
        Self::new(
            reader,
            writer,
            desired_version,
            repository_path,
            None::<(&str, _)>,
            git::ConnectMode::Process,
        )
    }
}

///
pub mod connect {
    use std::net::{TcpStream, ToSocketAddrs};

    use bstr::BString;

    use crate::client::git;
    /// The error used in [`connect()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("An IO error occurred when connecting to the server")]
        Io(#[from] std::io::Error),
        #[error("Could not parse {host:?} as virtual host with format <host>[:port]")]
        VirtualHostInvalid { host: String },
    }

    fn parse_host(input: String) -> Result<(String, Option<u16>), Error> {
        let mut tokens = input.splitn(2, ':');
        Ok(match (tokens.next(), tokens.next()) {
            (Some(host), None) => (host.to_owned(), None),
            (Some(host), Some(port)) => (
                host.to_owned(),
                Some(port.parse().map_err(|_| Error::VirtualHostInvalid { host: input })?),
            ),
            _ => unreachable!("we expect at least one token, the original string"),
        })
    }

    /// Connect to a git daemon running on `host` and optionally `port` and a repository at `path`.
    ///
    /// Use `desired_version` to specify a preferred protocol to use, knowing that it can be downgraded by a server not supporting it.
    pub fn connect(
        host: &str,
        path: BString,
        desired_version: crate::Protocol,
        port: Option<u16>,
    ) -> Result<git::Connection<TcpStream, TcpStream>, Error> {
        let read = TcpStream::connect_timeout(
            &(host, port.unwrap_or(9418))
                .to_socket_addrs()?
                .next()
                .expect("after successful resolution there is an IP address"),
            std::time::Duration::from_secs(5),
        )?;
        let write = read.try_clone()?;
        let vhost = std::env::var("GIT_OVERRIDE_VIRTUAL_HOST")
            .ok()
            .map(parse_host)
            .transpose()?
            .unwrap_or_else(|| (host.to_owned(), port));
        Ok(git::Connection::new(
            read,
            write,
            desired_version,
            path,
            Some(vhost),
            git::ConnectMode::Daemon,
        ))
    }
}

pub use connect::connect;
