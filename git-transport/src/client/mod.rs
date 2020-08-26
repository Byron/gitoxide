use crate::{Protocol, Service};
use std::io;

pub mod connect;
pub mod file;
pub mod git;
#[cfg(feature = "http-client-curl")]
pub mod http;
pub mod ssh;
#[doc(inline)]
pub use connect::connect;

#[cfg(feature = "http-client-curl")]
type HttpError = http::Error;
#[cfg(not(feature = "http-client-curl"))]
type HttpError = std::convert::Infallible;

pub mod capabilities;
#[doc(inline)]
pub use capabilities::Capabilities;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("An IO error occurred when talking to the server")]
    Io {
        #[from]
        err: io::Error,
    },
    #[error("Capabilities could not be parsed")]
    Capabilities {
        #[from]
        err: capabilities::Error,
    },
    #[error("A packet line could not be decoded")]
    LineDecode {
        #[from]
        err: git_packetline::decode::Error,
    },
    #[error("A {0} line was expected, but there was none")]
    ExpectedLine(&'static str),
    #[error("Expected a data line, but got a delimiter")]
    ExpectedDataLine,
    #[error(transparent)]
    Http(#[from] HttpError),
}

pub struct SetServiceResponse<'a> {
    /// The protocol the service can provide. May be different from the requested one
    pub actual_protocol: Protocol,
    pub capabilities: Capabilities,
    /// In protocol version one, this is set to a list of refs and their peeled counterparts.
    pub refs: Option<Box<dyn io::BufRead + 'a>>,
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum WriteMode {
    Binary,
    OneLFTerminatedLinePerWriteCall,
}

impl Default for WriteMode {
    fn default() -> Self {
        WriteMode::OneLFTerminatedLinePerWriteCall
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum MessageKind {
    Flush,
    Text(&'static [u8]),
}

/// A type implementing `Write`, which when done can be transformed into a `Read` for obtaining the response.
pub struct RequestWriter<'a> {
    pub(crate) writer: Box<dyn io::Write + 'a>,
    pub(crate) reader: Box<dyn io::BufRead + 'a>,
}

impl<'a> io::Write for RequestWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<'a> RequestWriter<'a> {
    pub fn new<R: io::Read, W: io::Write + 'a>(
        writer: W,
        line_provider: &'a mut git_packetline::Provider<R>,
        write_mode: WriteMode,
        on_drop: Vec<MessageKind>,
        handle_progress: Option<HandleProgress>,
    ) -> Self {
        let mut writer = git_packetline::Writer::new(writer);
        match write_mode {
            WriteMode::Binary => writer.enable_binary_mode(),
            WriteMode::OneLFTerminatedLinePerWriteCall => writer.enable_text_mode(),
        }
        let writer: Box<dyn io::Write> = if on_drop.is_empty() {
            Box::new(writer)
        } else {
            Box::new(WritePacketOnDrop::new(writer, on_drop))
        };
        RequestWriter {
            writer,
            reader: match handle_progress {
                Some(handler) => Box::new(line_provider.as_read_with_sidebands(handler)),
                None => Box::new(line_provider.as_read()),
            },
        }
    }
    pub fn into_read(self) -> ResponseReader<'a> {
        ResponseReader { reader: self.reader }
    }
}

/// A type implementing `Read` to obtain the server response.
pub struct ResponseReader<'a> {
    reader: Box<dyn io::BufRead + 'a>,
}

impl<'a> io::Read for ResponseReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<'a> io::BufRead for ResponseReader<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.reader.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt)
    }
}

pub type HandleProgress = Box<dyn FnMut(bool, &[u8])>;

pub(crate) struct WritePacketOnDrop<W: io::Write> {
    inner: git_packetline::Writer<W>,
    on_drop: Vec<MessageKind>,
}

impl<W: io::Write> WritePacketOnDrop<W> {
    pub fn new(inner: git_packetline::Writer<W>, on_drop: Vec<MessageKind>) -> Self {
        WritePacketOnDrop { inner, on_drop }
    }
}

impl<W: io::Write> io::Write for WritePacketOnDrop<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl<W: io::Write> Drop for WritePacketOnDrop<W> {
    fn drop(&mut self) {
        for msg in self.on_drop.drain(..) {
            match msg {
                MessageKind::Flush => git_packetline::PacketLine::Flush.to_write(&mut self.inner),
                MessageKind::Text(t) => git_packetline::borrowed::Text::from(t).to_write(&mut self.inner),
            }
            .expect("packet line write on drop must work or we may as well panic to prevent weird surprises");
        }
    }
}

/// All methods provided here must be called in the correct order according to the communication protocol used to connect to them.
/// It does, however, know just enough to be able to provide a higher-level interface than would otherwise be possible.
/// Thus the consumer of this trait will not have to deal with packet lines at all.
/// Generally, whenever a `Read` trait or `Write` trait is produced, it must be exhausted..
pub trait Transport {
    /// Initiate connection to the given service.
    /// Returns the service capabilities according according to the actual Protocol it supports,
    /// and possibly a list of refs to be obtained.
    /// This means that asking for an unsupported protocol will result in a protocol downgrade to the given one.
    /// using the `read_line(…)` function of the given BufReader. It must be exhausted, that is, read to the end,
    /// before the next method can be invoked.
    fn handshake(&mut self, service: Service) -> Result<SetServiceResponse, Error>;

    /// Obtain a writer for sending data and obtaining the response. It can be configured in various ways,
    /// and should to support with the task at hand.
    /// `send_mode` determines how calls to the `write(…)` method are interpreted, and `on_drop` determines what
    /// to do when the writer is consumed or dropped.
    /// If `handle_progress` is not None, it's function passed a text line without trailing LF from which progress information can be parsed.
    fn request(
        &mut self,
        write_mode: WriteMode,
        on_drop: Vec<MessageKind>,
        handle_progress: Option<HandleProgress>,
    ) -> Result<RequestWriter, Error>;
}
