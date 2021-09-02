use bstr::{BStr, BString, ByteSlice, ByteVec};
use std::borrow::Cow;

/// An empty array of a type usable with the `git::easy` API to help declaring no parents should be used
pub const NO_PARENT_IDS: [git_hash::ObjectId; 0] = [];

/// Produce a short commit summary for the given `message`.
///
/// This means the following
///
/// * Take the subject line which is delimited by two newlines (\n\n)
/// * transform intermediate consecutive whitespace including \r into one space
///
/// The resulting summary will have folded whitespace before a newline into spaces and stopped that process
/// once two consecutive newlines are encountered.
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
