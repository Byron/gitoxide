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

impl Into<io::Error> for Error {
    fn into(self) -> io::Error {
        io::Error::new(io::ErrorKind::Other, self)
    }
}

const NL: &[u8; 1] = b"\n";
const SPACE: &[u8; 1] = b" ";

pub fn trusted_header_field(name: &[u8], value: &[u8], mut out: impl io::Write) -> io::Result<()> {
    out.write_all(name)?;
    out.write_all(&SPACE[..])?;
    out.write_all(value)?;
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
