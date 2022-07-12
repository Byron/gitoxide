use std::{convert::TryFrom, fmt::Display};

use bstr::{BStr, BString};

use crate::{file::SectionBody, parse, File};

impl<'a> TryFrom<&'a str> for File<'a> {
    type Error = parse::Error;

    /// Convenience constructor. Attempts to parse the provided string into a
    /// [`File`]. See [`Events::from_str()`][crate::parse::Events::from_str()] for more information.
    fn try_from(s: &'a str) -> Result<File<'a>, Self::Error> {
        parse::Events::from_str(s).map(Self::from)
    }
}

impl<'a> TryFrom<&'a BStr> for File<'a> {
    type Error = parse::Error;

    /// Convenience constructor. Attempts to parse the provided byte string into
    /// a [`File`]. See [`Events::from_bytes()`][parse::Events::from_bytes()] for more information.
    fn try_from(value: &'a BStr) -> Result<File<'a>, Self::Error> {
        parse::Events::from_bytes(value).map(File::from)
    }
}

impl<'a> From<parse::Events<'a>> for File<'a> {
    fn from(events: parse::Events<'a>) -> Self {
        let mut this = File {
            frontmatter_events: events.frontmatter,
            ..Default::default()
        };

        for section in events.sections {
            this.push_section_internal(section.section_header, SectionBody(section.events));
        }

        this
    }
}

impl From<File<'_>> for BString {
    fn from(c: File<'_>) -> Self {
        c.into()
    }
}

impl Display for File<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_bstring(), f)
    }
}
