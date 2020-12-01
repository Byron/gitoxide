use crate::{borrowed, file::File, owned, Span};
use std::io;

/// Represents a possible edit to the git configuration file
enum Edit {
    Delete(Span), // section or entry
    SetSection(owned::Section),
    SetEntry(owned::Entry),
}

impl Into<Edit> for owned::Section {
    fn into(self) -> Edit {
        Edit::SetSection(self)
    }
}

impl Into<Edit> for owned::Entry {
    fn into(self) -> Edit {
        Edit::SetEntry(self)
    }
}

/// Collects edits to be applied to a [`File`], to be written out eventually.
pub struct Edits<'a> {
    parent: &'a File,
    edits: Vec<Edit>,
}

impl<'a> Edits<'a> {
    pub fn delete_section(&mut self, section: &borrowed::Section<'_>) -> &mut Self {
        self.edits.push(Edit::Delete(
            self.parent.token(section.index).as_section().expect("section").name,
        ));
        self
    }
    pub fn delete_entry(&mut self, entry: &borrowed::Entry<'_>) -> &mut Self {
        self.edits.push(Edit::Delete(
            self.parent.token(entry.index).as_entry().expect("entry").name,
        ));
        self
    }
    // Use with [`owned::Section`].
    //
    // Newly [instantiated][owned::Section::new()] sections will be appended, and existing ones can be edited
    // by calling [`borrowed::Section::to_editable()`].
    pub fn create_or_update_section(&mut self, section: owned::Section) -> &mut Self {
        self.edits.push(Edit::SetSection(section));
        self
    }
    pub fn create_or_update_entry(&mut self, entry: owned::Entry) -> &mut Self {
        self.edits.push(Edit::SetEntry(entry));
        self
    }

    pub fn to_write(&self, _out: impl io::Write) -> io::Result<()> {
        unimplemented!("to write")
    }
}

impl File {
    pub fn edit(&self) -> Edits {
        Edits {
            parent: self,
            edits: Vec::new(),
        }
    }
}
