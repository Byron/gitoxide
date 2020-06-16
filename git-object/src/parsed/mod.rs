use crate::Time;
use bstr::BStr;
use quick_error::quick_error;
use std::str;

mod tag;
mod util;

use nom::error::ParseError;
pub use tag::Tag;

#[cfg(test)]
mod tests;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        ParseIntegerError(msg: &'static str, kind: Vec<u8>, err: btoi::ParseIntegerError) {
            display("{}: {:?}", msg, std::str::from_utf8(&kind))
            cause(err)
        }
        ParseError(msg: &'static str, kind: Vec<u8>) {
            display("{}: {:?}", msg, std::str::from_utf8(&kind))
        }
        Nom(input: bstr::BString, msg: &'static str) {
            display("{}: '{}' could not be parsed", msg, input)
        }
        ParseKindError(err: crate::types::Error) {
            display("{}", err)
            cause(err)
        }
        ObjectKind(err: crate::Error) {
            from()
            cause(err)
        }
    }
}

impl Error {
    fn set_parse_context(mut self, ctx: &'static str) -> Self {
        match self {
            Error::Nom(_, ref mut message) => *message = ctx,
            _ => {}
        };
        self
    }

    fn context(msg: &'static str) -> impl Fn(nom::Err<Self>) -> nom::Err<Self> {
        move |e: nom::Err<Self>| e.map(|e| e.set_parse_context(msg))
    }
}

impl ParseError<&[u8]> for Error {
    fn from_error_kind(input: &[u8], kind: nom::error::ErrorKind) -> Self {
        Error::Nom(input.into(), "parse error")
    }

    fn append(_: &[u8], _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum Object<'data> {
    Tag(Tag<'data>),
}

impl<'data> Object<'data> {
    pub fn kind(&self) -> crate::Kind {
        match self {
            Object::Tag(_) => crate::Kind::Tag,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Signature<'data> {
    pub name: &'data BStr,
    pub email: &'data BStr,
    pub time: Time,
}
