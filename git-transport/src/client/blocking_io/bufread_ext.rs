use git_packetline::PacketLineRef;
use std::{
    io,
    ops::{Deref, DerefMut},
};

use crate::{
    client::{Error, MessageKind},
    Protocol,
};
/// A function `f(is_error, text)` receiving progress or error information.
pub type HandleProgress = Box<dyn FnMut(bool, &[u8])>;

/// This trait exists to get a version of a `git_packetline::Provider` without type parameters,
/// but leave support for reading lines directly without forcing them through `String`.
///
/// For the sake of usability, it also implements [`std::io::BufRead`] making it trivial to
/// read pack files while keeping open the option to read individual lines with low overhead.
pub trait ReadlineBufRead: io::BufRead {
    /// Read a packet line into the internal buffer and return it.
    ///
    /// Returns `None` if the end of iteration is reached because of one of the following:
    ///
    ///  * natural EOF
    ///  * ERR packet line encountered
    ///  * A `delimiter` packet line encountered
    fn readline(
        &mut self,
    ) -> Option<io::Result<Result<git_packetline::PacketLineRef<'_>, git_packetline::decode::Error>>>;
}

/// Provide even more access to the underlying packet reader.
pub trait ExtendedBufRead: ReadlineBufRead {
    /// Set the handler to which progress will be delivered.
    ///
    /// Note that this is only possible if packet lines are sent in side band mode.
    fn set_progress_handler(&mut self, handle_progress: Option<HandleProgress>);
    /// Peek the next data packet line. Maybe None if the next line is a packet we stop at, queryable using
    /// [`stopped_at()`][ExtendedBufRead::stopped_at()].
    fn peek_data_line(&mut self) -> Option<io::Result<Result<&[u8], Error>>>;
    /// Resets the reader to allow reading past a previous stop, and sets delimiters according to the
    /// given protocol.
    fn reset(&mut self, version: Protocol);
    /// Return the kind of message at which the reader stopped.
    fn stopped_at(&self) -> Option<MessageKind>;
}

impl<'a, T: ReadlineBufRead + ?Sized + 'a> ReadlineBufRead for Box<T> {
    fn readline(&mut self) -> Option<io::Result<Result<PacketLineRef<'_>, git_packetline::decode::Error>>> {
        ReadlineBufRead::readline(self.deref_mut())
    }
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

impl<T: io::Read> ReadlineBufRead for git_packetline::read::WithSidebands<'_, T, fn(bool, &[u8])> {
    fn readline(&mut self) -> Option<io::Result<Result<PacketLineRef<'_>, git_packetline::decode::Error>>> {
        self.read_data_line()
    }
}

impl<'a, T: io::Read> ReadlineBufRead for git_packetline::read::WithSidebands<'a, T, HandleProgress> {
    fn readline(&mut self) -> Option<io::Result<Result<PacketLineRef<'_>, git_packetline::decode::Error>>> {
        self.read_data_line()
    }
}

impl<'a, T: io::Read> ExtendedBufRead for git_packetline::read::WithSidebands<'a, T, HandleProgress> {
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
            Protocol::V1 => self.reset_with(&[git_packetline::PacketLineRef::Flush]),
            Protocol::V2 => self.reset_with(&[
                git_packetline::PacketLineRef::Delimiter,
                git_packetline::PacketLineRef::Flush,
            ]),
        }
    }
    fn stopped_at(&self) -> Option<MessageKind> {
        self.stopped_at().map(|l| match l {
            git_packetline::PacketLineRef::Flush => MessageKind::Flush,
            git_packetline::PacketLineRef::Delimiter => MessageKind::Delimiter,
            git_packetline::PacketLineRef::ResponseEnd => MessageKind::ResponseEnd,
            git_packetline::PacketLineRef::Data(_) => unreachable!("data cannot be a delimiter"),
        })
    }
}
