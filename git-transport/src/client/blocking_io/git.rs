use std::{io, io::Write};

use bstr::BString;

use git_packetline::PacketLine;

use crate::{
    client::{self, capabilities, Capabilities, SetServiceResponse},
    Protocol, Service,
};

pub(crate) mod message {
    use bstr::{BString, ByteVec};

    use crate::{Protocol, Service};

    pub fn connect(
        service: Service,
        version: Protocol,
        path: &[u8],
        virtual_host: Option<&(String, Option<u16>)>,
    ) -> BString {
        let mut out = bstr::BString::from(service.as_str());
        out.push(b' ');
        let path = git_url::expand_path::for_shell(path.into());
        out.extend_from_slice(&path);
        out.push(0);
        if let Some((host, port)) = virtual_host {
            out.push_str("host=");
            out.extend_from_slice(host.as_bytes());
            if let Some(port) = port {
                out.push_byte(b':');
                out.push_str(&format!("{}", port));
            }
            out.push(0);
        }
        // We only send the version when needed, as otherwise a V2 server who is asked for V1 will respond with 'version 1'
        // as extra lines in the reply, which we don't want to handle. Especially since an old server will not respond with that
        // line (is what I assume, at least), so it's an optional part in the response to understand and handle. There is no value
        // in that, so let's help V2 servers to respond in a way that assumes V1.
        if version != Protocol::V1 {
            out.push(0);
            out.push_str(format!("version={}", version as usize));
            out.push(0);
        }
        out
    }
    #[cfg(test)]
    mod tests {
        use crate::{client::blocking_io::git, Protocol, Service};

        #[test]
        fn version_1_without_host_and_version() {
            assert_eq!(
                git::message::connect(Service::UploadPack, Protocol::V1, b"hello/world", None),
                "git-upload-pack hello/world\0"
            )
        }
        #[test]
        fn version_2_without_host_and_version() {
            assert_eq!(
                git::message::connect(Service::UploadPack, Protocol::V2, b"hello\\world", None),
                "git-upload-pack hello\\world\0\0version=2\0"
            )
        }
        #[test]
        fn with_host_without_port() {
            assert_eq!(
                git::message::connect(
                    Service::UploadPack,
                    Protocol::V1,
                    b"hello\\world",
                    Some(&("host".into(), None))
                ),
                "git-upload-pack hello\\world\0host=host\0"
            )
        }
        #[test]
        fn with_host_with_port() {
            assert_eq!(
                git::message::connect(
                    Service::UploadPack,
                    Protocol::V1,
                    b"hello\\world",
                    Some(&("host".into(), Some(404)))
                ),
                "git-upload-pack hello\\world\0host=host:404\0"
            )
        }
    }
}

/// The way to connect to a process speaking the `git` protocol.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ConnectMode {
    /// A git daemon.
    Daemon,
    /// A spawned `git` process to upload a pack to the client.
    Process,
}

/// A TCP connection to either a `git` daemon or a spawned `git` process.
///
/// When connecting to a daemon, additional context information is sent with the first line of the handshake. Otherwise that
/// context is passed using command line arguments to a [spawned `git` process][crate::client::file::SpawnProcessOnDemand].
pub struct Connection<R, W> {
    writer: W,
    line_provider: git_packetline::StreamingPeekableIter<R>,
    path: BString,
    virtual_host: Option<(String, Option<u16>)>,
    desired_version: Protocol,
    mode: ConnectMode,
}

impl<R, W> client::Transport for Connection<R, W>
where
    R: io::Read,
    W: io::Write,
{
    fn handshake(&mut self, service: Service) -> Result<SetServiceResponse<'_>, client::Error> {
        if self.mode == ConnectMode::Daemon {
            let mut line_writer = git_packetline::Writer::new(&mut self.writer).binary_mode();
            line_writer.write_all(&message::connect(
                service,
                self.desired_version,
                &self.path,
                self.virtual_host.as_ref(),
            ))?;
            line_writer.flush()?;
        }

        let capabilities::recv::Outcome {
            capabilities,
            refs,
            protocol: actual_protocol,
        } = Capabilities::from_lines_with_version_detection(&mut self.line_provider)?;
        Ok(SetServiceResponse {
            actual_protocol,
            capabilities,
            refs,
        })
    }

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

    fn close(&mut self) -> Result<(), client::Error> {
        git_packetline::encode::flush_to_write(&mut self.writer)?;
        self.writer.flush()?;
        Ok(())
    }

    fn to_url(&self) -> String {
        git_url::Url {
            scheme: git_url::Scheme::File,
            user: None,
            host: None,
            port: None,
            path: self.path.clone(),
        }
        .to_string()
    }

    fn desired_protocol_version(&self) -> Protocol {
        self.desired_version
    }

    fn is_stateful(&self) -> bool {
        true
    }
}

impl<R, W> Connection<R, W>
where
    R: io::Read,
    W: io::Write,
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
        mode: ConnectMode,
    ) -> Self {
        Connection {
            writer: write,
            line_provider: git_packetline::StreamingPeekableIter::new(read, &[PacketLine::Flush]),
            path: repository_path.into(),
            virtual_host: virtual_host.map(|(h, p)| (h.into(), p)),
            desired_version,
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
            ConnectMode::Process,
        )
    }
}

///
pub mod connect {
    use std::{
        io,
        net::{TcpStream, ToSocketAddrs},
    };

    use crate::client::git::{ConnectMode, Connection};
    use bstr::BString;
    use quick_error::quick_error;
    quick_error! {
        /// The error used in [`connect()`].
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            Io(err: io::Error){
                display("An IO error occurred when connecting to the server")
                from()
                source(err)
            }
            VirtualHostInvalid(host: String) {
                display("Could not parse '{}' as virtual host with format <host>[:port]", host)
            }
        }
    }

    fn parse_host(input: String) -> Result<(String, Option<u16>), Error> {
        let mut tokens = input.splitn(2, ':');
        Ok(match (tokens.next(), tokens.next()) {
            (Some(host), None) => (host.to_owned(), None),
            (Some(host), Some(port)) => (
                host.to_owned(),
                Some(port.parse().map_err(|_| Error::VirtualHostInvalid(input))?),
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
    ) -> Result<Connection<TcpStream, TcpStream>, Error> {
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
            .transpose()?;
        Ok(Connection::new(
            read,
            write,
            desired_version,
            path,
            vhost,
            ConnectMode::Daemon,
        ))
    }
}
pub use connect::connect;
