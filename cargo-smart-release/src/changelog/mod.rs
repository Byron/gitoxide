use cargo_metadata::Package;
use git_repository as git;
use git_repository::prelude::ObjectIdExt;

use crate::utils::package_by_name;
use crate::{commit, utils, utils::is_top_level_package, ChangeLog};
use cargo_metadata::camino::{Utf8Path, Utf8PathBuf};

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
                        .expect("here we always have a valid version as it passed a filter when creating it"),
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
            .to_commit()
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
    pub fn for_package<'a>(
        crate_name: &str,
        history: &commit::History,
        ctx: &'a crate::Context,
    ) -> anyhow::Result<(Self, &'a Package)> {
        let package = package_by_name(&ctx.meta, crate_name)?;
        let mut log = ChangeLog::from_history_segments(
            package,
            &crate::git::history::crate_ref_segments(package, &ctx, history)?,
            &ctx.repo,
        );
        log.sections.insert(
            0,
            Section::Verbatim {
                text: include_str!("header.md").to_owned(),
                generated: true,
            },
        );
        Ok((log, package))
    }

    pub fn path_from_manifest(path: &Utf8Path) -> Utf8PathBuf {
        path.parent().expect("parent for Cargo.toml").join("CHANGELOG.md")
    }

    fn from_history_segments(package: &Package, segments: &[commit::history::Segment<'_>], repo: &git::Easy) -> Self {
        ChangeLog {
            sections: segments.iter().fold(Vec::new(), |mut acc, item| {
                acc.push(Section::from_history_segment(package, item, repo));
                acc
            }),
        }
    }
}
