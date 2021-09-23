use cargo_metadata::Package;
use git_repository as git;

use crate::{commit, utils, utils::is_top_level_package, ChangeLog};
use git_repository::prelude::ObjectIdExt;

pub enum Section {
    /// A part of a changelog which couldn't be understood and is taken in verbatim. This is usually the pre-amble of the changelog
    /// or a custom footer.
    Verbatim(String),

    /// A segment describing a particular release
    Release {
        name: Version,
        date: Option<time::OffsetDateTime>,
    },
}

pub enum Version {
    Unreleased,
    Semantic(semver::Version),
}
impl Section {
    pub fn from_history_segment(package: &Package, segment: &commit::history::Segment<'_>, repo: &git::Easy) -> Self {
        let package_name = (!is_top_level_package(&package.manifest_path, repo)).then(|| package.name.as_str());

        let version = crate::git::try_strip_tag_path(segment.head.name.to_ref())
            .map(|tag_name| {
                Version::Semantic(
                    utils::parse_possibly_prefixed_tag_version(package_name, tag_name)
                        .expect("here we always have a valid version as it passed a filter"),
                )
            })
            .unwrap_or_else(|| Version::Unreleased);

        let time = segment
            .head
            .peeled
            .expect("all refs here are peeled")
            .attach(repo)
            .object()
            .expect("object exists")
            .commit()
            .expect("target is a commit")
            .committer
            .time;
        let date_time = time::OffsetDateTime::from_unix_timestamp(time.time as i64)
            .expect("always valid unix time")
            .replace_offset(time::UtcOffset::from_whole_seconds(time.offset).expect("valid offset"));
        Section::Release {
            name: version,
            date: date_time.into(),
        }
    }
}

impl ChangeLog {
    pub fn from_history_segments(
        package: &Package,
        segments: &[commit::history::Segment<'_>],
        repo: &git::Easy,
    ) -> Self {
        ChangeLog {
            _segments: segments.iter().fold(Vec::new(), |mut acc, item| {
                acc.push(Section::from_history_segment(package, item, repo));
                acc
            }),
        }
    }
}
