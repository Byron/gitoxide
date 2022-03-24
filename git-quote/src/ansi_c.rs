pub mod undo {
    use bstr::{BStr, BString};
    use quick_error::quick_error;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            InvalidInput { message: String, input: BString } {
                display("{}: {:?}", message, input)
            }
            UnsupportedEscapeByte { byte: u8, input: BString } {
                display("Invalid escaped value {} in input {:?}", byte, input)
            }
        }
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

/// Unquote the given ansi-c quoted `input` string.
///
/// The `input` is returned unaltered if it doesn't start with a `"` character to indicate
/// quotation, otherwise a new unqoted string will always be allocated.
///
/// See [the tests][tests] for quotation examples.
///
/// [tests]: https://github.com/Byron/gitoxide/blob/e355b4ad133075152312816816af5ce72cf79cff/git-odb/src/alternate/unquote.rs#L110-L118
pub fn undo(input: &BStr) -> Result<Cow<'_, BStr>, undo::Error> {
    if !input.starts_with(b"\"") {
        return Ok(input.into());
    }
    if input.len() < 2 {
        return Err(undo::Error::new("Input must be surrounded by double quotes", input));
    }
    let original = input.as_bstr();
    let mut input = &input[1..];
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
                match input[position] {
                    b'"' => break,
                    b'\\' => {
                        let next = consume_one_past(&mut input, position)?;
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
                break;
            }
        }
    }
    Ok(out.into())
}
