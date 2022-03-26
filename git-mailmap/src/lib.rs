#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

use bstr::BStr;

pub mod parse;
pub fn parse(buf: &[u8]) -> parse::Lines<'_> {
    parse::Lines::new(buf)
}

mod entry;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Default)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry<'a> {
    #[cfg_attr(feature = "serde1", serde(borrow))]
    /// The name to map to.
    canonical_name: Option<&'a BStr>,
    /// The email map to.
    canonical_email: Option<&'a BStr>,
    /// The name to look for and replace.
    commit_name: Option<&'a BStr>,
    /// The email to look for and replace.
    commit_email: Option<&'a BStr>,
}
