use crate::parse::{Event, Section};
use bstr::{BStr, BString};
use std::borrow::Cow;
use std::fmt::Display;

/// A parsed section header, containing a name and optionally a subsection name.
///
/// Note that section headers must be parsed as valid ASCII, and thus all valid
/// instances must also necessarily be valid UTF-8.
// TODO: turn these into strings in with str::from_utf8_unchecked
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Header<'a> {
    /// The name of the header.
    pub name: Name<'a>,
    /// The separator used to determine if the section contains a subsection.
    /// This is either a period `.` or a string of whitespace. Note that
    /// reconstruction of subsection format is dependent on this value. If this
    /// is all whitespace, then the subsection name needs to be surrounded by
    /// quotes to have perfect reconstruction.
    pub separator: Option<Cow<'a, BStr>>,
    /// The subsection name without quotes if any exist.
    pub subsection_name: Option<Cow<'a, BStr>>,
}

impl Section<'_> {
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
    pub fn to_owned(&self) -> Section<'static> {
        Section {
            section_header: self.section_header.to_owned(),
            events: self.events.iter().map(Event::to_owned).collect(),
        }
    }
}

impl Display for Section<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.section_header)?;
        for event in &self.events {
            event.fmt(f)?;
        }
        Ok(())
    }
}

impl Header<'_> {
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
    pub fn to_owned(&self) -> Header<'static> {
        Header {
            name: self.name.to_owned(),
            separator: self.separator.clone().map(|v| Cow::Owned(v.into_owned())),
            subsection_name: self.subsection_name.clone().map(|v| Cow::Owned(v.into_owned())),
        }
    }
}

impl Display for Header<'_> {
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

impl From<Header<'_>> for BString {
    fn from(header: Header<'_>) -> Self {
        header.into()
    }
}

impl From<&Header<'_>> for BString {
    fn from(header: &Header<'_>) -> Self {
        header.to_string().into()
    }
}

impl<'a> From<Header<'a>> for Event<'a> {
    fn from(header: Header<'_>) -> Event<'_> {
        Event::SectionHeader(header)
    }
}

mod types {
    macro_rules! generate_case_insensitive {
        ($name:ident, $cow_inner_type:ty, $comment:literal) => {
            #[doc = $comment]
            #[derive(Clone, Eq, Ord, Debug, Default)]
            pub struct $name<'a>(pub std::borrow::Cow<'a, $cow_inner_type>);

            impl $name<'_> {
                /// Coerces into an owned instance. This differs from the standard
                /// [`clone`] implementation as calling clone will _not_ copy the
                /// borrowed variant, while this method will. In other words:
                ///
                /// | Borrow type | `.clone()` | `to_owned()` |
                /// | ----------- | ---------- | ------------ |
                /// | Borrowed    | Borrowed   | Owned        |
                /// | Owned       | Owned      | Owned        |
                ///
                /// This can be most effectively seen by the differing lifetimes
                /// between the two. This method guarantees a `'static` lifetime,
                /// while `clone` does not.
                ///
                /// [`clone`]: Self::clone
                #[must_use]
                pub fn to_owned(&self) -> $name<'static> {
                    $name(std::borrow::Cow::Owned(self.0.clone().into_owned()))
                }
            }

            impl PartialEq for $name<'_> {
                fn eq(&self, other: &Self) -> bool {
                    self.0.eq_ignore_ascii_case(&other.0)
                }
            }

            impl std::fmt::Display for $name<'_> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.0.fmt(f)
                }
            }

            // TODO: compare without lowercase conversion
            impl PartialOrd for $name<'_> {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    self.0
                        .to_ascii_lowercase()
                        .partial_cmp(&other.0.to_ascii_lowercase())
                }
            }

            impl std::hash::Hash for $name<'_> {
                fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                    self.0.to_ascii_lowercase().hash(state)
                }
            }

            impl<'a> From<&'a str> for $name<'a> {
                fn from(s: &'a str) -> Self {
                    Self(std::borrow::Cow::Borrowed(s.into()))
                }
            }

            impl<'a> From<std::borrow::Cow<'a, bstr::BStr>> for $name<'a> {
                fn from(s: std::borrow::Cow<'a, bstr::BStr>) -> Self {
                    Self(s)
                }
            }

            impl<'a> std::ops::Deref for $name<'a> {
                type Target = $cow_inner_type;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        };
    }
    generate_case_insensitive!(
        Name,
        bstr::BStr,
        "Wrapper struct for section header names, like `includeIf`, since these are case-insensitive."
    );

    generate_case_insensitive!(
        Key,
        bstr::BStr,
        "Wrapper struct for key names, like `path` in `include.path`, since keys are case-insensitive."
    );
}
pub use types::{Key, Name};
