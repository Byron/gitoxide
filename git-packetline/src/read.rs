use crate::{
    borrowed::{Band, Text},
    decode, MAX_DATA_LEN, MAX_LINE_LEN, U16_HEX_BYTES,
};
use crate::{PacketLine, RemoteProgress};
use bstr::ByteSlice;
use git_features::{progress, progress::Progress};
use std::io;

/// Read pack lines one after another, without consuming more than needed from the underlying
/// `Read`. `Flush` lines cause the reader to stop producing lines forever, leaving `Read` at the
/// start of whatever comes next.
pub struct Reader<T> {
    pub inner: T,
    peek_buf: Vec<u8>,
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
            peek_buf: Vec::new(),
            delimiter: delimiter.into().unwrap_or(PacketLine::Flush),
            is_done: false,
        }
    }

    pub fn reset(&mut self) {
        debug_assert!(self.is_done, "reset is only effective if we are actually done");
        self.is_done = false;
    }

    pub fn reset_with(&mut self, delimiter: PacketLine<'static>) {
        self.delimiter = delimiter;
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

    pub fn read_line(&mut self) -> Option<io::Result<Result<PacketLine, decode::Error>>> {
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
            Ok(Ok(line)) if line == self.delimiter => {
                self.is_done = true;
                None
            }
            res => Some(res),
        }
    }

    pub fn peek_line(&mut self) -> Option<io::Result<Result<PacketLine, decode::Error>>> {
        if self.is_done {
            return None;
        }
        Some(if self.peek_buf.is_empty() {
            self.peek_buf.resize(MAX_LINE_LEN, 0);
            match Self::read_line_inner(&mut self.inner, &mut self.peek_buf) {
                Ok(Ok(line)) => {
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

    pub fn as_read_with_sidebands<P: Progress>(
        &mut self,
        progress: P,
        parse_progress: fn(&[u8]) -> Option<RemoteProgress>,
    ) -> ReadWithProgress<T, P> {
        ReadWithProgress::with_progress(self, progress, parse_progress)
    }

    pub fn as_read(&mut self) -> ReadWithProgress<T, progress::Discard> {
        ReadWithProgress::new(self)
    }
}

type ProgressAndParser<P> = (P, fn(&[u8]) -> Option<RemoteProgress>);

pub struct ReadWithProgress<'a, T, P> {
    parent: &'a mut Reader<T>,
    progress_and_parse: Option<ProgressAndParser<P>>,
    buf: Vec<u8>,
    pos: usize,
    cap: usize,
}

impl<'a, T> ReadWithProgress<'a, T, progress::Discard>
where
    T: io::Read,
{
    fn new(parent: &'a mut Reader<T>) -> Self {
        ReadWithProgress {
            parent,
            progress_and_parse: None,
            buf: vec![0; MAX_DATA_LEN],
            pos: 0,
            cap: 0,
        }
    }
}

impl<'a, T, P> ReadWithProgress<'a, T, P>
where
    T: io::Read,
    P: Progress,
{
    fn with_progress(
        parent: &'a mut Reader<T>,
        progress: P,
        parse_progress: fn(&[u8]) -> Option<RemoteProgress>,
    ) -> Self {
        ReadWithProgress {
            parent,
            progress_and_parse: Some((progress, parse_progress)),
            buf: vec![0; MAX_DATA_LEN],
            pos: 0,
            cap: 0,
        }
    }
}

impl<'a, T, P> io::BufRead for ReadWithProgress<'a, T, P>
where
    T: io::Read,
    P: Progress,
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
                match self.progress_and_parse.as_mut() {
                    Some((progress, parse_progress)) => {
                        let mut band = line
                            .decode_band()
                            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
                        fn progress_name(current: Option<String>, action: &[u8]) -> String {
                            match current {
                                Some(current) => format!("{}: {}", current, action.as_bstr()),
                                None => action.as_bstr().to_string(),
                            }
                        }
                        match band {
                            Band::Data(ref mut d) => break d.read(&mut self.buf)?,
                            Band::Progress(d) => {
                                let text = Text::from(d).0;
                                match (parse_progress)(text) {
                                    Some(RemoteProgress {
                                        action,
                                        percent: _,
                                        step,
                                        max,
                                    }) => {
                                        progress.set_name(progress_name(progress.name(), action));
                                        progress.init(max, git_features::progress::count("objects"));
                                        if let Some(step) = step {
                                            progress.set(step);
                                        }
                                    }
                                    None => progress.set_name(progress_name(progress.name(), text)),
                                };
                            }
                            Band::Error(d) => progress.fail(progress_name(None, Text::from(d).0)),
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

impl<'a, T, P> io::Read for ReadWithProgress<'a, T, P>
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
