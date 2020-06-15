use crate::{object, Time};
use std::str;

mod tag;
mod util;
pub use tag::Tag;
use bstr::BStr;

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
        ObjectKind(err: object::Error) {
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
    pub fn kind(&self) -> object::Kind {
        match self {
            Object::Tag(_) => object::Kind::Tag,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Signature<'data> {
    pub name: &'data BStr,
    pub email: &'data BStr,
    pub time: Time,
}
