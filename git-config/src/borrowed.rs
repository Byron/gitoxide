use crate::{file::File, owned};

/// A read-only key value pair within a git config file.
pub struct Entry<'a> {
    pub(crate) parent: &'a File,
    pub(crate) index: usize,
}

impl<'a> Entry<'a> {
    /// Convert this instance into an owned version to allow changes.
    pub fn to_editable(&self) -> owned::Entry {
        let entry = self.parent.token(self.index).as_entry().expect("entry");
        owned::Entry {
            name: self.parent.bytes_at(entry.name).into(),
            value: entry.value.map(|span| self.parent.bytes_at(span).into()),
            span: Some(entry.name),
        }
    }
}

/// A read-only section within a git config file.
pub struct Section<'a> {
    pub(crate) parent: &'a File,
    pub(crate) index: usize,
}

impl<'a> Section<'a> {
    /// Convert this instance into an owned version to allow changes.
    pub fn to_editable(&self) -> owned::Section {
        let section = self.parent.token(self.index).as_section().expect("section");
        owned::Section {
            name: self.parent.bytes_at(section.name).into(),
            sub_name: section.sub_name.map(|span| self.parent.bytes_at(span).into()),
            span: Some(section.name),
            entries: self.entries().map(|e| e.to_editable()).collect(),
        }
    }
}
