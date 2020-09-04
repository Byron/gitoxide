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
use bstr::BString;
#[doc(inline)]
pub use capabilities::Capabilities;
use std::io::Write;

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
    #[error("The transport layer does not support authentication")]
    AuthenticationUnsupported,
    #[error("The transport layer refuses to use a given identity: {0}")]
    AuthenticationRefused(&'static str),
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
    /// A V2 delimiter
    Delimiter,
    Text(&'static [u8]),
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
/// An identity for use when authenticating the transport layer.
pub enum Identity {
    Account { username: String, password: String },
}

/// A type implementing `Write`, which when done can be transformed into a `Read` for obtaining the response.
pub struct RequestWriter<'a> {
    pub(crate) writer: WritePacketOnDrop<Box<dyn io::Write + 'a>>,
    pub(crate) reader: Box<dyn ExtendedBufRead + 'a>,
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
    pub fn new_from_bufread<W: io::Write + 'a>(
        writer: W,
        reader: Box<dyn ExtendedBufRead + 'a>,
        write_mode: WriteMode,
        on_drop: Vec<MessageKind>,
    ) -> Self {
        let mut writer = git_packetline::Writer::new(Box::new(writer) as Box<dyn io::Write>);
        match write_mode {
            WriteMode::Binary => writer.enable_binary_mode(),
            WriteMode::OneLFTerminatedLinePerWriteCall => writer.enable_text_mode(),
        }
        RequestWriter {
            writer: WritePacketOnDrop::new(writer, on_drop),
            reader,
        }
    }
    pub fn into_read(self) -> Box<dyn ExtendedBufRead + 'a> {
        self.reader
    }

    pub fn write_message(&mut self, kind: MessageKind) -> io::Result<()> {
        self.writer.write_message(kind)
    }
}

pub trait ExtendedBufRead: io::BufRead {
    fn set_progress_handler(&mut self, handle_progress: Option<HandleProgress>);
}

impl<'a, T: io::Read> ExtendedBufRead for git_packetline::provider::ReadWithSidebands<'a, T, HandleProgress> {
    fn set_progress_handler(&mut self, handle_progress: Option<HandleProgress>) {
        self.set_progress_handler(handle_progress)
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

    pub fn write_message(&mut self, message: MessageKind) -> io::Result<()> {
        match message {
            MessageKind::Flush => git_packetline::PacketLine::Flush.to_write(&mut self.inner.inner),
            MessageKind::Delimiter => git_packetline::PacketLine::Delimiter.to_write(&mut self.inner.inner),
            MessageKind::Text(t) => git_packetline::borrowed::Text::from(t).to_write(&mut self.inner.inner),
        }
        .map(|_| ())
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
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
        let mut on_drop = std::mem::take(&mut self.on_drop);
        for msg in on_drop.drain(..) {
            self.write_message(msg)
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

    /// If the handshake or subsequent reads failed with io::ErrorKind::PermissionDenied, use this method to
    /// inform the transport layer about the identity to use for subsequent calls.
    /// If authentication continues to fail even with an identity set, consider communicating this to the provider
    /// of the identity in order to mark it as invalid. Otherwise the user might have difficulty updating obsolete
    /// credentials.
    /// Please note that most transport layers are unauthenticated and thus return an error here.
    fn set_identity(&mut self, _identity: Identity) -> Result<(), Error> {
        Err(Error::AuthenticationUnsupported)
    }
    /// Obtain a writer for sending data and obtaining the response. It can be configured in various ways,
    /// and should to support with the task at hand.
    /// `send_mode` determines how calls to the `write(…)` method are interpreted, and `on_drop` determines
    /// which messages to write when the writer is dropped. This happens naturally when switching to reading the response with `into_read()`.
    /// If `handle_progress` is not None, it's function passed a text line without trailing LF from which progress information can be parsed.
    fn request(&mut self, write_mode: WriteMode, on_drop: Vec<MessageKind>) -> Result<RequestWriter, Error>;

    /// Closes the connection to indicate no further requests will be made.
    fn close(self) -> Result<(), Error>;

    /// Returns the canonical URL pointing to the destination of this transport.
    /// Please note that local paths may not be represented correctly, as they will go through a potentially lossy
    /// unicode conversion.
    fn to_url(&self) -> String;

    /// Returns the protocol version that was initially desired upon connection
    /// Please note that the actual protocol might differ after the handshake was conducted in case the server
    /// does not support it.
    fn desired_protocol_version(&self) -> Protocol;
}

pub trait TransportV2Ext {
    /// Invoke a protocol V2 style `command` with given `capabilities` and optional command specific `arguments`.
    /// The `capabilities` were communicated during the handshake.
    /// _Note:_ panics if handshake wasn't performed beforehand.
    fn invoke<'a>(
        &mut self,
        command: &str,
        capabilities: impl IntoIterator<Item = (&'a str, Option<&'a str>)>,
        arguments: Option<impl IntoIterator<Item = bstr::BString>>,
    ) -> Result<Box<dyn ExtendedBufRead + '_>, Error>;
}

impl<T: Transport> TransportV2Ext for T {
    fn invoke<'a>(
        &mut self,
        command: &str,
        capabilities: impl IntoIterator<Item = (&'a str, Option<&'a str>)>,
        arguments: Option<impl IntoIterator<Item = BString>>,
    ) -> Result<Box<dyn ExtendedBufRead + '_>, Error> {
        let mut writer = self.request(WriteMode::OneLFTerminatedLinePerWriteCall, vec![MessageKind::Flush])?;
        writer.write_all(format!("command={}", command).as_bytes())?;
        for (name, value) in capabilities {
            match value {
                Some(value) => writer.write_all(format!("{}={}", name, value).as_bytes()),
                None => writer.write_all(name.as_bytes()),
            }?;
        }
        if let Some(arguments) = arguments {
            writer.write_message(MessageKind::Delimiter)?;
            for argument in arguments {
                writer.write_all(argument.as_ref())?;
            }
        }
        Ok(writer.into_read())
    }
}
