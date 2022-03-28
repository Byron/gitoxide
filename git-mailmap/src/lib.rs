#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

use bstr::BStr;

pub mod parse;
pub fn parse(buf: &[u8]) -> parse::Lines<'_> {
    parse::Lines::new(buf)
}

pub fn parse_ignore_errors(buf: &[u8]) -> impl Iterator<Item = Entry<'_>> {
    parse(buf).filter_map(Result::ok)
}

mod entry;

mod snapshot;

#[derive(Default, Clone)]
pub struct Snapshot {
    /// Sorted by `old_email`
    entries_by_old_email: Vec<snapshot::EmailEntry>,
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy, Default)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry<'a> {
    #[cfg_attr(feature = "serde1", serde(borrow))]
    /// The name to map to.
    new_name: Option<&'a BStr>,
    /// The email map to.
    new_email: Option<&'a BStr>,
    /// The name to look for and replace.
    old_name: Option<&'a BStr>,
    /// The email to look for and replace.
    old_email: &'a BStr,
}
