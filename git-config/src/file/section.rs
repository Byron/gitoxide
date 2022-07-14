use crate::file::Section;
use crate::Source;
use std::ops::Deref;
use std::path::PathBuf;

/// Additional information about a section.
#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct Metadata {
    /// The file path of the source, if known.
    pub path: Option<PathBuf>,
    /// Where the section is coming from.
    pub source: Source,
    /// The levels of indirection of the file, with 0 being a section
    /// that was directly loaded, and 1 being an `include.path` of a
    /// level 0 file.
    pub level: u8,
}

impl<'a> Deref for Section<'a> {
    type Target = crate::parse::Section<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Section<'_> {
    /// Return our meta data, additional information about this section.
    pub fn meta(&self) -> &Metadata {
        self.meta.as_ref()
    }
}
