use crate::parser::{Event, ParsedSection, ParsedSectionHeader};
use bstr::BString;
use std::borrow::Cow;
use std::fmt::Display;

impl ParsedSection<'_> {
    /// Coerces into an owned instance. This differs from the standard [`clone`]
    /// implementation as calling clone will _not_ copy the borrowed variant,
    /// while this method will. In other words:
    ///
    /// | Borrow type | `.clone()` | `to_owned()` |
    /// | ----------- | ---------- | ------------ |
    /// | Borrowed    | Borrowed   | Owned        |
    /// | Owned       | Owned      | Owned        |
    ///
    /// This can be most effectively seen by the differing lifetimes between the
    /// two. This method guarantees a `'static` lifetime, while `clone` does
    /// not.
    ///
    /// [`clone`]: Self::clone
    #[must_use]
    pub fn to_owned(&self) -> ParsedSection<'static> {
        ParsedSection {
            section_header: self.section_header.to_owned(),
            events: self.events.iter().map(Event::to_owned).collect(),
        }
    }
}

impl Display for ParsedSection<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.section_header)?;
        for event in &self.events {
            event.fmt(f)?;
        }
        Ok(())
    }
}

impl ParsedSectionHeader<'_> {
    /// Generates a byte representation of the value. This should be used when
    /// non-UTF-8 sequences are present or a UTF-8 representation can't be
    /// guaranteed.
    #[must_use]
    pub fn to_bstring(&self) -> BString {
        self.into()
    }

    /// Coerces into an owned instance. This differs from the standard [`clone`]
    /// implementation as calling clone will _not_ copy the borrowed variant,
    /// while this method will. In other words:
    ///
    /// | Borrow type | `.clone()` | `to_owned()` |
    /// | ----------- | ---------- | ------------ |
    /// | Borrowed    | Borrowed   | Owned        |
    /// | Owned       | Owned      | Owned        |
    ///
    /// This can be most effectively seen by the differing lifetimes between the
    /// two. This method guarantees a `'static` lifetime, while `clone` does
    /// not.
    ///
    /// [`clone`]: Self::clone
    #[must_use]
    pub fn to_owned(&self) -> ParsedSectionHeader<'static> {
        ParsedSectionHeader {
            name: self.name.to_owned(),
            separator: self.separator.clone().map(|v| Cow::Owned(v.into_owned())),
            subsection_name: self.subsection_name.clone().map(|v| Cow::Owned(v.into_owned())),
        }
    }
}

impl Display for ParsedSectionHeader<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}", self.name)?;

        if let Some(v) = &self.separator {
            // Separator must be utf-8
            v.fmt(f)?;
            let subsection_name = self.subsection_name.as_ref().unwrap();
            if v.as_ref() == "." {
                subsection_name.fmt(f)?;
            } else {
                write!(f, "\"{}\"", subsection_name)?;
            }
        }

        write!(f, "]")
    }
}

impl From<ParsedSectionHeader<'_>> for BString {
    fn from(header: ParsedSectionHeader<'_>) -> Self {
        header.into()
    }
}

impl From<&ParsedSectionHeader<'_>> for BString {
    fn from(header: &ParsedSectionHeader<'_>) -> Self {
        header.to_string().into()
    }
}

impl<'a> From<ParsedSectionHeader<'a>> for Event<'a> {
    fn from(header: ParsedSectionHeader<'_>) -> Event<'_> {
        Event::SectionHeader(header)
    }
}
