use cargo_metadata::{
    camino::{Utf8Path, Utf8PathBuf},
    Package,
};
use git_repository as git;
use git_repository::prelude::ObjectIdExt;

use crate::{
    changelog,
    changelog::Section,
    commit, utils,
    utils::{is_top_level_package, package_by_name},
    ChangeLog,
};

impl Section {
    pub fn from_history_segment(package: &Package, segment: &commit::history::Segment<'_>, repo: &git::Easy) -> Self {
        let package_name = (!is_top_level_package(&package.manifest_path, repo)).then(|| package.name.as_str());

        let version = crate::git::try_strip_tag_path(segment.head.name.to_ref())
            .map(|tag_name| {
                changelog::Version::Semantic(
                    utils::parse_possibly_prefixed_tag_version(package_name, tag_name)
                        .expect("here we always have a valid version as it passed a filter when creating it"),
                )
            })
            .unwrap_or_else(|| changelog::Version::Unreleased);

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
            thanks_clippy_count: segment
                .history
                .iter()
                .map(|item| item.message.title == "thanks clippy")
                .count(),
            heading_level: 2,
            unknown: Default::default(),
        }
    }
}

impl ChangeLog {
    pub fn for_package_with_write_lock<'a>(
        crate_name: &str,
        history: &commit::History,
        ctx: &'a crate::Context,
    ) -> anyhow::Result<(Self, &'a Package, git::lock::File)> {
        let package = package_by_name(&ctx.meta, crate_name)?;
        let mut log = ChangeLog::from_history_segments(
            package,
            &crate::git::history::crate_ref_segments(package, ctx, history)?,
            &ctx.repo,
        );
        log.sections.insert(
            0,
            Section::Verbatim {
                text: include_str!("header.md").to_owned(),
                generated: true,
            },
        );
        let lock = git::lock::File::acquire_to_update_resource(
            path_from_manifest(&package.manifest_path),
            git::lock::acquire::Fail::Immediately,
            None,
        )?;
        Ok((log, package, lock))
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

fn path_from_manifest(path: &Utf8Path) -> Utf8PathBuf {
    path.parent().expect("parent for Cargo.toml").join("CHANGELOG.md")
}
