use crate::{client::SetServiceResponse, Protocol, Service};
use bstr::BString;
use std::{io, io::Write, net::TcpStream};

pub mod message;
pub(crate) mod recv;

pub struct Connection<R, W> {
    line_writer: git_packetline::Writer<W>,
    line_reader: git_packetline::Reader<R>,
    path: BString,
    virtual_host: Option<(String, Option<u16>)>,
    version: Protocol,
}

impl<R, W> crate::client::TransportSketch for Connection<R, W>
where
    R: io::Read,
    W: io::Write,
{
    fn set_service(&mut self, service: Service) -> Result<SetServiceResponse, crate::client::Error> {
        self.line_writer.write(&message::connect(
            service,
            self.version,
            &self.path,
            self.virtual_host.as_ref(),
        ))?;
        self.line_writer.flush()?;

        let (capabilities, refs) = recv::capabilties_and_possibly_refs(&mut self.line_reader, self.version)?;
        Ok(SetServiceResponse {
            actual_protocol: self.version, // verified by capability parsing. Version is otherwise assumed V1
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
            line_writer: git_packetline::Writer::new(write),
            line_reader: git_packetline::Reader::new(read, None),
            path: repository_path.into(),
            virtual_host: virtual_host.map(|(h, p)| (h.into(), p)),
            version: desired_version,
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
