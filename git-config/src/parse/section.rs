use crate::parse::{Event, Section};
use bstr::{BStr, BString};
use smallvec::SmallVec;
use std::borrow::Cow;
use std::fmt::Display;

/// A container for events, avoiding heap allocations in typical files.
pub type Events<'a> = SmallVec<[Event<'a>; 64]>;

/// A parsed section header, containing a name and optionally a subsection name.
///
/// Note that section headers must be parsed as valid ASCII, and thus all valid
/// instances must also necessarily be valid UTF-8.
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

impl Header<'_> {
    /// Generates a byte representation of the value. This should be used when
    /// non-UTF-8 sequences are present or a UTF-8 representation can't be
    /// guaranteed.
    #[must_use]
    pub fn to_bstring(&self) -> BString {
        self.into()
    }

    /// Turn this instance into a fully owned one with `'static` lifetime.
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

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::cmp::Ordering;

        #[test]
        fn case_insentive_eq() {
            assert_eq!(Key::from("aBc"), Key::from("AbC"));
        }

        #[test]
        fn case_insentive_ord() {
            assert_eq!(Key::from("a").cmp(&Key::from("a")), Ordering::Equal);
            assert_eq!(Key::from("aBc").cmp(&Key::from("AbC")), Ordering::Equal);
        }

        #[test]
        fn case_insentive_hash() {
            fn calculate_hash<T: std::hash::Hash>(t: T) -> u64 {
                use std::hash::Hasher;
                let mut s = std::collections::hash_map::DefaultHasher::new();
                t.hash(&mut s);
                s.finish()
            }
            assert_eq!(calculate_hash(Key::from("aBc")), calculate_hash(Key::from("AbC")));
        }
    }
}
pub use types::{Key, Name};
