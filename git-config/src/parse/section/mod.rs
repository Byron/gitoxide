use std::{borrow::Cow, fmt::Display};

use bstr::BStr;
use smallvec::SmallVec;

use crate::parse::{Event, Section};

///
pub mod header;

/// A container for events, avoiding heap allocations in typical files.
pub type Events<'a> = SmallVec<[Event<'a>; 64]>;

/// A parsed section header, containing a name and optionally a subsection name.
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
    /// The subsection name without quotes if any exist, and with escapes folded
    /// into their resulting characters.
    /// Thus during serialization, escapes and quotes must be re-added.
    /// This makes it possible to use [`Event`] data for lookups directly.
    pub subsection_name: Option<Cow<'a, BStr>>,
}

impl Section<'_> {
    /// Turn this instance into a fully owned one with `'static` lifetime.
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

mod types {
    macro_rules! generate_case_insensitive {
        ($name:ident, $cow_inner_type:ty, $comment:literal) => {
            #[doc = $comment]
            #[derive(Clone, Eq, Debug, Default)]
            pub struct $name<'a>(pub std::borrow::Cow<'a, $cow_inner_type>);

            impl $name<'_> {
                /// Turn this instance into a fully owned one with `'static` lifetime.
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

            impl PartialOrd for $name<'_> {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    self.cmp(other).into()
                }
            }

            impl Ord for $name<'_> {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    let a = self.0.iter().map(|c| c.to_ascii_lowercase());
                    let b = other.0.iter().map(|c| c.to_ascii_lowercase());
                    a.cmp(b)
                }
            }

            impl std::hash::Hash for $name<'_> {
                fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                    for b in self.0.iter() {
                        b.to_ascii_lowercase().hash(state);
                    }
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

pub(crate) fn into_cow_bstr(c: Cow<'_, str>) -> Cow<'_, BStr> {
    match c {
        Cow::Borrowed(s) => Cow::Borrowed(s.into()),
        Cow::Owned(s) => Cow::Owned(s.into()),
    }
}
