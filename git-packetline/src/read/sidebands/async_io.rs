use crate::{PacketLine, StreamingPeekableIter};
use futures_io::{AsyncBufRead, AsyncRead};
use std::pin::Pin;
use std::task::{Context, Poll};

/// An implementor of [`AsyncBufRead`] yielding packet lines on each call to [`read_line()`][AsyncBufRead::read_line()].
/// It's also possible to hide the underlying packet lines using the [`Read`][AsyncRead] implementation which is useful
/// if they represent binary data, like the one of a pack file.
pub struct WithSidebands<'a, T, F>
where
    T: AsyncRead,
{
    parent: &'a mut StreamingPeekableIter<T>,
    handle_progress: Option<F>,
    pos: usize,
    cap: usize,
}

impl<'a, T, F> Drop for WithSidebands<'a, T, F>
where
    T: AsyncRead,
{
    fn drop(&mut self) {
        self.parent.reset();
    }
}

impl<'a, T> WithSidebands<'a, T, fn(bool, &[u8])>
where
    T: AsyncRead,
{
    /// Create a new instance with the given provider as `parent`.
    pub fn new(parent: &'a mut StreamingPeekableIter<T>) -> Self {
        WithSidebands {
            parent,
            handle_progress: None,
            pos: 0,
            cap: 0,
        }
    }
}

impl<'a, T, F> WithSidebands<'a, T, F>
where
    T: AsyncRead + Unpin,
    F: FnMut(bool, &[u8]),
{
    /// Create a new instance with the given `parent` provider and the `handle_progress` function.
    ///
    /// Progress or error information will be passed to the given `handle_progress(is_error, text)` function, with `is_error: bool`
    /// being true in case the `text` is to be interpreted as error.
    pub fn with_progress_handler(parent: &'a mut StreamingPeekableIter<T>, handle_progress: F) -> Self {
        WithSidebands {
            parent,
            handle_progress: Some(handle_progress),
            pos: 0,
            cap: 0,
        }
    }

    /// Create a new instance without a progress handler.
    pub fn without_progress_handler(parent: &'a mut StreamingPeekableIter<T>) -> Self {
        WithSidebands {
            parent,
            handle_progress: None,
            pos: 0,
            cap: 0,
        }
    }

    /// Forwards to the parent [StreamingPeekableIter::reset_with()]
    pub fn reset_with(&mut self, delimiters: &'static [PacketLine<'static>]) {
        self.parent.reset_with(delimiters)
    }

    /// Forwards to the parent [StreamingPeekableIter::stopped_at()]
    pub fn stopped_at(&self) -> Option<PacketLine<'static>> {
        self.parent.stopped_at
    }

    /// Set or unset the progress handler.
    pub fn set_progress_handler(&mut self, handle_progress: Option<F>) {
        self.handle_progress = handle_progress;
    }

    /// Effectively forwards to the parent [StreamingPeekableIter::peek_line()], allowing to see what would be returned
    /// next on a call to [`read_line()`][io::BufRead::read_line()].
    pub async fn peek_data_line(&mut self) -> Option<std::io::Result<Result<&[u8], crate::decode::Error>>> {
        match self.parent.peek_line().await {
            Some(Ok(Ok(crate::PacketLine::Data(line)))) => Some(Ok(Ok(line))),
            Some(Ok(Err(err))) => Some(Ok(Err(err))),
            Some(Err(err)) => Some(Err(err)),
            _ => None,
        }
    }
}

impl<'a, T, F> AsyncBufRead for WithSidebands<'a, T, F>
where
    T: AsyncRead,
    F: FnMut(bool, &[u8]),
{
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<&[u8]>> {
        todo!("poll fill buf")
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        // SAFETY: self isn't moved
        #[allow(unsafe_code)]
        let this = unsafe { self.get_unchecked_mut() };
        this.pos = std::cmp::min(this.pos + amt, this.cap);
    }
}

impl<'a, T, F> AsyncRead for WithSidebands<'a, T, F>
where
    T: AsyncRead,
    F: FnMut(bool, &[u8]),
{
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<std::io::Result<usize>> {
        todo!("poll read")
    }
}
