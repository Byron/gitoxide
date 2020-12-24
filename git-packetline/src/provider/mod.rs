use crate::{decode, PacketLine, MAX_LINE_LEN, U16_HEX_BYTES};
use bstr::ByteSlice;
use std::io;

mod read;
pub use read::ReadWithSidebands;

/// Read pack lines one after another, without consuming more than needed from the underlying
/// [`Read`][io::Read]. [`Flush`][PacketLine::Flush] lines cause the reader to stop producing lines forever,
/// leaving [`Read`][io::Read] at the start of whatever comes next.
pub struct Provider<T> {
    inner: T,
    peek_buf: Vec<u8>,
    fail_on_err_lines: bool,
    buf: Vec<u8>,
    delimiters: &'static [PacketLine<'static>],
    is_done: bool,
    stopped_at: Option<PacketLine<'static>>,
}

impl<T> Provider<T>
where
    T: io::Read,
{
    /// Return a new instance from `read` which will stop decoding packet lines when receiving one of the given `delimiters`.
    pub fn new(read: T, delimiters: &'static [PacketLine<'static>]) -> Self {
        Provider {
            inner: read,
            buf: vec![0; MAX_LINE_LEN],
            peek_buf: Vec::new(),
            delimiters,
            fail_on_err_lines: false,
            is_done: false,
            stopped_at: None,
        }
    }

    /// Returns the packet line that stopped the iteration, or
    /// `None` if the end wasn't reached yet, on EOF, or if [`fail_on_err_lines()`][Provider::fail_on_err_lines()] was true.
    pub fn stopped_at(&self) -> Option<PacketLine<'static>> {
        self.stopped_at
    }

    /// Replace the reader used with the given `read`, resetting all other iteration state as well.
    pub fn replace(&mut self, read: T) -> T {
        let prev = std::mem::replace(&mut self.inner, read);
        self.reset();
        self.fail_on_err_lines = false;
        prev
    }

    /// Reset all iteration state allowing to continue a stopped iteration that is not yet at EOF.
    ///
    /// This can happen once a delimiter is reached.
    pub fn reset(&mut self) {
        let delimiters = std::mem::take(&mut self.delimiters);
        self.reset_with(delimiters);
    }

    /// Similar to [`reset()`][Provider::reset()] with support to changing the `delimiters`.
    pub fn reset_with(&mut self, delimiters: &'static [PacketLine<'static>]) {
        self.delimiters = delimiters;
        self.is_done = false;
        self.stopped_at = None;
    }

    /// If `value` is `true` the provider will check for special `ERR` packet lines and stop iteration when one is encountered.
    ///
    /// Use [`stopped_at()]`[Provider::stopped_at()] to inspect the cause of the end of the iteration.
    pub fn fail_on_err_lines(&mut self, value: bool) {
        self.fail_on_err_lines = value;
    }

    fn read_line_inner<'a>(reader: &mut T, buf: &'a mut Vec<u8>) -> io::Result<Result<PacketLine<'a>, decode::Error>> {
        let (hex_bytes, data_bytes) = buf.split_at_mut(4);
        reader.read_exact(hex_bytes)?;
        let num_data_bytes = match decode::hex_prefix(hex_bytes) {
            Ok(decode::PacketLineOrWantedSize::Line(line)) => return Ok(Ok(line)),
            Ok(decode::PacketLineOrWantedSize::Wanted(additional_bytes)) => additional_bytes as usize,
            Err(err) => return Ok(Err(err)),
        };

        let (data_bytes, _) = data_bytes.split_at_mut(num_data_bytes);
        reader.read_exact(data_bytes)?;
        match decode::to_data_line(data_bytes) {
            Ok(line) => Ok(Ok(line)),
            Err(err) => Ok(Err(err)),
        }
    }

    /// Read a packet line into the internal buffer and return it.
    ///
    /// Returns `None` if the end of iteration is reached because of one of the following:
    ///
    ///  * EOF
    ///  * ERR packet line encountered if [`fail_on_err_lines()`][Provider::fail_on_err_lines()] is true.
    ///  * A `delimiter` packet line encountered
    pub fn read_line(&mut self) -> Option<io::Result<Result<PacketLine<'_>, decode::Error>>> {
        if self.is_done {
            return None;
        }
        if !self.peek_buf.is_empty() {
            std::mem::swap(&mut self.peek_buf, &mut self.buf);
            self.peek_buf.clear();
            return Some(Ok(Ok(crate::decode(&self.buf).expect("only valid data in peek buf"))));
        } else if self.buf.len() != MAX_LINE_LEN {
            self.buf.resize(MAX_LINE_LEN, 0);
        }
        match Self::read_line_inner(&mut self.inner, &mut self.buf) {
            Ok(Ok(line)) => {
                if self.delimiters.contains(&line) {
                    self.is_done = true;
                    self.stopped_at = self.delimiters.iter().find(|l| **l == line).cloned();
                    None
                } else if self.fail_on_err_lines {
                    match line.check_error() {
                        Some(err) => {
                            self.is_done = true;
                            Some(Err(io::Error::new(io::ErrorKind::Other, err.0.as_bstr().to_string())))
                        }
                        None => Some(Ok(Ok(line))),
                    }
                } else {
                    Some(Ok(Ok(line)))
                }
            }
            res => Some(res),
        }
    }

    /// Modify the peek buffer, overwriting the byte at `position` with the given byte to `replace_with`.
    ///
    /// **Note** that `position` does not include the 4 bytes prefix (they are invisible outside the reader)
    pub fn peek_buffer_replace_and_truncate(&mut self, position: usize, replace_with: u8) {
        let position = position + U16_HEX_BYTES;
        self.peek_buf[position] = replace_with;

        let new_len = position + 1;
        self.peek_buf.truncate(new_len);
        self.peek_buf[..4].copy_from_slice(&crate::encode::u16_to_hex((new_len) as u16));
    }

    /// Peek the next packet line without consuming it.
    ///
    /// Multiple calls to peek will return the same packet line, if there is one.
    pub fn peek_line(&mut self) -> Option<io::Result<Result<PacketLine<'_>, decode::Error>>> {
        if self.is_done {
            return None;
        }
        Some(if self.peek_buf.is_empty() {
            self.peek_buf.resize(MAX_LINE_LEN, 0);
            match Self::read_line_inner(&mut self.inner, &mut self.peek_buf) {
                Ok(Ok(line)) => {
                    if self.delimiters.contains(&line) {
                        self.is_done = true;
                        self.stopped_at = self.delimiters.iter().find(|l| **l == line).cloned();
                        self.peek_buf.clear();
                        return None;
                    } else if self.fail_on_err_lines {
                        if let Some(err) = line.check_error() {
                            self.is_done = true;
                            let err = err.0.as_bstr().to_string();
                            self.peek_buf.clear();
                            return Some(Err(io::Error::new(io::ErrorKind::Other, err)));
                        }
                    }
                    let len = line
                        .as_slice()
                        .map(|s| s.len() + U16_HEX_BYTES)
                        .unwrap_or(U16_HEX_BYTES);
                    self.peek_buf.resize(len, 0);
                    Ok(Ok(crate::decode(&self.peek_buf).expect("only valid data here")))
                }
                Ok(Err(err)) => {
                    self.peek_buf.clear();
                    Ok(Err(err))
                }
                Err(err) => {
                    self.peek_buf.clear();
                    Err(err)
                }
            }
        } else {
            Ok(Ok(crate::decode(&self.peek_buf).expect("only valid data here")))
        })
    }

    pub fn as_read_with_sidebands<F: FnMut(bool, &[u8])>(&mut self, handle_progress: F) -> ReadWithSidebands<'_, T, F> {
        ReadWithSidebands::with_progress_handler(self, handle_progress)
    }

    pub fn as_read_without_sidebands<F: FnMut(bool, &[u8])>(&mut self) -> ReadWithSidebands<'_, T, F> {
        ReadWithSidebands::without_progress_handler(self)
    }

    pub fn as_read(&mut self) -> ReadWithSidebands<'_, T, fn(bool, &[u8])> {
        ReadWithSidebands::new(self)
    }
}
