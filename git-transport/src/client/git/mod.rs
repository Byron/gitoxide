use crate::{client, client::SetServiceResponse, client::WritePacketOnDrop, Protocol, Service};
use bstr::BString;
use git_packetline::PacketLine;
use std::{io, io::Write, net::TcpStream};

pub mod message;
pub(crate) mod recv;

pub struct Connection<R, W> {
    writer: W,
    line_reader: git_packetline::Provider<R>,
    path: BString,
    virtual_host: Option<(String, Option<u16>)>,
    version: Protocol,
}

impl<R, W> client::TransportSketch for Connection<R, W>
where
    R: io::Read,
    W: io::Write,
{
    fn handshake(&mut self, service: Service) -> Result<SetServiceResponse, client::Error> {
        let mut line_writer = git_packetline::Writer::new(&mut self.writer).binary_mode();
        line_writer.write_all(&message::connect(
            service,
            self.version,
            &self.path,
            self.virtual_host.as_ref(),
        ))?;
        line_writer.flush()?;

        let (capabilities, refs) = recv::capabilties_and_possibly_refs(&mut self.line_reader, self.version)?;
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
        handle_progress: Option<client::HandleProgress>,
    ) -> Result<client::RequestWriter, client::Error> {
        let mut writer = git_packetline::Writer::new(&mut self.writer);
        match write_mode {
            client::WriteMode::Binary => writer.enable_binary_mode(),
            client::WriteMode::OneLFTerminatedLinePerWriteCall => writer.enable_text_mode(),
        }
        let writer: Box<dyn io::Write> = if !on_drop.is_empty() {
            Box::new(WritePacketOnDrop::new(writer, on_drop))
        } else {
            Box::new(writer)
        };
        Ok(client::RequestWriter {
            writer,
            reader: match handle_progress {
                Some(handler) => Box::new(self.line_reader.as_read_with_sidebands(handler)),
                None => Box::new(self.line_reader.as_read()),
            },
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
            writer: write,
            line_reader: git_packetline::Provider::new(read, PacketLine::Flush),
            path: repository_path.into(),
            virtual_host: virtual_host.map(|(h, p)| (h.into(), p)),
            version: desired_version,
        }
    }
}

impl<R, W> client::Transport for Connection<R, W>
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
