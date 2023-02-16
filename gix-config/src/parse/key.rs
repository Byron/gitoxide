use bstr::{BStr, ByteSlice};

/// An unvalidated parse result of parsing input like `remote.origin.url` or `core.bare`.
#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Hash, Clone, Copy)]
pub struct Key<'a> {
    /// The name of the section, like `core` in `core.bare`.
    pub section_name: &'a str,
    /// The name of the sub-section, like `origin` in `remote.origin.url`.
    pub subsection_name: Option<&'a BStr>,
    /// The name of the section key, like `url` in `remote.origin.url`.
    pub value_name: &'a str,
}

/// Parse `input` like `core.bare` or `remote.origin.url` as a `Key` to make its fields available,
/// or `None` if there were not at least 2 tokens separated by `.`.
/// Note that `input` isn't validated, and is `str` as ascii is a subset of UTF-8 which is required for any valid keys.
pub fn parse_unvalidated<'a>(input: impl Into<&'a BStr>) -> Option<Key<'a>> {
    let input = input.into();
    let mut tokens = input.splitn(2, |b| *b == b'.');
    let section_name = tokens.next()?;
    let subsection_or_key = tokens.next()?;
    let mut tokens = subsection_or_key.rsplitn(2, |b| *b == b'.');
    let (subsection_name, value_name) = match (tokens.next(), tokens.next()) {
        (Some(key), Some(subsection)) => (Some(subsection.into()), key),
        (Some(key), None) => (None, key),
        (None, Some(_)) => unreachable!("iterator can't restart producing items"),
        (None, None) => return None,
    };

    Some(Key {
        section_name: section_name.to_str().ok()?,
        subsection_name,
        value_name: value_name.to_str().ok()?,
    })
}
