use crate::{
    client::{Error, MessageKind, WriteMode},
    Protocol,
};
use std::{
    io,
    ops::{Deref, DerefMut},
};

/// A function `f(is_error, text)` receiving progress or error information.
pub type HandleProgress = Box<dyn FnMut(bool, &[u8])>;

/// A [`Write`][io::Write] implementation optimized for writing packet lines.
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
            WriteMode::OneLfTerminatedLinePerWriteCall => writer.enable_text_mode(),
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
            MessageKind::Text(t) => git_packetline::immutable::Text::from(t).to_write(&mut self.writer.inner),
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
