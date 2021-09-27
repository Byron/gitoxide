use crate::ChangeLog;

mod init;
mod merge;
mod parse;
pub mod section;
mod write;

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
        /// portions of a release
        segments: Vec<section::Segment>,
    },
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Version {
    Unreleased,
    Semantic(semver::Version),
}

impl ChangeLog {
    pub fn most_recent_release_mut(&mut self) -> &mut Section {
        self.sections
            .iter_mut()
            .find(|s| matches!(s, Section::Release { .. }))
            .expect("we never have an entirely empty changelog")
    }
}

impl Section {
    /// Returns true if there are segments that would always be present as they carry essential information about the release.
    pub fn is_essential(&self) -> bool {
        match self {
            Section::Verbatim { .. } => true,
            Section::Release { segments, .. } => segments.iter().any(|s| !s.is_read_only()),
        }
    }
}

impl section::Segment {
    pub fn is_read_only(&self) -> bool {
        match self {
            section::Segment::User { .. } => false,
            section::Segment::Clippy(_) | section::Segment::Statistics(_) | section::Segment::Details(_) => true,
        }
    }
}
