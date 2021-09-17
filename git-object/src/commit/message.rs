use crate::bstr::{BStr, BString, ByteSlice, ByteVec};
use crate::commit::MessageRef;
use std::borrow::Cow;

mod decode {
    use crate::bstr::BStr;
    use nom::error::ParseError;
    use nom::IResult;

    /// Returns title and body, without separator
    pub fn bytes<'a, E: ParseError<&'a [u8]>>(_i: &'a [u8]) -> IResult<&[u8], (&'a BStr, &'a BStr), E> {
        todo!("actual decoding")
    }
}

impl<'a> MessageRef<'a> {
    /// Parse the given `input` as message.
    ///
    /// Note that this cannot fail as everything will be interpreted as title if there is no body separator.
    pub fn from_bytes(input: &'a [u8]) -> Self {
        let (rest, (title, body)) = decode::bytes::<()>(input)
            .map(|(i, (title, body))| (i, (title, Some(body))))
            .unwrap_or_else(|_| (&[], (input.as_bstr(), None)));
        debug_assert!(rest.is_empty(), "all consuming message parsing");
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
        let message = self.title.trim();
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
}
