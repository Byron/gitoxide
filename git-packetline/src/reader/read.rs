use crate::{
    borrowed::{Band, Text},
    Reader, RemoteProgress, MAX_DATA_LEN,
};
use bstr::ByteSlice;
use git_features::{progress, progress::Progress};
use std::io;

type ProgressAndParser<P> = (P, fn(&[u8]) -> Option<RemoteProgress>);

pub struct ReadWithProgress<'a, T, P>
where
    T: io::Read,
{
    parent: &'a mut Reader<T>,
    progress_and_parse: Option<ProgressAndParser<P>>,
    buf: Vec<u8>,
    pos: usize,
    cap: usize,
}

impl<'a, T, P> Drop for ReadWithProgress<'a, T, P>
where
    T: io::Read,
{
    fn drop(&mut self) {
        self.parent.reset();
    }
}

impl<'a, T> ReadWithProgress<'a, T, progress::Discard>
where
    T: io::Read,
{
    pub fn new(parent: &'a mut Reader<T>) -> Self {
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
    pub fn with_progress(
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
                            Band::Error(d) => progress.fail(progress_name(None, Text::from(d).as_slice())),
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
