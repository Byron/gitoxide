use std::{borrow::Cow, fmt::Display};

use bstr::{BStr, BString};

use crate::parse::Event;

impl Event<'_> {
    /// Serialize this type into a `BString` for convenience.
    ///
    /// Note that `to_string()` can also be used, but might not be lossless.
    #[must_use]
    pub fn to_bstring(&self) -> BString {
        let mut buf = Vec::new();
        self.write_to(&mut buf).expect("io error impossible");
        buf.into()
    }

    /// Turn ourselves into the text we represent, lossy.
    ///
    /// Note that this will be partial in case of `ValueNotDone` which doesn't include the backslash, and `SectionHeader` will only
    /// provide their name, lacking the sub-section name.
    pub fn to_bstr_lossy(&self) -> &BStr {
        match self {
            Self::ValueNotDone(e) | Self::Whitespace(e) | Self::Newline(e) | Self::Value(e) | Self::ValueDone(e) => {
                e.as_ref()
            }
            Self::KeyValueSeparator => "=".into(),
            Self::SectionKey(k) => k.0.as_ref(),
            Self::SectionHeader(h) => h.name.0.as_ref(),
            Self::Comment(c) => c.text.as_ref(),
        }
    }

    /// Stream ourselves to the given `out`, in order to reproduce this event mostly losslessly
    /// as it was parsed.
    pub fn write_to(&self, mut out: &mut dyn std::io::Write) -> std::io::Result<()> {
        match self {
            Self::ValueNotDone(e) => {
                out.write_all(e.as_ref())?;
                out.write_all(b"\\")
            }
            Self::Whitespace(e) | Self::Newline(e) | Self::Value(e) | Self::ValueDone(e) => out.write_all(e.as_ref()),
            Self::KeyValueSeparator => out.write_all(b"="),
            Self::SectionKey(k) => out.write_all(k.0.as_ref()),
            Self::SectionHeader(h) => h.write_to(&mut out),
            Self::Comment(c) => c.write_to(&mut out),
        }
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
        Display::fmt(&self.to_bstring(), f)
    }
}

impl From<Event<'_>> for BString {
    fn from(event: Event<'_>) -> Self {
        event.into()
    }
}

impl From<&Event<'_>> for BString {
    fn from(event: &Event<'_>) -> Self {
        event.to_bstring()
    }
}
