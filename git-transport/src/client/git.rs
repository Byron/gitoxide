use crate::{client, client::SetServiceResponse, Protocol, Service};
use bstr::BString;
use git_packetline::PacketLine;
use std::{io, io::Write, net::TcpStream};

pub mod message {
    use crate::{Protocol, Service};
    use bstr::{BString, ByteVec};

    pub fn connect(
        service: Service,
        version: Protocol,
        path: &[u8],
        virtual_host: Option<&(String, Option<u16>)>,
    ) -> BString {
        let mut out = bstr::BString::from(service.as_str());
        out.push(b' ');
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
}
pub(crate) mod recv {
    use crate::{client, client::Capabilities, Protocol};
    use bstr::ByteSlice;
    use std::io;

    pub fn capabilities_and_possibly_refs<'a, T: io::Read>(
        rd: &'a mut git_packetline::Provider<T>,
    ) -> Result<(Capabilities, Option<Box<dyn io::BufRead + 'a>>, Protocol), client::Error> {
        rd.fail_on_err_lines(true);
        let capabilities_or_version = rd
            .peek_line()
            .ok_or(client::Error::ExpectedLine("capabilities or version"))???;
        let first_line = capabilities_or_version
            .to_text()
            .ok_or(client::Error::ExpectedLine("text"))?;

        let version = if first_line.as_bstr().starts_with_str("version ") {
            if first_line.as_bstr().ends_with_str(" 1") {
                Protocol::V1
            } else {
                Protocol::V2
            }
        } else {
            Protocol::V1
        };
        match version {
            Protocol::V1 => {
                let (capabilities, delimiter_position) = Capabilities::from_bytes(first_line.0)?;
                rd.peek_buffer_replace_and_truncate(delimiter_position, b'\n');
                Ok((capabilities, Some(Box::new(rd.as_read())), Protocol::V1))
            }
            Protocol::V2 => Ok((Capabilities::from_lines(rd.as_read())?, None, Protocol::V2)),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ConnectMode {
    Daemon,
    Process,
}

pub struct Connection<R, W> {
    writer: W,
    line_provider: git_packetline::Provider<R>,
    path: BString,
    virtual_host: Option<(String, Option<u16>)>,
    desired_version: Protocol,
    actual_version: Protocol,
    mode: ConnectMode,
}

impl<R, W> client::Transport for Connection<R, W>
where
    R: io::Read,
    W: io::Write,
{
    fn handshake(&mut self, service: Service) -> Result<SetServiceResponse, client::Error> {
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

        let (capabilities, refs, actual_protocol) = recv::capabilities_and_possibly_refs(&mut self.line_provider)?;
        Ok(SetServiceResponse {
            actual_protocol,
            capabilities,
            refs,
        })
    }

    fn request(
        &mut self,
        write_mode: client::WriteMode,
        on_drop: Vec<client::MessageKind>,
    ) -> Result<client::RequestWriter, client::Error> {
        Ok(client::RequestWriter::new_from_bufread(
            &mut self.writer,
            Box::new(self.line_provider.as_read_without_sidebands()),
            write_mode,
            on_drop,
        ))
    }

    fn close(mut self) -> Result<(), client::Error> {
        if self.actual_version == Protocol::V2 {
            git_packetline::encode::flush_to_write(&mut self.writer)?;
            self.writer.flush()?;
        }
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
}

impl<R, W> Connection<R, W>
where
    R: io::Read,
    W: io::Write,
{
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
            line_provider: git_packetline::Provider::new(read, PacketLine::Flush),
            path: repository_path.into(),
            virtual_host: virtual_host.map(|(h, p)| (h.into(), p)),
            desired_version,
            actual_version: desired_version,
            mode,
        }
    }
    pub(crate) fn new_for_spawned_process(reader: R, writer: W, repository_path: impl Into<BString>) -> Self {
        Self::new(
            reader,
            writer,
            Protocol::V1, // only V1 is actually supported, as V2 really needs a server, which is not present here
            repository_path,
            None::<(&str, _)>,
            ConnectMode::Process,
        )
    }
}

use quick_error::quick_error;
use std::net::ToSocketAddrs;
quick_error! {
    #[derive(Debug)]
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

pub fn connect(
    host: &str,
    path: BString,
    version: crate::Protocol,
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
    Ok(Connection::new(read, write, version, path, vhost, ConnectMode::Daemon))
}
