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

pub enum WriteMode {
    Binary,
    OneLFTerminatedLinePerWriteCall,
}

impl Default for WriteMode {
    fn default() -> Self {
        WriteMode::OneLFTerminatedLinePerWriteCall
    }
}

pub enum DropBehavior {
    WriteFlush,
}

/// A type implementing `Write`, which when done can be transformed into a `Read` for obtaining the response.
pub struct RequestWriter<'a> {
    pub(crate) writer: Box<dyn io::Write + 'a>,
    pub(crate) _reader: Box<dyn io::BufRead + 'a>,
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
    pub fn into_read(self) -> ResponseReader<'a> {
        ResponseReader { _reader: self._reader }
    }
}

/// A type implementing `Read` to obtain the server response.
pub struct ResponseReader<'a> {
    _reader: Box<dyn io::BufRead + 'a>,
}

impl<'a> io::Read for ResponseReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self._reader.read(buf)
    }
}

/// All methods provided here must be called in the correct order according to the communication protocol used to connect to them.
/// It does, however, know just enough to be able to provide a higher-level interface than would otherwise be possible.
/// Thus the consumer of this trait will not have to deal with packet lines at all.
/// Generally, whenever a `Read` trait or `Write` trait is produced, it must be exhausted..
pub trait TransportSketch {
    /// Initiate connection to the given service.
    /// Returns the service capabilities according according to the actual Protocol it supports,
    /// and possibly a list of refs to be obtained.
    /// This means that asking for an unsupported protocol will result in a protocol downgrade to the given one.
    /// using the `read_line(…)` function of the given BufReader. It must be exhausted, that is, read to the end,
    /// before the next method can be invoked.
    fn set_service(&mut self, service: Service) -> Result<SetServiceResponse, Error>;

    /// Obtain a writer for sending data and obtaining the response. It can be configured in various ways,
    /// and should to support with the task at hand.
    /// `send_mode` determines how calls to the `write(…)` method are interpreted, and `on_drop` determines what
    /// to do when the writer is consumed or dropped.
    /// If `handle_progress` is not None, it's function passed a text line without trailing LF from which progress information can be parsed.
    fn request(
        &mut self,
        _write_mode: WriteMode,
        _on_drop: Option<DropBehavior>,
        _handle_progress: Option<Box<dyn FnMut(&[u8])>>,
    ) -> Result<RequestWriter, Error>;
}

pub trait Transport {}
