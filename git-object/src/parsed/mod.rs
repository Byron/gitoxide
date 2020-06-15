use quick_error::quick_error;
use crate::{Time};
use std::str;
use bstr::BStr;

mod tag;
mod util;

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
        ObjectKind(err: crate::Error) {
            from()
            cause(err)
        }
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
