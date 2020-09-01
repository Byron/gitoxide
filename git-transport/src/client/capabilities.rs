use crate::Protocol;
use bstr::{BStr, BString, ByteSlice};
use quick_error::quick_error;
use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        MissingDelimitingNullByte {
            display("Capabilities were missing entirely as there was no 0 byte")
        }
        NoCapabilities {
            display("there was not a single capability behind the delimiter")
        }
        MissingVersionLine {
            display("a version line was expected, but none was retrieved")
        }
        MalformattedVersionLine(actual: String) {
            display("expected 'version X', got '{}'", actual)
        }
        UnsupportedVersion(wanted: Protocol, got: String) {
            display("Got unsupported version '{}', expected '{}'", got, *wanted as usize)
        }
        Io(err: io::Error) {
            display("An IO error occurred while reading V2 lines")
            from()
            source(err)
        }
    }
}

#[derive(Clone)]
pub struct Capabilities {
    data: BString,
    value_sep: u8,
}
pub struct Capability<'a>(&'a BStr);

impl<'a> Capability<'a> {
    pub fn name(&self) -> &BStr {
        self.0
            .splitn(2, |b| *b == b'=')
            .next()
            .expect("there is always a single item")
            .as_bstr()
    }
    pub fn value(&self) -> Option<&BStr> {
        self.0.splitn(2, |b| *b == b'=').nth(1).map(|s| s.as_bstr())
    }
    pub fn values(&self) -> Option<impl Iterator<Item = &BStr>> {
        self.value().map(|v| v.split(|b| *b == b' ').map(|s| s.as_bstr()))
    }
}

impl Capabilities {
    pub fn from_bytes(bytes: &[u8]) -> Result<(Capabilities, usize), Error> {
        let delimiter_pos = bytes.find_byte(0).ok_or(Error::MissingDelimitingNullByte)?;
        if delimiter_pos + 1 == bytes.len() {
            return Err(Error::NoCapabilities);
        }
        let capabilities = &bytes[delimiter_pos + 1..];
        Ok((
            Capabilities {
                data: capabilities.as_bstr().to_owned(),
                value_sep: b' ',
            },
            delimiter_pos,
        ))
    }
    pub fn from_lines(read: impl io::BufRead) -> Result<Capabilities, Error> {
        let mut lines = read.lines();
        let version_line = lines.next().ok_or(Error::MissingVersionLine)??;
        let mut version_tokens = version_line.splitn(2, |b| b == ' ');
        match (version_tokens.next(), version_tokens.next()) {
            (Some(name), Some(value)) => {
                if name != "version" {
                    return Err(Error::MalformattedVersionLine(version_line));
                }
                if value != "2" {
                    return Err(Error::UnsupportedVersion(Protocol::V2, value.to_owned()));
                }
            }
            _ => return Err(Error::MalformattedVersionLine(version_line)),
        };
        Ok(Capabilities {
            value_sep: b'\n',
            data: lines
                .inspect(|l| {
                    if let Ok(l) = l {
                        assert!(
                            !l.contains('\n'),
                            "newlines are not expected in keys or values, got '{}'",
                            l
                        )
                    }
                })
                .collect::<Result<Vec<_>, _>>()?
                .join("\n")
                .into(),
        })
    }
    pub fn iter(&self) -> impl Iterator<Item = Capability> {
        self.data
            .split(move |b| *b == self.value_sep)
            .map(|c| Capability(c.as_bstr()))
    }
}
