use git_object::bstr::{BStr, BString, ByteSlice};
use std::borrow::Cow;
use std::io::Read;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{message}: {:?}", String::from_utf8_lossy(&.input))]
    InvalidInput { message: String, input: Vec<u8> },
    #[error("Invalid escaped value {byte} in input {:?}", String::from_utf8_lossy(&.input))]
    UnsupportedEscapeByte { byte: u8, input: Vec<u8> },
}

impl Error {
    fn new(message: impl ToString, input: &BStr) -> Error {
        Error::InvalidInput {
            message: message.to_string(),
            input: input.to_vec(),
        }
    }
}

pub fn ansi_c(input: &BStr) -> Result<Cow<'_, BStr>, Error> {
    if !input.starts_with(b"\"") {
        return Ok(input.into());
    }
    if input.len() < 2 {
        return Err(Error::new("Input must be surrounded by double quotes", input));
    }
    let original = input.as_bstr();
    let mut input = &input[1..];
    let mut out = BString::default();
    fn consume_one_past(input: &mut &BStr, position: usize) -> Result<u8, Error> {
        *input = input
            .get(position + 1..)
            .ok_or_else(|| Error::new("Unexpected end of input", input))?
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
                                &input
                                    .get(..2)
                                    .ok_or_else(|| {
                                        Error::new("Unexpected end of input when fetching two more octal bytes", input)
                                    })?
                                    .read(&mut buf[1..])
                                    .expect("impossible to fail as numbers match");
                                let byte = btoi::btou_radix(&buf, 8).map_err(|e| Error::new(e, original))?;
                                out.push(byte);
                                input = &input[2..];
                            }
                            _ => {
                                return Err(Error::UnsupportedEscapeByte {
                                    byte: next,
                                    input: original.to_vec(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use git_object::bstr::ByteSlice;

    macro_rules! test {
        ($name:ident, $input:literal, $expected:literal) => {
            #[test]
            fn $name() {
                assert_eq!(
                    ansi_c($input.as_bytes().as_bstr()).expect("valid input"),
                    std::borrow::Cow::Borrowed($expected.as_bytes().as_bstr())
                );
            }
        };
    }

    test!(unquoted_remains_unchanged, "hello", "hello");
    test!(empty_surrounded_by_quotes, "\"\"", "");
    test!(surrounded_only_by_quotes, "\"hello\"", "hello");
    test!(typical_escapes, r#""\n\r\t""#, b"\n\r\t");
    test!(untypical_escapes, r#""\a\b\f\v""#, b"\x07\x08\x0c\x0b");
    test!(literal_escape_and_double_quote, r#""\"\\""#, br#""\"#);
    test!(
        unicode_byte_escapes_by_number,
        r#""\346\277\261\351\207\216\t\347\264\224""#,
        "濱野\t純"
    );
}
