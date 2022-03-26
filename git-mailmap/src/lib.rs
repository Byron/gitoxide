#![forbid(unsafe_code, rust_2018_idioms)]

use bstr::BStr;

pub mod parse;
pub fn parse(buf: &[u8]) -> parse::Lines<'_> {
    parse::Lines::new(buf)
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Default)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry<'a> {
    /// The name to map to.
    canonical_name: Option<&'a BStr>,
    /// The email map to.
    canonical_email: Option<&'a BStr>,
    /// The name to look for and replace.
    commit_name: Option<&'a BStr>,
    /// The email to look for and replace.
    commit_email: Option<&'a BStr>,
}

mod entry {
    use crate::Entry;
    use bstr::BStr;

    impl<'a> Entry<'a> {
        pub fn change_name_by_email(proper_name: impl Into<&'a BStr>, commit_email: impl Into<&'a BStr>) -> Self {
            Entry {
                canonical_name: Some(proper_name.into()),
                commit_email: Some(commit_email.into()),
                ..Default::default()
            }
        }
        pub fn change_email_by_email(proper_email: impl Into<&'a BStr>, commit_email: impl Into<&'a BStr>) -> Self {
            Entry {
                canonical_email: Some(proper_email.into()),
                commit_email: Some(commit_email.into()),
                ..Default::default()
            }
        }
        pub fn change_name_and_email_by_email(
            proper_name: impl Into<&'a BStr>,
            proper_email: impl Into<&'a BStr>,
            commit_email: impl Into<&'a BStr>,
        ) -> Self {
            Entry {
                canonical_name: Some(proper_name.into()),
                canonical_email: Some(proper_email.into()),
                commit_email: Some(commit_email.into()),
                ..Default::default()
            }
        }

        pub fn change_name_and_email_by_name_and_email(
            proper_name: impl Into<&'a BStr>,
            proper_email: impl Into<&'a BStr>,
            commit_name: impl Into<&'a BStr>,
            commit_email: impl Into<&'a BStr>,
        ) -> Self {
            Entry {
                canonical_name: Some(proper_name.into()),
                canonical_email: Some(proper_email.into()),
                commit_name: Some(commit_name.into()),
                commit_email: Some(commit_email.into()),
            }
        }
    }
}
