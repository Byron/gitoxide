use crate::compound;
use std::{fs, io, path::PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Parse(#[from] parse::Error),
    #[error(transparent)]
    Init(#[from] compound::init::Error),
    #[error("Alternates form a cycle: {} -> {}", .0.iter().map(|p| format!("'{}'", p.display())).collect::<Vec<_>>().join(" -> "), .0.first().expect("more than one directories").display())]
    Cycle(Vec<PathBuf>),
}

pub mod unquote {
    use git_object::bstr::{BStr, BString, ByteSlice};
    use std::borrow::Cow;

    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("{message}: {:?}", String::from_utf8_lossy(&.input))]
        InvalidInput { message: &'static str, input: Vec<u8> },
        #[error("Unexpected end of input when fetching the next {0} bytes")]
        UnexpectedEndOfInput(usize),
        #[error("Invalid escaped value {byte} in input {:?}", String::from_utf8_lossy(&.input))]
        UnsupportedEscapeByte { byte: u8, input: Vec<u8> },
    }

    impl Error {
        fn new(message: &'static str, input: &BStr) -> Error {
            Error::InvalidInput {
                message,
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
    }
}

pub mod parse {
    use crate::alternate::unquote;
    use git_object::bstr::ByteSlice;
    use std::{borrow::Cow, path::PathBuf};

    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("Could not obtain an object path for the alternate directory '{}'", String::from_utf8_lossy(&.0))]
        PathConversion(Vec<u8>),
        #[error("Could not unquote alternate path")]
        Unquote(#[from] unquote::Error),
    }

    pub(crate) fn content(input: &[u8]) -> Result<Vec<PathBuf>, Error> {
        let mut out = Vec::new();
        for line in input.split(|b| *b == b'\n') {
            let line = line.as_bstr();
            if line.is_empty() || line.starts_with(b"#") {
                continue;
            }
            out.push(
                if line.starts_with(b"\"") {
                    unquote::ansi_c(line)?
                } else {
                    Cow::Borrowed(line)
                }
                .to_path()
                .map(ToOwned::to_owned)
                .map_err(|_| Error::PathConversion(line.to_vec()))?,
            )
        }
        Ok(out)
    }
}

pub fn resolve(objects_directory: impl Into<PathBuf>) -> Result<Vec<compound::Db>, Error> {
    let relative_base = objects_directory.into();
    let mut dirs = vec![(0, relative_base.clone())];
    let mut out = Vec::new();
    let mut seen = vec![relative_base.canonicalize()?];
    while let Some((depth, dir)) = dirs.pop() {
        match fs::read(dir.join("info").join("alternates")) {
            Ok(input) => {
                for path in parse::content(&input)?.into_iter() {
                    let path = relative_base.join(path);
                    let path_canonicalized = path.canonicalize()?;
                    if seen.contains(&path_canonicalized) {
                        continue;
                    }
                    seen.push(path_canonicalized);
                    dirs.push((depth + 1, path));
                }
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                // Only resolve for repositories with at least one link, otherwise the line below causes infinite recursion
                if depth != 0 {
                    // The tail of a chain doesn't have alternates, and thus is the real deal
                    out.push(compound::Db::at(dir)?);
                }
            }
            Err(err) => return Err(err.into()),
        };
    }

    if out.is_empty() && seen.len() > 1 {
        return Err(Error::Cycle(seen));
    }
    Ok(out)
}
