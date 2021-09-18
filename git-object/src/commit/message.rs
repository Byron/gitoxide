use std::borrow::Cow;

use crate::{
    bstr::{BStr, BString, ByteSlice, ByteVec},
    commit::MessageRef,
};
use std::ops::Deref;

mod decode {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_till1},
        combinator::all_consuming,
        error::ParseError,
        sequence::pair,
        IResult,
    };

    use crate::bstr::{BStr, ByteSlice};

    fn newline<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], &'a [u8], E> {
        alt((tag(b"\r\n"), tag(b"\n")))(i)
    }

    fn subject_and_body<'a, E: ParseError<&'a [u8]>>(
        i: &'a [u8],
    ) -> IResult<&'a [u8], (&'a BStr, Option<&'a BStr>), E> {
        let mut c = i;
        let mut consumed_bytes = 0;
        while !c.is_empty() {
            c = match take_till1::<_, _, E>(|c| c == b'\n' || c == b'\r')(c) {
                Ok((i1, segment)) => {
                    consumed_bytes += segment.len();
                    match pair::<_, _, _, E, _, _>(newline, newline)(i1) {
                        Ok((body, _)) => {
                            return Ok((
                                &[],
                                (
                                    &i[0usize..consumed_bytes].as_bstr(),
                                    (!body.is_empty()).then(|| body.as_bstr()),
                                ),
                            ));
                        }
                        Err(_) => match i1.get(1..) {
                            Some(next) => {
                                consumed_bytes += 1;
                                next
                            }
                            None => break,
                        },
                    }
                }
                Err(_) => match c.get(1..) {
                    Some(next) => {
                        consumed_bytes += 1;
                        next
                    }
                    None => break,
                },
            };
        }
        Ok((&[], (i.as_bstr(), None)))
    }

    /// Returns title and body, without separator
    pub fn bytes(message: &[u8]) -> (&BStr, Option<&BStr>) {
        all_consuming(subject_and_body::<()>)(message).expect("cannot fail").1
    }
}

impl<'a> MessageRef<'a> {
    /// Parse the given `input` as message.
    ///
    /// Note that this cannot fail as everything will be interpreted as title if there is no body separator.
    pub fn from_bytes(input: &'a [u8]) -> Self {
        let (title, body) = decode::bytes(input);
        MessageRef { title, body }
    }

    /// Produce a short commit summary for the message title.
    ///
    /// This means the following
    ///
    /// * Take the subject line which is delimited by two newlines (\n\n)
    /// * transform intermediate consecutive whitespace including \r into one space
    ///
    /// The resulting summary will have folded whitespace before a newline into spaces and stopped that process
    /// once two consecutive newlines are encountered.
    pub fn summary(&self) -> Cow<'a, BStr> {
        summary(self.title)
    }

    /// Further parse the body into into non-trailer and trailers, which can be iterated from the returned [`BodyRef`].
    pub fn body(&self) -> Option<BodyRef<'a>> {
        self.body.map(|b| BodyRef {
            _body_without_footer: b,
            _start_of_footer: &[],
        })
    }
}

pub fn summary(message: &BStr) -> Cow<'_, BStr> {
    let message = message.trim();
    match message.find_byte(b'\n') {
        Some(mut pos) => {
            let mut out = BString::default();
            let mut previous_pos = None;
            loop {
                if let Some(previous_pos) = previous_pos {
                    if previous_pos + 1 == pos {
                        let len_after_trim = out.trim_end().len();
                        out.resize(len_after_trim, 0);
                        break out.into();
                    }
                }
                let message_to_newline = &message[previous_pos.map(|p| p + 1).unwrap_or(0)..pos];

                if let Some(pos_before_whitespace) = message_to_newline.rfind_not_byteset(b"\t\n\x0C\r ") {
                    out.extend_from_slice(&message_to_newline[..pos_before_whitespace + 1]);
                }
                out.push_byte(b' ');
                previous_pos = Some(pos);
                match message.get(pos + 1..).and_then(|i| i.find_byte(b'\n')) {
                    Some(next_nl_pos) => pos += next_nl_pos + 1,
                    None => {
                        if let Some(slice) = message.get((pos + 1)..) {
                            out.extend_from_slice(slice);
                        }
                        break out.into();
                    }
                }
            }
        }
        None => message.as_bstr().into(),
    }
}

/// A reference to a message body, further parsed to only contain the non-trailer parts.
///
/// See [git-interpret-trailers](https://git-scm.com/docs/git-interpret-trailers) for more information
/// on what constitutes trailers and not that this implementation is only good for typical sign-off footer or key-value parsing.
pub struct BodyRef<'a> {
    _body_without_footer: &'a BStr,
    _start_of_footer: &'a [u8],
}

impl<'a> AsRef<BStr> for BodyRef<'a> {
    fn as_ref(&self) -> &BStr {
        self._body_without_footer
    }
}

impl<'a> Deref for BodyRef<'a> {
    type Target = BStr;

    fn deref(&self) -> &Self::Target {
        self._body_without_footer
    }
}
