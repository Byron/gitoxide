use crate::MAX_DATA_LEN;
use quick_error::quick_error;

quick_error! {
    /// The error returned by most functions in the [`encode`][crate::encode] module
    #[derive(Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        Io(err: std::io::Error) {
            display("An error occurred while writing")
            from()
            source(err)
        }
        DataLengthLimitExceeded(length_in_bytes: usize) {
            display("Cannot encode more than {} bytes, got {}", MAX_DATA_LEN, length_in_bytes)
        }
        DataIsEmpty {
            display("Empty lines are invalid")
        }
    }
}

// #[cfg(all(not(feature = "blocking-io"), feature = "async-io"))]
mod async_io {
    use super::u16_to_hex;
    use crate::{encode::Error, MAX_DATA_LEN};
    use futures_io::AsyncWrite;
    use futures_lite::AsyncWriteExt;
    use std::{
        io,
        pin::Pin,
        task::{Context, Poll},
    };

    pin_project_lite::pin_project! {
        /// A way of writing packet lines asynchronously.
        pub struct LineWriter<'a, W: ?Sized> {
            #[pin]
            writer: &'a mut W,
            prefix: &'a [u8],
            suffix: &'a [u8],
            state: State,
        }
    }
    enum State {
        Idle,
        WriteHexLen([u8; 4]),
        WritePrefix,
        WriteData,
        WriteSuffix,
    }

    impl<W: AsyncWrite + Unpin + ?Sized> AsyncWrite for LineWriter<'_, W> {
        fn poll_write(mut self: Pin<&mut Self>, cx: &mut Context<'_>, data: &[u8]) -> Poll<io::Result<usize>> {
            fn into_io_err(err: Error) -> io::Error {
                io::Error::new(io::ErrorKind::Other, err)
            }
            loop {
                match &mut self.state {
                    State::Idle => {
                        let data_len = self.prefix.len() + data.len() + self.suffix.len();
                        if data_len > MAX_DATA_LEN {
                            return Poll::Ready(Err(into_io_err(Error::DataLengthLimitExceeded(data_len))));
                        }
                        if data.is_empty() {
                            return Poll::Ready(Err(into_io_err(Error::DataIsEmpty)));
                        }
                        let data_len = data_len + 4;
                        let len_buf = u16_to_hex(data_len as u16);
                        self.state = State::WriteHexLen(len_buf)
                    }
                    State::WriteHexLen(hex_len) => {
                        {
                            let mut this = self.as_mut().project();
                            futures_lite::ready!(this.writer.poll_write(cx, hex_len.as_ref()));
                        }
                        if !self.prefix.is_empty() {
                            self.state = State::WritePrefix
                        } else {
                            self.state = State::WriteData
                        }
                    }
                    _ => todo!("other states"),
                }
            }
        }

        fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            let mut this = self.project();
            this.writer.poll_flush(cx)
        }

        fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            let mut this = self.project();
            this.writer.poll_close(cx)
        }
    }

    async fn prefixed_and_suffixed_data_to_write(
        prefix: &[u8],
        data: &[u8],
        suffix: &[u8],
        mut out: impl AsyncWrite + Unpin,
    ) -> Result<usize, Error> {
        let data_len = prefix.len() + data.len() + suffix.len();
        if data_len > MAX_DATA_LEN {
            return Err(Error::DataLengthLimitExceeded(data_len));
        }
        if data.is_empty() {
            return Err(Error::DataIsEmpty);
        }

        let data_len = data_len + 4;
        let buf = u16_to_hex(data_len as u16);

        out.write_all(&buf).await?;
        if !prefix.is_empty() {
            out.write_all(prefix).await?;
        }
        out.write_all(data).await?;
        if !suffix.is_empty() {
            out.write_all(suffix).await?;
        }
        Ok(data_len)
    }

    async fn prefixed_data_to_write(prefix: &[u8], data: &[u8], out: impl AsyncWrite + Unpin) -> Result<usize, Error> {
        prefixed_and_suffixed_data_to_write(prefix, data, &[], out).await
    }

    /// Write a `text` message to `out`, which is assured to end in a newline.
    pub async fn text_to_write(text: &[u8], out: impl AsyncWrite + Unpin) -> Result<usize, Error> {
        prefixed_and_suffixed_data_to_write(&[], text, &[b'\n'], out).await
    }

    /// Write a `data` message to `out`.
    pub async fn data_to_write(data: &[u8], out: impl AsyncWrite + Unpin) -> Result<usize, Error> {
        prefixed_data_to_write(&[], data, out).await
    }
}
#[cfg(all(not(feature = "blocking-io"), feature = "async-io"))]
pub use async_io::*;

#[cfg(feature = "blocking-io")]
mod blocking_io {
    use super::u16_to_hex;
    use crate::{encode::Error, Channel, DELIMITER_LINE, ERR_PREFIX, FLUSH_LINE, MAX_DATA_LEN, RESPONSE_END_LINE};
    use std::io;

    /// Write a response-end message to `out`.
    pub fn response_end_to_write(mut out: impl io::Write) -> io::Result<usize> {
        out.write_all(RESPONSE_END_LINE).map(|_| 4)
    }

    /// Write a delim message to `out`.
    pub fn delim_to_write(mut out: impl io::Write) -> io::Result<usize> {
        out.write_all(DELIMITER_LINE).map(|_| 4)
    }

    /// Write a flush message to `out`.
    pub fn flush_to_write(mut out: impl io::Write) -> io::Result<usize> {
        out.write_all(FLUSH_LINE).map(|_| 4)
    }

    /// Write an error `message` to `out`.
    pub fn error_to_write(message: &[u8], out: impl io::Write) -> Result<usize, Error> {
        prefixed_data_to_write(ERR_PREFIX, message, out)
    }

    /// Write `data` of `kind` to `out` using side-band encoding.
    pub fn band_to_write(kind: Channel, data: &[u8], out: impl io::Write) -> Result<usize, Error> {
        prefixed_data_to_write(&[kind as u8], data, out)
    }

    /// Write a `data` message to `out`.
    pub fn data_to_write(data: &[u8], out: impl io::Write) -> Result<usize, Error> {
        prefixed_data_to_write(&[], data, out)
    }

    /// Write a `text` message to `out`, which is assured to end in a newline.
    pub fn text_to_write(text: &[u8], out: impl io::Write) -> Result<usize, Error> {
        prefixed_and_suffixed_data_to_write(&[], text, &[b'\n'], out)
    }

    fn prefixed_data_to_write(prefix: &[u8], data: &[u8], out: impl io::Write) -> Result<usize, Error> {
        prefixed_and_suffixed_data_to_write(prefix, data, &[], out)
    }

    fn prefixed_and_suffixed_data_to_write(
        prefix: &[u8],
        data: &[u8],
        suffix: &[u8],
        mut out: impl io::Write,
    ) -> Result<usize, Error> {
        let data_len = prefix.len() + data.len() + suffix.len();
        if data_len > MAX_DATA_LEN {
            return Err(Error::DataLengthLimitExceeded(data_len));
        }
        if data.is_empty() {
            return Err(Error::DataIsEmpty);
        }

        let data_len = data_len + 4;
        let buf = u16_to_hex(data_len as u16);

        out.write_all(&buf)?;
        if !prefix.is_empty() {
            out.write_all(prefix)?;
        }
        out.write_all(data)?;
        if !suffix.is_empty() {
            out.write_all(suffix)?;
        }
        Ok(data_len)
    }
}
#[cfg(feature = "blocking-io")]
pub use blocking_io::*;

#[cfg(any(feature = "async-io", feature = "blocking-io"))]
pub(crate) fn u16_to_hex(value: u16) -> [u8; 4] {
    let mut buf = [0u8; 4];
    hex::encode_to_slice((value as u16).to_be_bytes(), &mut buf).expect("two bytes to 4 hex chars never fails");
    buf
}
