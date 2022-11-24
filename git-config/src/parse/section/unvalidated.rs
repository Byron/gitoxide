use bstr::{BStr, ByteSlice};

/// An unvalidated parse result of a key for a section, parsing input like `remote.origin` or `core`.
#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Hash, Clone, Copy)]
pub struct Key<'a> {
    /// The name of the section, like `remote` in `remote.origin`.
    pub section_name: &'a str,
    /// The name of the sub-section, like `origin` in `remote.origin`.
    pub subsection_name: Option<&'a BStr>,
}

impl<'a> Key<'a> {
    /// Parse `input` like `remote.origin` or `core` as a `Key` to make its section specific fields available,
    /// or `None` if there were not one or two tokens separated by `.`.
    /// Note that `input` isn't validated, and is `str` as ascii is a subset of UTF-8 which is required for any valid keys.
    pub fn parse(input: impl Into<&'a BStr>) -> Option<Self> {
        let input = input.into();
        let mut tokens = input.splitn(2, |b| *b == b'.');

        Some(Key {
            section_name: tokens.next()?.to_str().ok()?,
            subsection_name: tokens.next().map(Into::into),
        })
    }
}
