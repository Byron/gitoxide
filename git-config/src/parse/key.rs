/// An unvalidated parse result of parsing input like `remote.origin.url` or `core.bare`.
#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Hash, Clone, Copy)]
pub struct Key<'a> {
    /// The name of the section, like `core` in `core.bare`.
    pub section_name: &'a str,
    /// The name of the sub-section, like `origin` in `remote.origin.url`.
    pub subsection_name: Option<&'a str>,
    /// The name of the section key, like `url` in `remote.origin.url`.
    pub value_name: &'a str,
}

/// Parse `input` like `core.bare` or `remote.origin.url` as a `Key` to make its fields available,
/// or `None` if there were not at least 2 tokens separated by `.`.
/// Note that `input` isn't validated, and is `str` as ascii is a subset of UTF-8 which is required for any valid keys.
pub fn parse_unvalidated(input: &str) -> Option<Key<'_>> {
    let (section_name, subsection_or_key) = input.split_once('.')?;
    let (subsection_name, value_name) = match subsection_or_key.rsplit_once('.') {
        Some((subsection, key)) => (Some(subsection), key),
        None => (None, subsection_or_key),
    };

    Some(Key {
        section_name,
        subsection_name,
        value_name,
    })
}
