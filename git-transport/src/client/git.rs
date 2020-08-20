use crate::{client::SetServiceResponse, Protocol, Service};
use bstr::{BString, ByteVec};
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

pub fn connect_message(
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

impl<R, W> crate::client::TransportSketch for Connection<R, W>
where
    R: io::Read,
    W: io::Write,
{
    fn set_service(&mut self, service: Service) -> Result<SetServiceResponse, crate::client::Error> {
        self.write.write_all(&connect_message(
            service,
            self.protocol,
            &self.path,
            self.virtual_host.as_ref(),
        ))?;
        self.write.flush()?;
        let capabilities = self
            .line_reader
            .peek_line()
            .ok_or(crate::client::Error::ExpectedLine("capabilities or version"))???;

        Ok(SetServiceResponse {
            actual_protocol: Protocol::V1, // TODO - read actual only if we are in version two or above
            capabilities: Capabilities::try_from(
                capabilities
                    .to_text()
                    .ok_or(crate::client::Error::ExpectedDataLine)?
                    .as_slice(),
            )?,
            refs: None, // TODO
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

use crate::client::Capabilities;
use quick_error::quick_error;
use std::convert::TryFrom;
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
