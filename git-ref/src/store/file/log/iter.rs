use crate::store::file::log;
use bstr::ByteSlice;

///
pub mod decode {
    use bstr::{BString, ByteSlice};

    /// The error returned by items in the [forward][super::forward()] iterator
    #[derive(Debug)]
    pub struct Error {
        inner: nom::error::VerboseError<BString>,
        line: LineNumber,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "In line {}: {}", self.line, self.inner)
        }
    }

    impl<'a> std::error::Error for Error {}

    impl Error {
        pub(crate) fn new(err: nom::Err<nom::error::VerboseError<&[u8]>>, line: LineNumber) -> Self {
            Error {
                line,
                inner: match err {
                    nom::Err::Error(err) | nom::Err::Failure(err) => nom::error::VerboseError {
                        errors: err
                            .errors
                            .into_iter()
                            .map(|(i, v)| (i.as_bstr().to_owned(), v))
                            .collect(),
                    },
                    nom::Err::Incomplete(_) => unreachable!("we are not a streaming parser"),
                },
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
pub fn forward(lines: &[u8]) -> impl Iterator<Item = Result<log::Line<'_>, decode::Error>> {
    lines.as_bstr().lines_with_terminator().enumerate().map(|(ln, line)| {
        log::line::decode::line(&line)
            .map(|(_, line)| line)
            .map_err(|err| decode::Error::new(err, decode::LineNumber::FromStart(ln)))
    })
}
