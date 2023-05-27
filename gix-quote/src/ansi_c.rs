///
pub mod undo {
    use bstr::{BStr, BString};

    /// The error returned by [`ansi_c`][crate::ansi_c::undo()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("{message}: {input:?}")]
        InvalidInput { message: String, input: BString },
        #[error("Invalid escaped value {byte} in input {input:?}")]
        UnsupportedEscapeByte { byte: u8, input: BString },
    }

    impl Error {
        pub(crate) fn new(message: impl ToString, input: &BStr) -> Error {
            Error::InvalidInput {
                message: message.to_string(),
                input: input.into(),
            }
        }
    }
}

use std::{borrow::Cow, io::Read};

use bstr::{BStr, BString, ByteSlice};

/// Unquote the given ansi-c quoted `input` string, returning it and all of the consumed bytes.
///
/// The `input` is returned unaltered if it doesn't start with a `"` character to indicate
/// quotation, otherwise a new unquoted string will always be allocated.
/// The amount of consumed bytes allow to pass strings that start with a quote, and skip all quoted text for additional processing
///
/// See [the tests][tests] for quotation examples.
///
/// [tests]: https://github.com/Byron/gitoxide/blob/e355b4ad133075152312816816af5ce72cf79cff/gix-odb/src/alternate/unquote.rs#L110-L118
pub fn undo(input: &BStr) -> Result<(Cow<'_, BStr>, usize), undo::Error> {
    if !input.starts_with(b"\"") {
        return Ok((input.into(), input.len()));
    }
    if input.len() < 2 {
        return Err(undo::Error::new("Input must be surrounded by double quotes", input));
    }
    let original = input.as_bstr();
    let mut input = &input[1..];
    let mut consumed = 1;
    let mut out = BString::default();
    fn consume_one_past(input: &mut &BStr, position: usize) -> Result<u8, undo::Error> {
        *input = input
            .get(position + 1..)
            .ok_or_else(|| undo::Error::new("Unexpected end of input", input))?
            .as_bstr();
        let next = input[0];
        *input = input.get(1..).unwrap_or_default().as_bstr();
        Ok(next)
    }
    loop {
        match input.find_byteset(b"\"\\") {
            Some(position) => {
                out.extend_from_slice(&input[..position]);
                consumed += position + 1;
                match input[position] {
                    b'"' => break,
                    b'\\' => {
                        let next = consume_one_past(&mut input, position)?;
                        consumed += 1;
                        match next {
                            b'n' => out.push(b'\n'),
                            b'r' => out.push(b'\r'),
                            b't' => out.push(b'\t'),
                            b'a' => out.push(7),
                            b'b' => out.push(8),
                            b'v' => out.push(0xb),
                            b'f' => out.push(0xc),
                            b'"' => out.push(b'"'),
                            b'\\' => out.push(b'\\'),
                            b'0' | b'1' | b'2' | b'3' => {
                                let mut buf = [next; 3];
                                input
                                    .get(..2)
                                    .ok_or_else(|| {
                                        undo::Error::new(
                                            "Unexpected end of input when fetching two more octal bytes",
                                            input,
                                        )
                                    })?
                                    .read_exact(&mut buf[1..])
                                    .expect("impossible to fail as numbers match");
                                let byte = btoi::btou_radix(&buf, 8).map_err(|e| undo::Error::new(e, original))?;
                                out.push(byte);
                                input = &input[2..];
                                consumed += 2;
                            }
                            _ => {
                                return Err(undo::Error::UnsupportedEscapeByte {
                                    byte: next,
                                    input: original.into(),
                                })
                            }
                        }
                    }
                    _ => unreachable!("cannot find character that we didn't search for"),
                }
            }
            None => {
                out.extend_from_slice(input);
                consumed += input.len();
                break;
            }
        }
    }
    Ok((out.into(), consumed))
}
