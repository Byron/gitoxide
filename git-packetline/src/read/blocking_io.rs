use crate::{
    decode,
    read::{ExhaustiveOutcome, WithSidebands},
    PacketLine, StreamingPeekableIter, MAX_LINE_LEN, U16_HEX_BYTES,
};
use bstr::ByteSlice;
use std::io;

/// Non-IO methods
impl<T> StreamingPeekableIter<T>
where
    T: io::Read,
{
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

    /// This function is needed to help the borrow checker allow us to return references all the time
    /// It contains a bunch of logic shared between peek and read_line invocations.
    fn read_line_inner_exhaustive<'a>(
        reader: &mut T,
        buf: &'a mut Vec<u8>,
        delimiters: &[PacketLine<'static>],
        fail_on_err_lines: bool,
        buf_resize: bool,
    ) -> ExhaustiveOutcome<'a> {
        (
            false,
            None,
            Some(match Self::read_line_inner(reader, buf) {
                Ok(Ok(line)) => {
                    if delimiters.contains(&line) {
                        let stopped_at = delimiters.iter().find(|l| **l == line).cloned();
                        buf.clear();
                        return (true, stopped_at, None);
                    } else if fail_on_err_lines {
                        if let Some(err) = line.check_error() {
                            let err = err.0.as_bstr().to_string();
                            buf.clear();
                            return (true, None, Some(Err(io::Error::new(io::ErrorKind::Other, err))));
                        }
                    }
                    let len = line
                        .as_slice()
                        .map(|s| s.len() + U16_HEX_BYTES)
                        .unwrap_or(U16_HEX_BYTES);
                    if buf_resize {
                        buf.resize(len, 0);
                    }
                    Ok(Ok(crate::decode(buf).expect("only valid data here")))
                }
                Ok(Err(err)) => {
                    buf.clear();
                    Ok(Err(err))
                }
                Err(err) => {
                    buf.clear();
                    Err(err)
                }
            }),
        )
    }

    /// Read a packet line into the internal buffer and return it.
    ///
    /// Returns `None` if the end of iteration is reached because of one of the following:
    ///
    ///  * natural EOF
    ///  * ERR packet line encountered if [`fail_on_err_lines()`][StreamingPeekableIter::fail_on_err_lines()] is true.
    ///  * A `delimiter` packet line encountered
    pub fn read_line(&mut self) -> Option<io::Result<Result<PacketLine<'_>, decode::Error>>> {
        if self.is_done {
            return None;
        }
        if !self.peek_buf.is_empty() {
            std::mem::swap(&mut self.peek_buf, &mut self.buf);
            self.peek_buf.clear();
            Some(Ok(Ok(crate::decode(&self.buf).expect("only valid data in peek buf"))))
        } else {
            if self.buf.len() != MAX_LINE_LEN {
                self.buf.resize(MAX_LINE_LEN, 0);
            }
            let (is_done, stopped_at, res) = Self::read_line_inner_exhaustive(
                &mut self.read,
                &mut self.buf,
                self.delimiters,
                self.fail_on_err_lines,
                false,
            );
            self.is_done = is_done;
            self.stopped_at = stopped_at;
            res
        }
    }

    /// Peek the next packet line without consuming it.
    ///
    /// Multiple calls to peek will return the same packet line, if there is one.
    pub fn peek_line(&mut self) -> Option<io::Result<Result<PacketLine<'_>, decode::Error>>> {
        if self.is_done {
            return None;
        }
        if self.peek_buf.is_empty() {
            self.peek_buf.resize(MAX_LINE_LEN, 0);
            let (is_done, stopped_at, res) = Self::read_line_inner_exhaustive(
                &mut self.read,
                &mut self.peek_buf,
                self.delimiters,
                self.fail_on_err_lines,
                true,
            );
            self.is_done = is_done;
            self.stopped_at = stopped_at;
            res
        } else {
            Some(Ok(Ok(crate::decode(&self.peek_buf).expect("only valid data here"))))
        }
    }

    /// Return this instance as implementor of [`Read`][io::Read] assuming side bands to be used in all received packet lines.
    /// Each invocation of [`read_line()`][io::BufRead::read_line()] returns a packet line.
    ///
    /// Progress or error information will be passed to the given `handle_progress(is_error, text)` function, with `is_error: bool`
    /// being true in case the `text` is to be interpreted as error.
    ///
    /// _Please note_ that side bands need to be negotiated with the server.
    pub fn as_read_with_sidebands<F: FnMut(bool, &[u8])>(&mut self, handle_progress: F) -> WithSidebands<'_, T, F> {
        WithSidebands::with_progress_handler(self, handle_progress)
    }

    /// Same as [`as_read_with_sidebands(…)`][StreamingPeekableIter::as_read_with_sidebands()], but for channels without side band support.
    ///
    /// The type parameter `F` needs to be configured for this method to be callable using the 'turbofish' operator.
    /// Use [`as_read()`][StreamingPeekableIter::as_read()].
    pub fn as_read_without_sidebands<F: FnMut(bool, &[u8])>(&mut self) -> WithSidebands<'_, T, F> {
        WithSidebands::without_progress_handler(self)
    }

    /// Same as [`as_read_with_sidebands(…)`][StreamingPeekableIter::as_read_with_sidebands()], but for channels without side band support.
    ///
    /// Due to the preconfigured function type this method can be called without 'turbofish'.
    pub fn as_read(&mut self) -> WithSidebands<'_, T, fn(bool, &[u8])> {
        WithSidebands::new(self)
    }
}
