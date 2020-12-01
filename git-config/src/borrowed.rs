use crate::{file::File, owned};

pub struct Entry<'a> {
    pub(crate) parent: &'a File,
    pub(crate) index: usize,
}

impl<'a> Entry<'a> {
    pub fn to_editable(&self) -> owned::Entry {
        let entry = self.parent.token(self.index).as_entry().expect("entry");
        owned::Entry {
            name: self.parent.bytes_at(entry.name).into(),
            value: entry.value.map(|span| self.parent.bytes_at(span).into()),
            span: Some(entry.name),
        }
    }
}

pub struct Section<'a> {
    pub(crate) parent: &'a File,
    pub(crate) index: usize,
}

impl<'a> Section<'a> {
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
