//! [Parse][parse()] .mailmap files as used in git repositories and remap names and emails
//! using an [accelerated data-structure][Snapshot].
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, rust_2018_idioms)]
#![forbid(unsafe_code)]

use bstr::BStr;

///
pub mod parse;

/// Parse the given `buf` of bytes line by line into mapping [Entries][Entry].
///
/// Errors may occur per line, but it's up to the caller to stop iteration when
/// one is encountered.
pub fn parse(buf: &[u8]) -> parse::Lines<'_> {
    parse::Lines::new(buf)
}

/// Similar to [parse()], but will skip all lines that didn't parse correctly, silently squelching all errors.
pub fn parse_ignore_errors(buf: &[u8]) -> impl Iterator<Item = Entry<'_>> {
    parse(buf).filter_map(Result::ok)
}

mod entry;

///
pub mod snapshot;

/// A data-structure to efficiently store a list of entries for optimal, case-insensitive lookup by email and
/// optionally name to find mappings to new names and/or emails.
///
/// The memory layout is efficient, even though lots of small allocations are performed to store strings of emails and names.
#[derive(Default, Clone)]
pub struct Snapshot {
    /// Sorted by `old_email`
    entries_by_old_email: Vec<snapshot::EmailEntry>,
}

/// An typical entry of a mailmap, which always contains an `old_email` by which
/// the mapping is performed to replace the given `new_name` and `new_email`.
///
/// Optionally, `old_name` is also used for lookup.
///
/// Typically created by [parse()].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    /// The name to map to.
    pub(crate) new_name: Option<&'a BStr>,
    /// The email map to.
    pub(crate) new_email: Option<&'a BStr>,
    /// The name to look for and replace.
    pub(crate) old_name: Option<&'a BStr>,
    /// The email to look for and replace.
    pub(crate) old_email: &'a BStr,
}
