use crate::{
    borrowed::{Band, Text},
    PacketLine, Provider, MAX_DATA_LEN,
};
use std::io;

/// An implementor of [`BufRead`][io::BufRead] yielding packet lines on each call to [`read_line()`][io::BufRead::read_line()].
/// It's also possible to hide the underlying packet lines using the [`Read`][io::Read] implementation which is useful
/// if they represent binary data, like the one of a pack file.
///
/// # Performance Notice
/// Reading from this intermediary copies bytes 3 times:
/// OS -> (parent) line provider buffer -> our buffer -> caller's output buffer
/// which won't make this very efficient for huge bandwidths.
pub struct ReadWithSidebands<'a, T, F>
where
    T: io::Read,
{
    parent: &'a mut Provider<T>,
    handle_progress: Option<F>,
    buf: Vec<u8>,
    pos: usize,
    cap: usize,
}

impl<'a, T, F> Drop for ReadWithSidebands<'a, T, F>
where
    T: io::Read,
{
    fn drop(&mut self) {
        self.parent.reset();
    }
}

impl<'a, T> ReadWithSidebands<'a, T, fn(bool, &[u8])>
where
    T: io::Read,
{
    /// Create a new instance with the given provider as `parent`.
    pub fn new(parent: &'a mut Provider<T>) -> Self {
        ReadWithSidebands {
            parent,
            handle_progress: None,
            buf: vec![0; MAX_DATA_LEN],
            pos: 0,
            cap: 0,
        }
    }
}

impl<'a, T, F> ReadWithSidebands<'a, T, F>
where
    T: io::Read,
    F: FnMut(bool, &[u8]),
{
    /// Create a new instance with the given `parent` provider and the `handle_progress` function.
    ///
    /// Progress or error information will be passed to the given `handle_progress(is_error, text)` function, with `is_error: bool`
    /// being true in case the `text` is to be interpreted as error.
    pub fn with_progress_handler(parent: &'a mut Provider<T>, handle_progress: F) -> Self {
        ReadWithSidebands {
            parent,
            handle_progress: Some(handle_progress),
            buf: vec![0; MAX_DATA_LEN],
            pos: 0,
            cap: 0,
        }
    }

    /// Create a new instance without a progress handler.
    pub fn without_progress_handler(parent: &'a mut Provider<T>) -> Self {
        ReadWithSidebands {
            parent,
            handle_progress: None,
            buf: vec![0; MAX_DATA_LEN],
            pos: 0,
            cap: 0,
        }
    }

    /// Forwards to the parent [Provider::reset_with()]
    pub fn reset_with(&mut self, delimiters: &'static [PacketLine<'static>]) {
        self.parent.reset_with(delimiters)
    }

    /// Forwards to the parent [Provider::stopped_at()]
    pub fn stopped_at(&self) -> Option<PacketLine<'static>> {
        self.parent.stopped_at
    }

    /// Set or unset the progress handler.
    pub fn set_progress_handler(&mut self, handle_progress: Option<F>) {
        self.handle_progress = handle_progress;
    }

    /// Effectively forwards to the parent [Provider::peek_line()], allowing to see what would be returned
    /// next on a call to [`read_line()`][io::BufRead::read_line()].
    pub fn peek_data_line(&mut self) -> Option<io::Result<Result<&[u8], crate::decode::Error>>> {
        match self.parent.peek_line() {
            Some(Ok(Ok(crate::PacketLine::Data(line)))) => Some(Ok(Ok(line))),
            Some(Ok(Err(err))) => Some(Ok(Err(err))),
            Some(Err(err)) => Some(Err(err)),
            _ => None,
        }
    }
}

impl<'a, T, F> io::BufRead for ReadWithSidebands<'a, T, F>
where
    T: io::Read,
    F: FnMut(bool, &[u8]),
{
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        use io::Read;
        if self.pos >= self.cap {
            debug_assert!(self.pos == self.cap);
            self.cap = loop {
                let line = match self.parent.read_line() {
                    Some(line) => line?.map_err(|err| io::Error::new(io::ErrorKind::Other, err))?,
                    None => break 0,
                };
                match self.handle_progress.as_mut() {
                    Some(handle_progress) => {
                        let mut band = line
                            .decode_band()
                            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
                        match band {
                            Band::Data(ref mut d) => break d.read(&mut self.buf)?,
                            Band::Progress(d) => {
                                let text = Text::from(d).0;
                                handle_progress(false, text);
                            }
                            Band::Error(d) => {
                                let text = Text::from(d).0;
                                handle_progress(true, text);
                            }
                        };
                    }
                    None => {
                        break match line.as_slice() {
                            Some(ref mut d) => d.read(&mut self.buf)?,
                            None => {
                                return Err(io::Error::new(
                                    io::ErrorKind::UnexpectedEof,
                                    "encountered non-data line in a data-line only context",
                                ))
                            }
                        }
                    }
                }
            };
            self.pos = 0;
        }
        Ok(&self.buf[self.pos..self.cap])
    }

    fn consume(&mut self, amt: usize) {
        self.pos = std::cmp::min(self.pos + amt, self.cap);
    }

    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        assert_eq!(
            self.cap, 0,
            "we don't support partial buffers right now - read-line must be used consistently"
        );
        let line = std::str::from_utf8(self.fill_buf()?)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
            .unwrap();
        buf.push_str(line);
        let bytes = line.len();
        self.cap = 0;
        Ok(bytes)
    }
}

impl<'a, T, F> io::Read for ReadWithSidebands<'a, T, F>
where
    T: io::Read,
    F: FnMut(bool, &[u8]),
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        use std::io::BufRead;
        let nread = {
            let mut rem = self.fill_buf()?;
            rem.read(buf)?
        };
        self.consume(nread);
        Ok(nread)
    }
}
