use crate::PacketLine;

/// Read pack lines one after another, without consuming more than needed from the underlying
/// [`Read`][io::Read]. [`Flush`][PacketLine::Flush] lines cause the reader to stop producing lines forever,
/// leaving [`Read`][io::Read] at the start of whatever comes next.
///
/// This implementation tries hard not to allocate at all which leads to quite some added complexity and plenty of extra memory copies.
pub struct StreamingPeekableIter<T> {
    read: T,
    #[cfg(feature = "blocking-io")]
    peek_buf: Vec<u8>,
    #[cfg(feature = "blocking-io")]
    buf: Vec<u8>,
    fail_on_err_lines: bool,
    delimiters: &'static [PacketLine<'static>],
    is_done: bool,
    stopped_at: Option<PacketLine<'static>>,
}

impl<T> StreamingPeekableIter<T> {
    /// Returns the packet line that stopped the iteration, or
    /// `None` if the end wasn't reached yet, on EOF, or if [`fail_on_err_lines()`][StreamingPeekableIter::fail_on_err_lines()] was true.
    pub fn stopped_at(&self) -> Option<PacketLine<'static>> {
        self.stopped_at
    }

    /// Reset all iteration state allowing to continue a stopped iteration that is not yet at EOF.
    ///
    /// This can happen once a delimiter is reached.
    pub fn reset(&mut self) {
        let delimiters = std::mem::take(&mut self.delimiters);
        self.reset_with(delimiters);
    }

    /// Similar to [`reset()`][StreamingPeekableIter::reset()] with support to changing the `delimiters`.
    pub fn reset_with(&mut self, delimiters: &'static [PacketLine<'static>]) {
        self.delimiters = delimiters;
        self.is_done = false;
        self.stopped_at = None;
    }

    /// If `value` is `true` the provider will check for special `ERR` packet lines and stop iteration when one is encountered.
    ///
    /// Use [`stopped_at()]`[StreamingPeekableIter::stopped_at()] to inspect the cause of the end of the iteration.
    /// ne
    pub fn fail_on_err_lines(&mut self, value: bool) {
        self.fail_on_err_lines = value;
    }

    /// Replace the reader used with the given `read`, resetting all other iteration state as well.
    pub fn replace(&mut self, read: T) -> T {
        let prev = std::mem::replace(&mut self.read, read);
        self.reset();
        self.fail_on_err_lines = false;
        prev
    }
}

#[cfg(feature = "blocking-io")]
mod blocking;
#[cfg(feature = "blocking-io")]
pub use blocking::WithSidebands;
