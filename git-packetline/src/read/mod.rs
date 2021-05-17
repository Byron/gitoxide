use crate::{PacketLine, MAX_LINE_LEN, U16_HEX_BYTES};

/// Read pack lines one after another, without consuming more than needed from the underlying
/// [`Read`][std::io::Read]. [`Flush`][PacketLine::Flush] lines cause the reader to stop producing lines forever,
/// leaving [`Read`][std::io::Read] at the start of whatever comes next.
///
/// This implementation tries hard not to allocate at all which leads to quite some added complexity and plenty of extra memory copies.
pub struct StreamingPeekableIter<T> {
    read: T,
    peek_buf: Vec<u8>,
    buf: Vec<u8>,
    fail_on_err_lines: bool,
    delimiters: &'static [PacketLine<'static>],
    is_done: bool,
    stopped_at: Option<PacketLine<'static>>,
}

impl<T> StreamingPeekableIter<T> {
    /// Return a new instance from `read` which will stop decoding packet lines when receiving one of the given `delimiters`.
    pub fn new(read: T, delimiters: &'static [PacketLine<'static>]) -> Self {
        StreamingPeekableIter {
            read,
            buf: vec![0; MAX_LINE_LEN],
            peek_buf: Vec::new(),
            delimiters,
            fail_on_err_lines: false,
            is_done: false,
            stopped_at: None,
        }
    }

    /// Modify the peek buffer, overwriting the byte at `position` with the given byte to `replace_with` while truncating
    /// it to contain only bytes until the newly replaced `position`.
    ///
    /// This is useful if you would want to remove 'special bytes' hidden behind, say a NULL byte to disappear and allow
    /// standard line readers to read the next line as usual.
    ///
    /// **Note** that `position` does not include the 4 bytes prefix (they are invisible outside the reader)
    pub fn peek_buffer_replace_and_truncate(&mut self, position: usize, replace_with: u8) {
        let position = position + U16_HEX_BYTES;
        self.peek_buf[position] = replace_with;

        let new_len = position + 1;
        self.peek_buf.truncate(new_len);
        self.peek_buf[..4].copy_from_slice(&crate::encode::u16_to_hex((new_len) as u16));
    }

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
mod blocking_io;

#[cfg(all(not(feature = "blocking-io"), feature = "async-io"))]
mod async_io;

mod sidebands;
pub use sidebands::WithSidebands;
