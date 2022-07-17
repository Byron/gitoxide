use std::{convert::TryFrom, fmt::Display, str::FromStr};

use bstr::{BStr, BString};

use crate::file::Metadata;
use crate::{parse, File};

impl FromStr for File<'static> {
    type Err = parse::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse::Events::from_bytes_owned(s.as_bytes(), None)
            .map(|events| File::from_parse_events_no_includes(events, Metadata::api()))
    }
}

impl<'a> TryFrom<&'a str> for File<'a> {
    type Error = parse::Error;

    /// Convenience constructor. Attempts to parse the provided string into a
    /// [`File`]. See [`Events::from_str()`][crate::parse::Events::from_str()] for more information.
    fn try_from(s: &'a str) -> Result<File<'a>, Self::Error> {
        parse::Events::from_str(s).map(|events| Self::from_parse_events_no_includes(events, Metadata::api()))
    }
}

impl<'a> TryFrom<&'a BStr> for File<'a> {
    type Error = parse::Error;

    /// Convenience constructor. Attempts to parse the provided byte string into
    /// a [`File`]. See [`Events::from_bytes()`][parse::Events::from_bytes()] for more information.
    fn try_from(value: &'a BStr) -> Result<File<'a>, Self::Error> {
        parse::Events::from_bytes(value, None)
            .map(|events| Self::from_parse_events_no_includes(events, Metadata::api()))
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
