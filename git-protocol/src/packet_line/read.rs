use crate::packet_line::{
    borrowed::{Band, Text},
    decode, MAX_DATA_LEN, MAX_LINE_LEN,
};
use crate::{PacketLine, RemoteProgress};
use bstr::ByteSlice;
use git_features::progress::Progress;
use std::io;

/// Read pack lines one after another, without consuming more than needed from the underlying
/// `Read`. `Flush` lines cause the reader to stop producing lines forever, leaver `Read` at the
/// start of whatever comes next.
pub struct Reader<T> {
    pub inner: T,
    buf: Vec<u8>,
    delimiter: PacketLine<'static>,
    is_done: bool,
}

impl<T> Reader<T>
where
    T: io::Read,
{
    pub fn new(inner: T, delimiter: impl Into<Option<PacketLine<'static>>>) -> Self {
        Reader {
            inner,
            buf: vec![0; MAX_LINE_LEN],
            delimiter: delimiter.into().unwrap_or(PacketLine::Flush),
            is_done: false,
        }
    }

    pub fn reset(&mut self) {
        debug_assert!(self.is_done, "reset is only effective if we are actually done");
        self.is_done = false;
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

    pub fn read_line(&mut self) -> io::Result<Result<PacketLine, decode::Error>> {
        let eof = || {
            Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "attempt to read past flush line",
            ))
        };
        if self.is_done {
            return eof();
        }
        match Self::read_line_inner(&mut self.inner, &mut self.buf) {
            Ok(Ok(line)) if line == self.delimiter => {
                self.is_done = true;
                eof()
            }
            err => err,
        }
    }

    pub fn as_read_with_sidebands<P: Progress>(&mut self, progress: P) -> ToRead<T, P> {
        ToRead::new(self, progress)
    }
}

pub struct ToRead<'a, T, P> {
    parent: &'a mut Reader<T>,
    progress: P,
    buf: Vec<u8>,
    pos: usize,
    cap: usize,
}
impl<'a, T, P> ToRead<'a, T, P>
where
    T: io::Read,
    P: Progress,
{
    fn new(parent: &'a mut Reader<T>, progress: P) -> Self {
        ToRead {
            parent,
            progress,
            buf: vec![0; MAX_DATA_LEN],
            pos: 0,
            cap: 0,
        }
    }
}

impl<'a, T, P> io::BufRead for ToRead<'a, T, P>
where
    T: io::Read,
    P: Progress,
{
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        use io::Read;
        if self.pos >= self.cap {
            debug_assert!(self.pos == self.cap);
            self.cap = loop {
                let line = self
                    .parent
                    .read_line()?
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
                let mut band = line
                    .decode_band()
                    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
                match band {
                    Band::Data(ref mut d) => break d.read(&mut self.buf)?,
                    Band::Progress(d) => {
                        let text = Text::from(d).0;
                        match RemoteProgress::from_bytes(text) {
                            Some(RemoteProgress {
                                action: _,
                                percent: _,
                                step,
                                max,
                            }) => {
                                self.progress.init(max, git_features::progress::count("objects"));
                                if let Some(step) = step {
                                    self.progress.set(step);
                                }
                            }
                            None => self.progress.info(text.as_bstr().to_string()),
                        };
                        // unimplemented!("progress")
                    }
                    Band::Error(d) => {
                        self.progress.fail(Text::from(d).0.as_bstr().to_string());
                    }
                };
            };
            self.pos = 0;
        }
        Ok(&self.buf[self.pos..self.cap])
    }

    fn consume(&mut self, amt: usize) {
        self.pos = std::cmp::min(self.pos + amt, self.cap);
    }
}

impl<'a, T, P> io::Read for ToRead<'a, T, P>
where
    T: io::Read,
    P: Progress,
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
