use crate::ChangeLog;

mod init;
mod merge;
mod parse;
mod write;

pub mod section {
    #[derive(Eq, PartialEq, Debug, Clone)]
    pub enum Segment {
        /// A portion of a Section that we couldn't make sense of, but which should be kept as is nonetheless.
        User {
            text: String,
        },
        /// A thanks clippy headline with the amount of times clippy helped
        Clippy(Data<ThanksClippy>),
        Statistics(Data<CommitStatistics>),
    }

    #[derive(Eq, Debug, Clone)]
    pub enum Data<T> {
        Parsed,
        Generated(T),
    }

    impl<T: PartialEq<T>> PartialEq<Data<T>> for Data<T> {
        fn eq(&self, other: &Data<T>) -> bool {
            match (self, other) {
                (Data::Generated(lhs), Data::Generated(rhs)) => lhs == rhs,
                (_, _) => true,
            }
        }
    }

    #[derive(PartialEq, Eq, Debug, Clone)]
    pub struct CommitStatistics {
        /// Amount of commits that contributed to the release
        pub count: usize,
        /// Amount of commits that could be parsed as git-conventional
        pub conventional_count: usize,
        /// The issue numbers that were referenced in commit messages
        pub unique_issues_count: usize,
    }

    impl CommitStatistics {
        pub const TITLE: &'static str = "Commit Statistics";
    }

    #[derive(PartialEq, Eq, Debug, Clone)]
    pub struct ThanksClippy {
        pub count: usize,
    }

    impl ThanksClippy {
        pub const TITLE: &'static str = "Thanks Clippy…";
    }
}

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
            Section::Release { segments, .. } => segments.iter().any(|s| match s {
                section::Segment::User { .. } => true,
                section::Segment::Clippy(_) | section::Segment::Statistics(_) => false,
            }),
        }
    }
}
