use crate::file::{Section, SectionBody};
use crate::parse::section;
use crate::Source;
use bstr::BString;
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
    type Target = SectionBody<'a>;

    fn deref(&self) -> &Self::Target {
        &self.body
    }
}

impl<'a> Section<'a> {
    /// Return our header.
    pub fn header(&self) -> &section::Header<'a> {
        &self.header
    }

    /// Return our body, containing all keys and values.
    pub fn body(&self) -> &SectionBody<'a> {
        &self.body
    }

    /// Serialize this type into a `BString` for convenience.
    ///
    /// Note that `to_string()` can also be used, but might not be lossless.
    #[must_use]
    pub fn to_bstring(&self) -> BString {
        let mut buf = Vec::new();
        self.write_to(&mut buf).expect("io error impossible");
        buf.into()
    }

    /// Stream ourselves to the given `out`, in order to reproduce this section mostly losslessly
    /// as it was parsed.
    pub fn write_to(&self, mut out: impl std::io::Write) -> std::io::Result<()> {
        self.header.write_to(&mut out)?;
        for event in self.body.as_ref() {
            event.write_to(&mut out)?;
        }
        Ok(())
    }
}

pub(crate) mod body;
