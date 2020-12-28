use crate::{Protocol, Service};
use bstr::BString;
use std::{
    io,
    io::Write,
    ops::{Deref, DerefMut},
};

#[cfg(test)]
mod tests;

pub mod connect;
pub mod file;
pub mod git;
#[cfg(feature = "http-client-curl")]
pub mod http;
pub mod ssh;
#[doc(inline)]
pub use connect::connect;

pub mod capabilities;
#[doc(inline)]
pub use capabilities::Capabilities;

#[cfg(feature = "http-client-curl")]
type HttpError = http::Error;
#[cfg(not(feature = "http-client-curl"))]
type HttpError = std::convert::Infallible;

/// The error used in most methods of the [`client`][crate::client] module
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
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

/// The response of the [`handshake()`][Transport::handshake()] method.
pub struct SetServiceResponse<'a> {
    /// The protocol the service can provide. May be different from the requested one
    pub actual_protocol: Protocol,
    pub capabilities: Capabilities,
    /// In protocol version one, this is set to a list of refs and their peeled counterparts.
    pub refs: Option<Box<dyn io::BufRead + 'a>>,
}

/// Configure how the [`RequestWriter`] behaves when writing bytes.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum WriteMode {
    /// Each [write()][Write::write()] call writes the bytes verbatim as one or more packet lines.
    Binary,
    /// Each [write()][Write::write()] call assumes text in the input, assures a trailing newline and writes it as single packet line.
    OneLFTerminatedLinePerWriteCall,
}

impl Default for WriteMode {
    fn default() -> Self {
        WriteMode::OneLFTerminatedLinePerWriteCall
    }
}

/// The kind of packet line to write when transforming a [`RequestWriter`] into an [`ExtendedBufRead`].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum MessageKind {
    /// A `flush` packet.
    Flush,
    /// A V2 delimiter.
    Delimiter,
    /// The end of a response.
    ResponseEnd,
    /// The given text.
    Text(&'static [u8]),
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
/// An identity for use when authenticating the transport layer.
pub enum Identity {
    /// An account based identity
    Account {
        /// The user's name
        username: String,
        /// The user's password
        password: String,
    },
}

/// A [`Write`] implementation optimized for writing packet lines.
/// A type implementing `Write` for packet lines, which when done can be transformed into a `Read` for
/// obtaining the response.
pub struct RequestWriter<'a> {
    on_into_read: MessageKind,
    pub(crate) writer: git_packetline::Writer<Box<dyn io::Write + 'a>>,
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
    /// Create a new instance from a `writer` (commonly a socket), a `reader` into which to transform once the
    /// writes are finished, along with configuration for the `write_mode` and information about which message to write
    /// when this instance is converted into a `reader` to read the request's response.
    pub fn new_from_bufread<W: io::Write + 'a>(
        writer: W,
        reader: Box<dyn ExtendedBufRead + 'a>,
        write_mode: WriteMode,
        on_into_read: MessageKind,
    ) -> Self {
        let mut writer = git_packetline::Writer::new(Box::new(writer) as Box<dyn io::Write>);
        match write_mode {
            WriteMode::Binary => writer.enable_binary_mode(),
            WriteMode::OneLFTerminatedLinePerWriteCall => writer.enable_text_mode(),
        }
        RequestWriter {
            on_into_read,
            writer,
            reader,
        }
    }

    /// Discard the ability to write and turn this instance into the reader for obtaining the other side's response.
    pub fn into_read(mut self) -> io::Result<Box<dyn ExtendedBufRead + 'a>> {
        self.write_message(self.on_into_read)?;
        Ok(self.reader)
    }

    /// Write the given message as packet line.
    pub fn write_message(&mut self, message: MessageKind) -> io::Result<()> {
        match message {
            MessageKind::Flush => git_packetline::PacketLine::Flush.to_write(&mut self.writer.inner),
            MessageKind::Delimiter => git_packetline::PacketLine::Delimiter.to_write(&mut self.writer.inner),
            MessageKind::ResponseEnd => git_packetline::PacketLine::ResponseEnd.to_write(&mut self.writer.inner),
            MessageKind::Text(t) => git_packetline::borrowed::Text::from(t).to_write(&mut self.writer.inner),
        }
        .map(|_| ())
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
    }
}

/// This trait exists to get a version of a `git_packetline::Provider` without type parameters.
/// For the sake of usability, it also implements [`std::io::BufRead`] making it trivial to (eventually)
/// read pack files while keeping the possibility to read individual lines with low overhead.
pub trait ExtendedBufRead: io::BufRead {
    /// Set the handler to which progress will be delivered.
    ///
    /// Note that this is only possible if packet lines are sent in side band mode.
    fn set_progress_handler(&mut self, handle_progress: Option<HandleProgress>);
    /// Peek the next data packet line.
    fn peek_data_line(&mut self) -> Option<io::Result<Result<&[u8], Error>>>;
    /// Resets the reader to allow reading past a previous stop, and sets delimiters according to the
    /// given protocol.
    fn reset(&mut self, version: Protocol);
    /// Return the kind of message at which the reader stopped.
    fn stopped_at(&self) -> Option<MessageKind>;
}

impl<'a, T: ExtendedBufRead + ?Sized + 'a> ExtendedBufRead for Box<T> {
    fn set_progress_handler(&mut self, handle_progress: Option<HandleProgress>) {
        self.deref_mut().set_progress_handler(handle_progress)
    }

    fn peek_data_line(&mut self) -> Option<io::Result<Result<&[u8], Error>>> {
        self.deref_mut().peek_data_line()
    }

    fn reset(&mut self, version: Protocol) {
        self.deref_mut().reset(version)
    }

    fn stopped_at(&self) -> Option<MessageKind> {
        self.deref().stopped_at()
    }
}

impl<'a, T: io::Read> ExtendedBufRead for git_packetline::provider::ReadWithSidebands<'a, T, HandleProgress> {
    fn set_progress_handler(&mut self, handle_progress: Option<HandleProgress>) {
        self.set_progress_handler(handle_progress)
    }
    fn peek_data_line(&mut self) -> Option<io::Result<Result<&[u8], Error>>> {
        match self.peek_data_line() {
            Some(Ok(Ok(line))) => Some(Ok(Ok(line))),
            Some(Ok(Err(err))) => Some(Ok(Err(err.into()))),
            Some(Err(err)) => Some(Err(err)),
            None => None,
        }
    }
    fn reset(&mut self, version: Protocol) {
        match version {
            Protocol::V1 => self.reset_with(&[git_packetline::PacketLine::Flush]),
            Protocol::V2 => {
                self.reset_with(&[git_packetline::PacketLine::Delimiter, git_packetline::PacketLine::Flush])
            }
        }
    }
    fn stopped_at(&self) -> Option<MessageKind> {
        self.stopped_at().map(|l| match l {
            git_packetline::PacketLine::Flush => MessageKind::Flush,
            git_packetline::PacketLine::Delimiter => MessageKind::Delimiter,
            git_packetline::PacketLine::ResponseEnd => MessageKind::ResponseEnd,
            git_packetline::PacketLine::Data(_) => unreachable!("data cannot be a delimiter"),
        })
    }
}

/// A function `f(is_error, text)` receiving progress or error information.
pub type HandleProgress = Box<dyn FnMut(bool, &[u8])>;

/// All methods provided here must be called in the correct order according to the [communication protocol][Protocol]
/// used to connect to them.
/// It does, however, know just enough to be able to provide a higher-level interface than would otherwise be possible.
/// Thus the consumer of this trait will not have to deal with packet lines at all.
/// **Note that**  whenever a `Read` trait or `Write` trait is produced, it must be exhausted.
pub trait Transport {
    /// Initiate connection to the given service.
    /// Returns the service capabilities according according to the actual [Protocol] it supports,
    /// and possibly a list of refs to be obtained.
    /// This means that asking for an unsupported protocol will result in a protocol downgrade to the given one.
    /// using the `read_line(…)` function of the given [BufReader][SetServiceResponse::refs].
    /// It must be exhausted, that is, read to the end before the next method can be invoked.
    fn handshake(&mut self, service: Service) -> Result<SetServiceResponse<'_>, Error>;

    /// If the handshake or subsequent reads failed with [io::ErrorKind::PermissionDenied], use this method to
    /// inform the transport layer about the identity to use for subsequent calls.
    /// If authentication continues to fail even with an identity set, consider communicating this to the provider
    /// of the identity in order to mark it as invalid. Otherwise the user might have difficulty updating obsolete
    /// credentials.
    /// Please note that most transport layers are unauthenticated and thus return [an error][Error::AuthenticationUnsupported] here.
    fn set_identity(&mut self, _identity: Identity) -> Result<(), Error> {
        Err(Error::AuthenticationUnsupported)
    }
    /// Get a writer for sending data and obtaining the response. It can be configured in various ways
    /// to support the task at hand.
    /// `write_mode` determines how calls to the `write(…)` method are interpreted, and `on_into_read` determines
    /// which message to write when the writer is turned into the response reader using [`into_read()`][RequestWriter::into_read()].
    fn request(&mut self, write_mode: WriteMode, on_into_read: MessageKind) -> Result<RequestWriter<'_>, Error>;

    /// Closes the connection to indicate no further requests will be made.
    fn close(&mut self) -> Result<(), Error>;

    /// Returns the canonical URL pointing to the destination of this transport.
    /// Please note that local paths may not be represented correctly, as they will go through a potentially lossy
    /// unicode conversion.
    fn to_url(&self) -> String;

    /// Returns the protocol version that was initially desired upon connection
    /// Please note that the actual protocol might differ after the handshake was conducted in case the server
    /// did not support it.
    fn desired_protocol_version(&self) -> Protocol;

    /// Returns true if the transport is inherently stateful, or false otherwise.
    /// Not being stateful implies that certain information has to be resent on each 'turn'
    /// of the fetch negotiation.
    ///
    /// # Implementation Details
    ///
    /// This answer should not be based on the [Protocol] itself, which might enforce stateless
    /// interactions despite the connections staying intact which might imply statefulness.
    fn is_stateful(&self) -> bool;
}

pub trait TransportV2Ext {
    /// Invoke a protocol V2 style `command` with given `capabilities` and optional command specific `arguments`.
    /// The `capabilities` were communicated during the handshake.
    /// _Note:_ panics if [handshake][Transport::handshake()] wasn't performed beforehand.
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
        let mut writer = self.request(WriteMode::OneLFTerminatedLinePerWriteCall, MessageKind::Flush)?;
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
        Ok(writer.into_read()?)
    }
}
