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
    use std::io;

    pub fn capabilties_and_possibly_refs<'a, T: io::Read>(
        rd: &'a mut git_packetline::Provider<T>,
        version: Protocol,
    ) -> Result<(Capabilities, Option<Box<dyn io::BufRead + 'a>>), client::Error> {
        rd.fail_on_err_lines(true);
        match version {
            Protocol::V1 => {
                let capabilities = rd
                    .peek_line()
                    .ok_or(client::Error::ExpectedLine("capabilities or version"))???;
                let (capabilities, delimiter_position) = Capabilities::from_bytes(
                    capabilities
                        .to_text()
                        .ok_or(client::Error::ExpectedLine("text"))?
                        .as_slice(),
                )?;
                rd.peek_buffer_replace_and_truncate(delimiter_position, b'\n');
                Ok((capabilities, Some(Box::new(rd.as_read()))))
            }
            Protocol::V2 => Ok((Capabilities::from_lines(rd.as_read())?, None)),
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
    version: Protocol,
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
                self.version,
                &self.path,
                self.virtual_host.as_ref(),
            ))?;
            line_writer.flush()?;
        }

        let (capabilities, refs) = recv::capabilties_and_possibly_refs(&mut self.line_provider, self.version)?;
        Ok(SetServiceResponse {
            actual_protocol: self.version, // verified by capability parsing. Version is otherwise assumed V1
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
            version: desired_version,
            mode,
        }
    }
    pub(crate) fn new_for_spawned_process(reader: R, writer: W, path: BString, version: Protocol) -> Self {
        Self::new(reader, writer, version, path, None::<(&str, _)>, ConnectMode::Process)
    }
}
use quick_error::quick_error;
quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Tbd {
            display("tbd")
        }
    }
}

pub fn connect(
    _host: &str,
    _path: BString,
    _version: crate::Protocol,
    _port: Option<u16>,
) -> Result<Connection<TcpStream, TcpStream>, Error> {
    unimplemented!("file connection")
}
