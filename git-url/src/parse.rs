use crate::borrowed;
use nom::error::ParseError;
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        NomDetail(input: bstr::BString, msg: &'static str) {
            display("{}: '{}' could not be parsed", msg, input)
        }
    }
}

impl Error {
    fn set_parse_context(mut self, ctx: &'static str) -> Self {
        if let Error::NomDetail(_, ref mut message) = self {
            *message = ctx
        }
        self
    }

    pub(crate) fn context(msg: &'static str) -> impl Fn(nom::Err<Self>) -> nom::Err<Self> {
        move |e: nom::Err<Self>| e.map(|e| e.set_parse_context(msg))
    }
}

impl ParseError<&[u8]> for Error {
    fn from_error_kind(input: &[u8], _kind: nom::error::ErrorKind) -> Self {
        Error::NomDetail(input.into(), "parse error")
    }

    fn append(_: &[u8], _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

pub fn parse(_url: &[u8]) -> Result<borrowed::Url, Error> {
    unimplemented!("parse")
}
