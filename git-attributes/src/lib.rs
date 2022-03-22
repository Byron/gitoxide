#![forbid(unsafe_code, rust_2018_idioms)]

use bstr::BStr;

pub enum State<'a> {
    /// The attribute is listed, or has the special value 'true'
    Set,
    /// The attribute has the special value 'false', or was prefixed with a `-` sign.
    Unset,
    /// The attribute is set to the given value, which followed the `=` sign.
    /// Note that values can be empty.
    Value(&'a BStr),
    /// The attribute isn't mentioned with a given path or is explicitly set to `Unspecified` using the `!` sign.
    Unspecified,
}

pub mod ignore;

pub mod parse;

pub fn parse(buf: &[u8]) -> parse::attribute::Lines<'_> {
    parse::attribute::Lines::new(buf)
}
