use crate::store::file::log;
use crate::store::file::log::iter::decode::LineNumber;
use bstr::ByteSlice;

///
pub mod decode {
    use bstr::{BString, ByteSlice};

    /// The error returned by items in the [forward][super::forward()] iterator
    #[derive(Debug)]
    pub struct Error {
        input: BString,
        line: LineNumber,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "In line {}: {:?} did not match '<old-hexsha> <new-hexsha> <name> <<email>> <timestamp> <tz>\\t<message>'", self.line, self.input)
        }
    }

    impl<'a> std::error::Error for Error {}

    impl Error {
        pub(crate) fn new(input: &[u8], line: LineNumber) -> Self {
            Error {
                line,
                input: input.as_bstr().to_owned(),
            }
        }
    }

    #[derive(Debug)]
    pub(crate) enum LineNumber {
        FromStart(usize),
        FromEnd(usize),
    }

    impl std::fmt::Display for LineNumber {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let (line, suffix) = match self {
                LineNumber::FromStart(line) => (line, ""),
                LineNumber::FromEnd(line) => (line, " from the end"),
            };
            write!(f, "{}{}", line + 1, suffix)
        }
    }
}

fn convert<'a, E: nom::error::ParseError<&'a [u8]> + nom::error::ContextError<&'a [u8]>>(
    input: &'a [u8],
    ln: decode::LineNumber,
    parsed: nom::IResult<&'a [u8], log::Line<'a>, E>,
) -> Result<log::Line<'a>, decode::Error> {
    parsed.map(|(_, line)| line).map_err(|_| decode::Error::new(input, ln))
}

/// Returns a forward iterator over the given `lines`, starting from the first line in the file and ending at the last.
///
/// Note that `lines` are an entire reflog file.
///
/// This iterator is useful when the ref log file is going to be rewritten which forces processing of the entire file.
/// It will continue parsing even if individual log entries failed to parse, leaving it to the driver to decide whether to
/// abort or continue.
pub fn forward(lines: &[u8]) -> impl Iterator<Item = Result<log::Line<'_>, decode::Error>> {
    lines.as_bstr().lines().enumerate().map(|(ln, line)| {
        convert(
            &line,
            decode::LineNumber::FromStart(ln),
            log::line::decode::one::<()>(&line),
        )
    })
}

/// An iterator yielding parsed lines in a file in reverse.
#[allow(dead_code)]
pub struct Reverse<F, const SIZE: usize> {
    buf: [u8; SIZE],
    count: usize,
    read_and_pos: Option<(F, u64)>,
    last_nl_pos: Option<usize>,
}

/// An iterator over entries of the `log` file in reverse, using `buf` as sliding window.
///
/// Note that `buf` must be big enough to capture typical line length or else partial lines will be parsed and probably fail
/// in the process.
///
/// This iterator is very expensive in terms of I/O operations and shouldn't be used to read more than the last few entries of the log.
/// Use a forward iterator instead for these cases.
///
/// It will continue parsing even if individual log entries failed to parse, leaving it to the driver to decide whether to
/// abort or continue.
pub fn reverse<F, const SIZE: usize>(mut log: F) -> std::io::Result<Reverse<F, SIZE>>
where
    F: std::io::Read + std::io::Seek,
{
    let pos = log.seek(std::io::SeekFrom::End(0))?;
    Ok(Reverse {
        buf: [0; SIZE],
        count: 0,
        read_and_pos: Some((log, pos)),
        last_nl_pos: None,
    })
}

impl<F, const SIZE: usize> Iterator for Reverse<F, SIZE>
where
    F: std::io::Read + std::io::Seek,
{
    type Item = std::io::Result<Result<log::mutable::Line, decode::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.last_nl_pos.take(), self.read_and_pos.take()) {
            // Initial state - load first data block
            (None, Some((mut read, pos))) => {
                let npos = pos.saturating_sub(self.buf.len() as u64);
                if let Err(err) = read.seek(std::io::SeekFrom::Start(npos)) {
                    return Some(Err(err));
                }

                let n = (pos - npos) as usize;
                if n == 0 {
                    return None;
                }
                let buf = &mut self.buf[..n];
                if let Err(err) = read.read_exact(buf) {
                    return Some(Err(err));
                };

                let last_byte = *buf.last().expect("we have read non-zero bytes before");
                self.last_nl_pos = Some(if last_byte != b'\n' { buf.len() } else { buf.len() - 1 });
                self.read_and_pos = Some((read, npos));
                self.next()
            }
            // Has data block and can extract lines from it, load new blocks as needed
            (Some(end), Some(read_and_pos)) => match self.buf[..end].rfind_byte(b'\n') {
                Some(start) => {
                    self.read_and_pos = Some(read_and_pos);
                    self.last_nl_pos = Some(start);
                    let buf = &self.buf[start + 1..end];
                    let res = Some(Ok(convert(
                        buf,
                        LineNumber::FromEnd(self.count),
                        log::line::decode::one::<()>(buf),
                    )
                    .map(Into::into)));
                    self.count += 1;
                    res
                }
                None => {
                    let (_read, last_read_pos) = read_and_pos;
                    if last_read_pos == 0 {
                        let buf = &self.buf[..end];
                        Some(Ok(convert(
                            buf,
                            LineNumber::FromEnd(self.count),
                            log::line::decode::one::<()>(buf),
                        )
                        .map(Into::into)))
                    } else {
                        todo!("load more we are not yet done, handle remaining buffer content")
                    }
                }
            },
            // depleted
            (None, None) => None,
            (Some(_), None) => unreachable!("BUG: Invalid state: we never discard only our file, always both."),
        }
    }
}
