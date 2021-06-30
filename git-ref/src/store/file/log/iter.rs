use crate::store::file::log;
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
        // FromEnd(usize),
    }

    impl std::fmt::Display for LineNumber {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let (line, suffix) = match self {
                LineNumber::FromStart(line) => (line, ""),
                // LineNumber::FromEnd(line) => (line, " from the end"),
            };
            write!(f, "{}{}", line + 1, suffix)
        }
    }
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
        log::line::decode::line::<()>(&line)
            .map(|(_, line)| line)
            .map_err(|_| decode::Error::new(line, decode::LineNumber::FromStart(ln)))
    })
}

/// An iterator yielding parsed lines in a file in reverse.
#[allow(dead_code)]
pub struct Reverse<'a, F> {
    buf: &'a mut [u8],
    read: Option<F>,
    iter: Option<std::iter::Peekable<bstr::SplitReverse<'a>>>,
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
pub fn reverse<F>(log: F, buf: &mut [u8]) -> Reverse<'_, F>
where
    F: std::io::Read + std::io::Seek,
{
    Reverse {
        buf,
        read: Some(log),
        iter: None,
    }
}

impl<'a, F> Iterator for Reverse<'a, F> {
    type Item = Result<log::Line<'a>, decode::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("reverse iteration")
    }
}
