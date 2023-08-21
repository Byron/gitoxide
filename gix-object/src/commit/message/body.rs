use std::ops::Deref;

use winnow::{
    combinator::{eof, rest, separated_pair, terminated},
    error::{ErrorKind, ParserError},
    prelude::*,
    token::take_until1,
};

use crate::{
    bstr::{BStr, ByteSlice},
    commit::message::BodyRef,
};

/// An iterator over trailers as parsed from a commit message body.
///
/// lines with parsing failures will be skipped
pub struct Trailers<'a> {
    pub(crate) cursor: &'a [u8],
}

/// A trailer as parsed from the commit message body.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TrailerRef<'a> {
    /// The name of the trailer, like "Signed-off-by", up to the separator ": "
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub token: &'a BStr,
    /// The value right after the separator ": ", with leading and trailing whitespace trimmed.
    /// Note that multi-line values aren't currently supported.
    pub value: &'a BStr,
}

fn parse_single_line_trailer<'a, E: ParserError<&'a [u8]>>(i: &mut &'a [u8]) -> PResult<(&'a BStr, &'a BStr), E> {
    *i = i.trim_end();
    let (token, value) = separated_pair(take_until1(b":".as_ref()), b": ", rest).parse_next(i)?;

    if token.trim_end().len() != token.len() || value.trim_start().len() != value.len() {
        Err(winnow::error::ErrMode::from_error_kind(i, ErrorKind::Fail).cut())
    } else {
        Ok((token.as_bstr(), value.as_bstr()))
    }
}

impl<'a> Iterator for Trailers<'a> {
    type Item = TrailerRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.is_empty() {
            return None;
        }
        for mut line in self.cursor.lines_with_terminator() {
            self.cursor = &self.cursor[line.len()..];
            if let Some(trailer) = terminated(parse_single_line_trailer::<()>, eof)
                .parse_next(&mut line)
                .ok()
                .map(|(token, value)| TrailerRef {
                    token: token.trim().as_bstr(),
                    value: value.trim().as_bstr(),
                })
            {
                return Some(trailer);
            }
        }
        None
    }
}

impl<'a> BodyRef<'a> {
    /// Parse `body` bytes into the trailer and the actual body.
    pub fn from_bytes(body: &'a [u8]) -> Self {
        body.rfind(b"\n\n")
            .map(|pos| (2, pos))
            .or_else(|| body.rfind(b"\r\n\r\n").map(|pos| (4, pos)))
            .and_then(|(sep_len, pos)| {
                let trailer = &body[pos + sep_len..];
                let body = &body[..pos];
                Trailers { cursor: trailer }.next().map(|_| BodyRef {
                    body_without_trailer: body.as_bstr(),
                    start_of_trailer: trailer,
                })
            })
            .unwrap_or_else(|| BodyRef {
                body_without_trailer: body.as_bstr(),
                start_of_trailer: &[],
            })
    }

    /// Returns the body with the trailers stripped.
    ///
    /// You can iterate trailers with the [`trailers()`][BodyRef::trailers()] method.
    pub fn without_trailer(&self) -> &'a BStr {
        self.body_without_trailer
    }

    /// Return an iterator over the trailers parsed from the last paragraph of the body. May be empty.
    pub fn trailers(&self) -> Trailers<'a> {
        Trailers {
            cursor: self.start_of_trailer,
        }
    }
}

impl<'a> AsRef<BStr> for BodyRef<'a> {
    fn as_ref(&self) -> &BStr {
        self.body_without_trailer
    }
}

impl<'a> Deref for BodyRef<'a> {
    type Target = BStr;

    fn deref(&self) -> &Self::Target {
        self.body_without_trailer
    }
}
#[cfg(test)]
mod test_parse_trailer {
    use super::*;

    fn parse(input: &str) -> (&BStr, &BStr) {
        parse_single_line_trailer::<()>.parse_peek(input.as_bytes()).unwrap().1
    }

    #[test]
    fn simple_newline() {
        assert_eq!(parse("foo: bar\n"), ("foo".into(), "bar".into()));
    }

    #[test]
    fn simple_non_ascii_no_newline() {
        assert_eq!(parse("🤗: 🎉"), ("🤗".into(), "🎉".into()));
    }

    #[test]
    fn with_lots_of_whitespace_newline() {
        assert_eq!(
            parse("hello foo: bar there   \n"),
            ("hello foo".into(), "bar there".into())
        );
    }

    #[test]
    fn extra_whitespace_before_token_or_value_is_error() {
        assert!(parse_single_line_trailer::<()>.parse_peek(b"foo : bar").is_err());
        assert!(parse_single_line_trailer::<()>.parse_peek(b"foo:  bar").is_err())
    }

    #[test]
    fn simple_newline_windows() {
        assert_eq!(parse("foo: bar\r\n"), ("foo".into(), "bar".into()));
    }
}
