use crate::{client::SetServiceResponse, Protocol, Service};
use bstr::BString;
use std::{io, net::TcpStream};

pub struct Connection<R, W> {
    write: W,
    line_reader: git_packetline::Reader<R>,
    path: BString,
    virtual_host: Option<(String, Option<u16>)>,
    protocol: Protocol,
}

impl<R, W> crate::client::Transport for Connection<R, W>
where
    R: io::Read,
    W: io::Write,
{
}

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
        if version != Protocol::V1 {
            out.push(0);
            out.push_str(format!("version={}", version as usize));
            out.push(0);
        }
        out
    }
}

pub(crate) mod recv {
    use crate::client::Capabilities;
    use std::io;

    pub fn capabilties_and_possibly_refs<'a, T: io::Read>(
        rd: &'a mut git_packetline::Reader<T>,
    ) -> Result<(Capabilities, Option<Box<dyn io::BufRead + 'a>>), crate::client::Error> {
        rd.fail_on_err_lines(true);
        let capabilities = rd
            .peek_line()
            .ok_or(crate::client::Error::ExpectedLine("capabilities or version"))???;
        let (capabilities, delimiter_position) = Capabilities::from_bytes(
            capabilities
                .to_text()
                .ok_or(crate::client::Error::ExpectedDataLine)?
                .as_slice(),
        )?;
        rd.peek_buffer_replace_and_truncate(delimiter_position, b'\n');
        Ok((capabilities, Some(Box::new(rd.as_read()))))
    }
}

impl<R, W> crate::client::TransportSketch for Connection<R, W>
where
    R: io::Read,
    W: io::Write,
{
    fn set_service(&mut self, service: Service) -> Result<SetServiceResponse, crate::client::Error> {
        self.write.write_all(&message::connect(
            service,
            self.protocol,
            &self.path,
            self.virtual_host.as_ref(),
        ))?;
        self.write.flush()?;

        let (capabilities, refs) = recv::capabilties_and_possibly_refs(&mut self.line_reader)?;
        Ok(SetServiceResponse {
            actual_protocol: Protocol::V1, // TODO - read actual only if we are in version two or above
            capabilities,
            refs,
        })
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
    ) -> Self {
        Connection {
            write,
            line_reader: git_packetline::Reader::new(read, None),
            path: repository_path.into(),
            virtual_host: virtual_host.map(|(h, p)| (h.into(), p)),
            protocol: desired_version,
        }
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
