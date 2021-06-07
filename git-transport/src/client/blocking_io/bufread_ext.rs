use crate::{
    client::{Error, MessageKind},
    Protocol,
};
use std::{
    io,
    ops::{Deref, DerefMut},
};
/// A function `f(is_error, text)` receiving progress or error information.
pub type HandleProgress = Box<dyn FnMut(bool, &[u8])>;

/// This trait exists to get a version of a `git_packetline::Provider` without type parameters.
/// For the sake of usability, it also implements [`std::io::BufRead`] making it trivial to
/// read pack files while keeping open the option to read individual lines with low overhead.
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
