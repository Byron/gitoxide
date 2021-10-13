use std::cmp::Ordering;

use crate::changelog::section::segment::conventional::as_headline;
use crate::ChangeLog;

pub mod init;
mod merge;
mod parse;
pub mod section;
pub mod write;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Section {
    /// A part of a changelog which couldn't be understood and is taken in verbatim. This is usually the pre-amble of the changelog
    /// or a custom footer.
    Verbatim {
        /// The section text, unchanged, up until the next `Release`.
        text: String,
        /// True if this is not created by a human
        generated: bool,
    },

    /// A segment describing a particular release
    Release {
        name: Version,
        date: Option<time::OffsetDateTime>,
        /// the amount of # in front of the heading denoting the release name
        heading_level: usize,
        /// text of events of everything we couldn't parse
        unknown: String,
        /// Removed git conventional messages parsed back from html tags. These may live without a headline, to delete the headline.
        removed_messages: Vec<git_repository::hash::ObjectId>,
        /// portions of a release
        segments: Vec<section::Segment>,
    },
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Version {
    Unreleased,
    Semantic(semver::Version),
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Version::Unreleased, _) => Ordering::Greater.into(),
            (_, Version::Unreleased) => Ordering::Less.into(),
            (Version::Semantic(lhs), Version::Semantic(rhs)) => lhs.partial_cmp(rhs),
        }
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Version::Unreleased, _) => Ordering::Greater,
            (_, Version::Unreleased) => Ordering::Less,
            (Version::Semantic(lhs), Version::Semantic(rhs)) => lhs.cmp(rhs),
        }
    }
}

impl ChangeLog {
    pub fn most_recent_release_section_mut(&mut self) -> (usize, &mut Section) {
        self.sections
            .iter_mut()
            .enumerate()
            .find(|(_, s)| matches!(s, Section::Release { .. }))
            .expect("we never have an entirely empty changelog")
    }
}

impl Section {
    /// Returns true if there are segments that would always be present as they carry essential information about the release.
    pub fn is_essential(&self) -> bool {
        match self {
            Section::Verbatim { .. } => true,
            Section::Release {
                segments,
                unknown,
                removed_messages,
                ..
            } => !unknown.is_empty() || !removed_messages.is_empty() || segments.iter().any(|s| !s.is_read_only()),
        }
    }
    /// Returns true if there is no user-made section, or no edit by users in conventional segments at all.
    /// Note that we can't tell if existing messages were edited (because we don't try hard enough).
    pub fn is_probably_lacking_user_edits(&self) -> bool {
        match self {
            Section::Verbatim { .. } => false,
            Section::Release { segments, .. } => {
                if segments.iter().any(|s| matches!(s, section::Segment::User { .. })) {
                    return false;
                };
                segments.iter().any(
                    |s| matches!(s, section::Segment::Conventional(section::segment::Conventional {kind, removed, ..}) if removed.is_empty() && as_headline(*kind).is_none()),
                )
            }
        }
    }
}
