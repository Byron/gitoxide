use crate::{decode, PacketLine, StreamingPeekableIter, MAX_LINE_LEN, U16_HEX_BYTES};
use bstr::ByteSlice;
use futures_io::AsyncRead;
use futures_lite::AsyncReadExt;
use std::io;

type ExhaustiveOutcome<'a> = (
    bool,                                                      // is_done
    Option<PacketLine<'static>>,                               // stopped_at
    Option<io::Result<Result<PacketLine<'a>, decode::Error>>>, // actual method result
);

/// Non-IO methods
impl<T> StreamingPeekableIter<T>
where
    T: AsyncRead + Unpin,
{
    async fn read_line_inner<'a>(
        reader: &mut T,
        buf: &'a mut Vec<u8>,
    ) -> io::Result<Result<PacketLine<'a>, decode::Error>> {
        let (hex_bytes, data_bytes) = buf.split_at_mut(4);
        reader.read_exact(hex_bytes).await?;
        let num_data_bytes = match decode::hex_prefix(hex_bytes) {
            Ok(decode::PacketLineOrWantedSize::Line(line)) => return Ok(Ok(line)),
            Ok(decode::PacketLineOrWantedSize::Wanted(additional_bytes)) => additional_bytes as usize,
            Err(err) => return Ok(Err(err)),
        };

        let (data_bytes, _) = data_bytes.split_at_mut(num_data_bytes);
        reader.read_exact(data_bytes).await?;
        match decode::to_data_line(data_bytes) {
            Ok(line) => Ok(Ok(line)),
            Err(err) => Ok(Err(err)),
        }
    }

    /// This function is needed to help the borrow checker allow us to return references all the time
    /// It contains a bunch of logic shared between peek and read_line invocations.
    async fn read_line_inner_exhaustive<'a>(
        reader: &mut T,
        buf: &'a mut Vec<u8>,
        delimiters: &[PacketLine<'static>],
        fail_on_err_lines: bool,
        buf_resize: bool,
    ) -> ExhaustiveOutcome<'a> {
        (
            false,
            None,
            Some(match Self::read_line_inner(reader, buf).await {
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
    pub async fn read_line(&mut self) -> Option<io::Result<Result<PacketLine<'_>, decode::Error>>> {
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
                &self.delimiters,
                self.fail_on_err_lines,
                false,
            )
            .await;
            self.is_done = is_done;
            self.stopped_at = stopped_at;
            res
        }
    }

    /// Peek the next packet line without consuming it.
    ///
    /// Multiple calls to peek will return the same packet line, if there is one.
    pub async fn peek_line(&mut self) -> Option<io::Result<Result<PacketLine<'_>, decode::Error>>> {
        if self.is_done {
            return None;
        }
        if self.peek_buf.is_empty() {
            self.peek_buf.resize(MAX_LINE_LEN, 0);
            let (is_done, stopped_at, res) = Self::read_line_inner_exhaustive(
                &mut self.read,
                &mut self.peek_buf,
                &self.delimiters,
                self.fail_on_err_lines,
                true,
            )
            .await;
            self.is_done = is_done;
            self.stopped_at = stopped_at;
            res
        } else {
            Some(Ok(Ok(crate::decode(&self.peek_buf).expect("only valid data here"))))
        }
    }
}
