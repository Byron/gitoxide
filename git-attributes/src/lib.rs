#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

use bstr::BStr;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum State<'a> {
    /// The attribute is listed, or has the special value 'true'
    Set,
    /// The attribute has the special value 'false', or was prefixed with a `-` sign.
    Unset,
    /// The attribute is set to the given value, which followed the `=` sign.
    /// Note that values can be empty.
    #[cfg_attr(feature = "serde1", serde(borrow))]
    Value(&'a BStr),
    /// The attribute isn't mentioned with a given path or is explicitly set to `Unspecified` using the `!` sign.
    Unspecified,
}

pub mod parse;

pub fn parse(buf: &[u8]) -> parse::Lines<'_> {
    parse::Lines::new(buf)
}
