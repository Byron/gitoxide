use crate::{commit, ChangeLog};
use git_repository as git;

pub enum Segment {
    /// A part of a changelog which couldn't be understood and is taken in verbatim. This is usually the pre-amble of the changelog
    /// or a custom footer.
    Verbatim(String),

    /// A segment describing a particular release
    Release { name: Version, date: time::Date },
}

pub enum Version {
    Unreleased,
    Semantic(semver::Version),
}
impl Segment {
    pub fn from_history_segment(_segment: &commit::history::Segment<'_>, _repo: &git::Easy) -> Self {
        todo!("segment from history item")
    }
}

impl ChangeLog {
    pub fn from_history_segments(segments: &[commit::history::Segment<'_>], repo: &git::Easy) -> Self {
        ChangeLog {
            _segments: segments.iter().fold(Vec::new(), |mut acc, item| {
                acc.push(Segment::from_history_segment(item, repo));
                acc
            }),
        }
    }
}
