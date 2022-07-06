use bstr::{BString, ByteVec};
use std::{convert::TryFrom, fmt::Display};

use crate::{
    file::SectionBody,
    parse::{parse_from_bytes, parse_from_str, Error, Event, Parser},
    File,
};

impl<'a> TryFrom<&'a str> for File<'a> {
    type Error = Error<'a>;

    /// Convenience constructor. Attempts to parse the provided string into a
    /// [`File`]. See [`parse_from_str`] for more information.
    ///
    /// [`parse_from_str`]: crate::parse::parse_from_str
    fn try_from(s: &'a str) -> Result<File<'a>, Self::Error> {
        parse_from_str(s).map(Self::from)
    }
}

impl<'a> TryFrom<&'a [u8]> for File<'a> {
    type Error = Error<'a>;

    /// Convenience constructor. Attempts to parse the provided byte string into
    //// a [`File`]. See [`parse_from_bytes`] for more information.
    ///
    /// [`parse_from_bytes`]: crate::parser::parse_from_bytes
    fn try_from(value: &'a [u8]) -> Result<File<'a>, Self::Error> {
        parse_from_bytes(value).map(File::from)
    }
}

impl<'a> TryFrom<&'a BString> for File<'a> {
    type Error = Error<'a>;

    /// Convenience constructor. Attempts to parse the provided byte string into
    //// a [`File`]. See [`parse_from_bytes`] for more information.
    ///
    /// [`parse_from_bytes`]: crate::parser::parse_from_bytes
    fn try_from(value: &'a BString) -> Result<File<'a>, Self::Error> {
        parse_from_bytes(value.as_ref()).map(File::from)
    }
}

impl<'a> From<Parser<'a>> for File<'a> {
    fn from(parser: Parser<'a>) -> Self {
        let mut new_self = Self::default();

        // Current section that we're building
        let mut prev_section_header = None;
        let mut section_events = SectionBody::new();

        #[allow(clippy::explicit_into_iter_loop)]
        // it's not really an iterator (yet), needs streaming iterator support
        for event in parser.into_iter() {
            match event {
                Event::SectionHeader(header) => {
                    if let Some(prev_header) = prev_section_header.take() {
                        new_self.push_section_internal(prev_header, section_events);
                    } else {
                        new_self.frontmatter_events = section_events;
                    }
                    prev_section_header = Some(header);
                    section_events = SectionBody::new();
                }
                e @ Event::Key(_)
                | e @ Event::Value(_)
                | e @ Event::ValueNotDone(_)
                | e @ Event::ValueDone(_)
                | e @ Event::KeyValueSeparator => section_events.as_mut().push(e),
                e @ Event::Comment(_) | e @ Event::Newline(_) | e @ Event::Whitespace(_) => {
                    section_events.as_mut().push(e);
                }
            }
        }

        // The last section doesn't get pushed since we only push if there's a
        // new section header, so we need to call push one more time.
        if let Some(header) = prev_section_header {
            new_self.push_section_internal(header, section_events);
        } else {
            new_self.frontmatter_events = section_events;
        }

        new_self
    }
}

impl From<File<'_>> for BString {
    fn from(c: File<'_>) -> Self {
        c.into()
    }
}

impl From<&File<'_>> for BString {
    fn from(config: &File<'_>) -> Self {
        let mut value = BString::default();

        for events in config.frontmatter_events.as_ref() {
            value.push_str(events.to_bstring());
        }

        for section_id in &config.section_order {
            value.push_str(
                config
                    .section_headers
                    .get(section_id)
                    .expect("section_header does not contain section id from section_order")
                    .to_bstring(),
            );

            for event in config
                .sections
                .get(section_id)
                .expect("sections does not contain section id from section_order")
                .as_ref()
            {
                value.push_str(event.to_bstring());
            }
        }

        value
    }
}

impl Display for File<'_> {
    /// Note that this is a best-effort attempt at printing a `GitConfig`. If
    /// there are non UTF-8 values in your config, this will _NOT_ render as
    /// read.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for front_matter in self.frontmatter_events.as_ref() {
            front_matter.fmt(f)?;
        }

        for section_id in &self.section_order {
            self.section_headers.get(section_id).unwrap().fmt(f)?;
            for event in self.sections.get(section_id).unwrap().as_ref() {
                event.fmt(f)?;
            }
        }

        Ok(())
    }
}
