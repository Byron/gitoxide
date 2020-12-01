use crate::Span;
use bstr::BString;

/// A key-value entry of a git-config file, like `name = value`
pub struct Entry {
    pub name: BString,
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
    pub name: BString,
    pub sub_name: Option<BString>,
    pub entries: Vec<Entry>,
    pub(crate) span: Option<Span>,
}

impl Entry {
    pub fn new(name: BString, value: Option<BString>) -> Self {
        Entry {
            name,
            value,
            span: None,
        }
    }

    pub fn name(mut self, name: impl Into<BString>) -> Self {
        self.name = name.into();
        self
    }
    pub fn value(mut self, name: Option<BString>) -> Self {
        self.value = name;
        self
    }
}

impl Section {
    pub fn new(name: BString, sub_name: Option<BString>, entries: Vec<Entry>) -> Self {
        Section {
            name,
            sub_name,
            entries,
            span: None,
        }
    }

    pub fn name(mut self, name: impl Into<BString>) -> Self {
        self.name = name.into();
        self
    }
    pub fn sub_name(mut self, sub_name: Option<BString>) -> Self {
        self.sub_name = sub_name;
        self
    }
}
