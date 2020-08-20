use crate::{
    client::{Capabilities, SetServiceResponse},
    Protocol, Service,
};
use std::{io, net::TcpStream, path::Path};

pub struct Connection<R, W> {
    read: R,
    write: W,
    protocol: Protocol,
}

impl<R, W> crate::client::Transport for Connection<R, W>
where
    R: io::Read,
    W: io::Write,
{
    fn set_service(&self) -> &[&str] {
        unimplemented!("cached capabilities")
    }

    fn command_capabilities(&self, _command: &str, _out: &mut Vec<&str>) -> bool {
        unimplemented!("command capabilities")
    }
}

impl<R, W> crate::client::TransportSketch for Connection<R, W>
where
    R: io::Read,
    W: io::Write,
{
    fn set_service(
        &mut self,
        _service: Service,
        _protocol: Protocol,
    ) -> Result<SetServiceResponse, crate::client::Error> {
        unimplemented!()
    }
}

impl<R, W> Connection<R, W>
where
    R: io::Read,
    W: io::Write,
{
    pub fn new(read: R, write: W, desired_version: Protocol) -> Self {
        Connection {
            read,
            write,
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
    _path: &Path,
    _version: crate::Protocol,
    _port: Option<u16>,
) -> Result<Connection<TcpStream, TcpStream>, Error> {
    unimplemented!("file connection")
}
