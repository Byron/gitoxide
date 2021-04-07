use crate::owned::{self, NL, SPACE};
use bstr::{BString, ByteSlice};
use quick_error::quick_error;
use std::io;

quick_error! {
    #[derive(Debug)]
    enum Error {
        NewlineInHeaderValue(value: BString) {
            display("Newlines are not allowed in header values: {:?}", value)
        }
        EmptyValue {
            display("Header values must not be empty")
        }
    }
}

impl From<Error> for io::Error {
    fn from(other: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, other)
    }
}

pub fn header_field_multi_line(name: &[u8], value: &[u8], mut out: impl io::Write) -> io::Result<()> {
    let mut lines = value.as_bstr().lines();
    trusted_header_field(name, lines.next().expect("non-empty value"), &mut out)?;
    for line in lines {
        out.write_all(SPACE)?;
        out.write_all(line)?;
        out.write_all(NL)?;
    }
    // If the last line ended on a newline, we have to re-add it by force
    if let Some(b) = value.last() {
        if *b == b'\n' {
            out.write_all(SPACE)?;
            out.write_all(NL)?;
        }
    }
    Ok(())
}

pub fn trusted_header_field(name: &[u8], value: &[u8], mut out: impl io::Write) -> io::Result<()> {
    out.write_all(name)?;
    out.write_all(SPACE)?;
    out.write_all(value)?;
    out.write_all(NL)
}

pub fn trusted_header_signature(name: &[u8], value: &owned::Signature, mut out: impl io::Write) -> io::Result<()> {
    out.write_all(name)?;
    out.write_all(SPACE)?;
    value.write_to(&mut out)?;
    out.write_all(NL)
}

pub fn trusted_header_id(name: &[u8], value: &git_hash::ObjectId, mut out: impl io::Write) -> io::Result<()> {
    out.write_all(name)?;
    out.write_all(&SPACE[..])?;
    value.write_hex_to(&mut out)?;
    out.write_all(&NL[..])
}

pub fn header_field(name: &[u8], value: &[u8], out: impl io::Write) -> io::Result<()> {
    if value.is_empty() {
        return Err(Error::EmptyValue.into());
    }
    if value.find(NL).is_some() {
        return Err(Error::NewlineInHeaderValue(value.into()).into());
    }
    trusted_header_field(name, value, out)
}
