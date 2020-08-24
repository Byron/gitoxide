use crate::{
    borrowed::{Band, Text},
    Reader, MAX_DATA_LEN,
};
use std::io;

pub struct ReadWithSidebands<'a, T, F>
where
    T: io::Read,
{
    parent: &'a mut Reader<T>,
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
    pub fn new(parent: &'a mut Reader<T>) -> Self {
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
    pub fn with_progress_handler(parent: &'a mut Reader<T>, handle_progress: F) -> Self {
        ReadWithSidebands {
            parent,
            handle_progress: Some(handle_progress),
            buf: vec![0; MAX_DATA_LEN],
            pos: 0,
            cap: 0,
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
                                (handle_progress)(false, text);
                            }
                            Band::Error(d) => {
                                let text = Text::from(d).0;
                                (handle_progress)(true, text);
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
