use crate::Span;
use bstr::BString;

/// A key-value entry of a git-config file, like `name = value`
pub struct Entry {
    /// The name of the entry.
    pub name: BString,
    /// The entry's value as it would appear in the git config file.
    pub value: Option<BString>,
    pub(crate) span: Option<Span>,
}

/// A section or sub-section (in case `sub_name` is `Some()`), with all their entries.
///
/// For example
/// ```text
/// [hello]
/// a = 2
///
/// [hello.world]
/// b = c
/// x = y
/// ```
pub struct Section {
    /// The name of the section.
    pub name: BString,
    /// The name of the sub-section if present.
    pub sub_name: Option<BString>,
    /// The key value pairs within this section.
    pub entries: Vec<Entry>,
    pub(crate) span: Option<Span>,
}

impl Entry {
    /// Create a new entry with `name` and `value`.
    ///
    /// Note that it is unrelated to any file unless used with [edits][crate::file::Edits::create_or_update_entry()].
    pub fn new(name: BString, value: Option<BString>) -> Self {
        Entry {
            name,
            value,
            span: None,
        }
    }

    /// Chainable setter for the `name` of the entry.
    pub fn name(mut self, name: impl Into<BString>) -> Self {
        self.name = name.into();
        self
    }
    /// Chainable setter for the `value` of the entry.
    pub fn value(mut self, value: Option<BString>) -> Self {
        self.value = value;
        self
    }
}

impl Section {
    /// Create a new section with `name` and possibly a sub-section name, along with its entries.
    ///
    /// Note that it is unrelated to any file unless used with [edits][crate::file::Edits::create_or_update_section()].
    pub fn new(name: BString, sub_name: Option<BString>, entries: Vec<Entry>) -> Self {
        Section {
            name,
            sub_name,
            entries,
            span: None,
        }
    }

    /// Chainable setter for the `name` of the section.
    pub fn name(mut self, name: impl Into<BString>) -> Self {
        self.name = name.into();
        self
    }
    /// Chainable setter for the `sub_name` of the section, making it a sub-section.
    pub fn sub_name(mut self, sub_name: Option<BString>) -> Self {
        self.sub_name = sub_name;
        self
    }
}
