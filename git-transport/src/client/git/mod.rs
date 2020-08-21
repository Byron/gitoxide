use crate::{client::SetServiceResponse, Protocol, Service};
use bstr::BString;
use std::{io, net::TcpStream};

pub mod message;
pub(crate) mod recv;

pub struct Connection<R, W> {
    write: W,
    line_reader: git_packetline::Reader<R>,
    path: BString,
    virtual_host: Option<(String, Option<u16>)>,
    protocol: Protocol,
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

impl<R, W> crate::client::Transport for Connection<R, W>
where
    R: io::Read,
    W: io::Write,
{
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
