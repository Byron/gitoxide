use std::{borrow::Cow, fmt::Display};

use bstr::BString;

use crate::parse::Event;

impl Event<'_> {
    /// Generates a byte representation of the value. This should be used when
    /// non-UTF-8 sequences are present or a UTF-8 representation can't be
    /// guaranteed.
    #[must_use]
    pub fn to_bstring(&self) -> BString {
        self.into()
    }

    /// Turn this instance into a fully owned one with `'static` lifetime.
    #[must_use]
    pub fn to_owned(&self) -> Event<'static> {
        match self {
            Event::Comment(e) => Event::Comment(e.to_owned()),
            Event::SectionHeader(e) => Event::SectionHeader(e.to_owned()),
            Event::SectionKey(e) => Event::SectionKey(e.to_owned()),
            Event::Value(e) => Event::Value(Cow::Owned(e.clone().into_owned())),
            Event::ValueNotDone(e) => Event::ValueNotDone(Cow::Owned(e.clone().into_owned())),
            Event::ValueDone(e) => Event::ValueDone(Cow::Owned(e.clone().into_owned())),
            Event::Newline(e) => Event::Newline(Cow::Owned(e.clone().into_owned())),
            Event::Whitespace(e) => Event::Whitespace(Cow::Owned(e.clone().into_owned())),
            Event::KeyValueSeparator => Event::KeyValueSeparator,
        }
    }
}

impl Display for Event<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(e) | Self::ValueNotDone(e) | Self::ValueDone(e) => match std::str::from_utf8(e) {
                Ok(e) => e.fmt(f),
                Err(_) => write!(f, "{:02x?}", e),
            },
            Self::Comment(e) => e.fmt(f),
            Self::SectionHeader(e) => e.fmt(f),
            Self::SectionKey(e) => e.fmt(f),
            Self::Newline(e) | Self::Whitespace(e) => e.fmt(f),
            Self::KeyValueSeparator => write!(f, "="),
        }
    }
}

impl From<Event<'_>> for BString {
    fn from(event: Event<'_>) -> Self {
        event.into()
    }
}

impl From<&Event<'_>> for BString {
    fn from(event: &Event<'_>) -> Self {
        match event {
            Event::Value(e) | Event::ValueNotDone(e) | Event::ValueDone(e) => e.as_ref().into(),
            Event::Comment(e) => e.into(),
            Event::SectionHeader(e) => e.into(),
            Event::SectionKey(e) => e.0.as_ref().into(),
            Event::Newline(e) | Event::Whitespace(e) => e.as_ref().into(),
            Event::KeyValueSeparator => "=".into(),
        }
    }
}
