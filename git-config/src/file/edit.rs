use crate::{borrowed, file::File, owned};
use dangerous::Span;
use std::io;

/// Represents a possible edit to the git configuration file
#[must_use = "An edit must be added to an `Edits` instance to be effective"]
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
    /// Delete the given section as obtained by the [sections iterator][File::sections()].
    pub fn delete_section(&mut self, section: &borrowed::Section<'_>) -> &mut Self {
        self.edits.push(Edit::Delete(
            self.parent.token(section.index).as_section().expect("section").name,
        ));
        self
    }
    /// Delete the given entry as obtained by the [entries iterator][borrowed::Section::entries()].
    pub fn delete_entry(&mut self, entry: &borrowed::Entry<'_>) -> &mut Self {
        self.edits.push(Edit::Delete(
            self.parent.token(entry.index).as_entry().expect("entry").name,
        ));
        self
    }
    /// Create or update the given `section`.
    //
    /// Newly [instantiated][owned::Section::new()] sections will be appended, and existing ones can be edited
    /// by calling [`borrowed::Section::to_editable()`].
    pub fn create_or_update_section(&mut self, section: owned::Section) -> &mut Self {
        self.edits.push(Edit::SetSection(section));
        self
    }

    /// Create or update the given `entry`.
    //
    /// Newly [instantiated][owned::Entry::new()] sections will be appended, and existing ones can be edited
    /// by calling [`borrowed::Entry::to_editable()`].
    pub fn create_or_update_entry(&mut self, entry: owned::Entry) -> &mut Self {
        self.edits.push(Edit::SetEntry(entry));
        self
    }

    /// Assure new values are matching [these rules](https://github.com/git/git/blob/66e871b6647ffea61a77a0f82c7ef3415f1ee79c/Documentation/config.txt#L17:L20)
    pub fn to_write(&self, _out: impl io::Write) -> io::Result<()> {
        unimplemented!("to write")
    }
}

impl File {
    /// Return an empty collection of edits to be applied none-destructively to the parent file.
    #[must_use = "Edits must be written using the `write_to(…)` method to be effective."]
    pub fn edit(&self) -> Edits<'_> {
        Edits {
            parent: self,
            edits: Vec::new(),
        }
    }
}
