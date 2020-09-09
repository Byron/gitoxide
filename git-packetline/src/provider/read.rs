use crate::{
    borrowed::{Band, Text},
    PacketLine, Provider, MAX_DATA_LEN,
};
use std::io;

/// Note: Reading from this intermediary copies bytes 3 times:
/// OS -> (parent) line provider buffer -> our buffer -> caller's output buffer
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
    pub fn with_progress_handler(parent: &'a mut Provider<T>, handle_progress: F) -> Self {
        ReadWithSidebands {
            parent,
            handle_progress: Some(handle_progress),
            buf: vec![0; MAX_DATA_LEN],
            pos: 0,
            cap: 0,
        }
    }

    pub fn without_progress_handler(parent: &'a mut Provider<T>) -> Self {
        ReadWithSidebands {
            parent,
            handle_progress: None,
            buf: vec![0; MAX_DATA_LEN],
            pos: 0,
            cap: 0,
        }
    }

    pub fn reset_with(&mut self, delimiters: &'static [PacketLine<'static>]) {
        self.parent.reset_with(delimiters)
    }

    pub fn stopped_at(&self) -> Option<PacketLine<'static>> {
        self.parent.stopped_at
    }

    pub fn set_progress_handler(&mut self, handle_progress: Option<F>) {
        self.handle_progress = handle_progress;
    }

    pub fn peek_data_line(&mut self) -> Option<io::Result<Result<&[u8], crate::decode::Error>>> {
        match self.parent.peek_line() {
            Some(Ok(Ok(line))) => match line {
                crate::PacketLine::Data(line) => Some(Ok(Ok(line))),
                _ => None,
            },
            Some(Ok(Err(err))) => Some(Ok(Err(err))),
            Some(Err(err)) => Some(Err(err)),
            None => None,
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
