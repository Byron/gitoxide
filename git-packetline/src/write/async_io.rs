use crate::{encode, MAX_DATA_LEN, U16_HEX_BYTES};
use futures_io::AsyncWrite;
use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};

pin_project_lite::pin_project! {
    /// An implementor of [`Write`][io::Write] which passes all input to an inner `Write` in packet line data encoding,
    /// one line per `write(…)` call or as many lines as it takes if the data doesn't fit into the maximum allowed line length.
    pub struct Writer<T> {
        #[pin]
        inner: encode::LineWriter<'static, T>,
        state: State
    }
}

enum State {
    Idle,
    WriteData(usize),
}

impl<T: AsyncWrite + Unpin> Writer<T> {
    /// Create a new instance from the given `write`
    pub fn new(write: T) -> Self {
        Writer {
            inner: encode::LineWriter::new(write, &[], &[]),
            state: State::Idle,
        }
    }

    /// Return the inner writer, consuming self.
    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

/// Non-IO methods
impl<T: AsyncWrite + Unpin> Writer<T> {
    /// If called, each call to [`write()`][io::Write::write()] will write bytes as is.
    pub fn enable_binary_mode(&mut self) {
        self.inner.suffix = &[];
    }
    /// If called, each call to [`write()`][io::Write::write()] will write the input as text, appending a trailing newline
    /// if needed before writing.
    pub fn enable_text_mode(&mut self) {
        self.inner.suffix = &[b'\n'];
    }
    /// As [`enable_text_mode()`][Writer::enable_text_mode()], but suitable for chaining.
    pub fn text_mode(mut self) -> Self {
        self.enable_text_mode();
        self
    }
    /// As [`enable_binary_mode()`][Writer::enable_binary_mode()], but suitable for chaining.
    pub fn binary_mode(mut self) -> Self {
        self.enable_binary_mode();
        self
    }
}

impl<T: AsyncWrite + Unpin> AsyncWrite for Writer<T> {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        let mut this = self.project();
        loop {
            match this.state {
                State::Idle => {
                    if buf.is_empty() {
                        return Poll::Ready(Err(io::Error::new(
                            io::ErrorKind::Other,
                            "empty packet lines are not permitted as '0004' is invalid",
                        )));
                    }
                    *this.state = State::WriteData(0)
                }
                State::WriteData(written) => {
                    while *written != buf.len() {
                        let data = &buf[*written..*written + (buf.len() - *written).min(MAX_DATA_LEN)];
                        let n = futures_lite::ready!(this.inner.as_mut().poll_write(cx, data))?;
                        if n == 0 {
                            return Poll::Ready(Err(io::ErrorKind::WriteZero.into()));
                        }
                        *written += n;
                        *written -= U16_HEX_BYTES + this.inner.suffix.len();
                    }
                    *this.state = State::Idle;
                    return Poll::Ready(Ok(buf.len()));
                }
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.project().inner.poll_close(cx)
    }
}
