use cargo_metadata::{
    camino::{Utf8Path, Utf8PathBuf},
    Package,
};
use git_repository as git;
use git_repository::prelude::ObjectIdExt;

use crate::{
    changelog,
    changelog::{section, Section},
    commit, utils,
    utils::{is_top_level_package, package_by_name, will},
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
        let date = match version {
            changelog::Version::Unreleased => None,
            changelog::Version::Semantic(_) => Some(date_time),
        };
        let mut segments = Vec::new();

        {
            segments.push(section::Segment::Statistics(section::Data::Generated(
                section::CommitStatistics {
                    count: segment.history.len(),
                },
            )))
        }
        {
            let count = segment
                .history
                .iter()
                .filter(|item| item.message.title.starts_with("thanks clippy"))
                .count();
            if count > 0 {
                segments.push(section::Segment::Clippy(section::Data::Generated(
                    section::ThanksClippy { count },
                )))
            }
        }
        Section::Release {
            name: version,
            date,
            heading_level: 2,
            segments,
            unknown: Default::default(),
        }
    }
}

impl ChangeLog {
    pub fn for_package_with_write_lock<'a>(
        package: &'a Package,
        history: &commit::History,
        ctx: &'a crate::Context,
        dry_run: bool,
    ) -> anyhow::Result<(Self, bool, git::lock::File)> {
        let mut generated = ChangeLog::from_history_segments(
            package,
            &crate::git::history::crate_ref_segments(package, ctx, history)?,
            &ctx.repo,
        );
        generated.sections.insert(
            0,
            Section::Verbatim {
                text: include_str!("header.md").to_owned(),
                generated: true,
            },
        );
        let changelog_path = path_from_manifest(&package.manifest_path);
        let lock =
            git::lock::File::acquire_to_update_resource(&changelog_path, git::lock::acquire::Fail::Immediately, None)?;
        let (log, changed) = if let Ok(existing) = std::fs::read_to_string(changelog_path) {
            log::info!("{} edit existing changelog for '{}'", will(dry_run), package.name);
            let existing = ChangeLog::from_markdown(&existing);
            let copy_of_existing = existing.clone();
            let merged = existing.merge_generated(generated);
            let changed = merged != copy_of_existing;
            (merged, changed)
        } else {
            log::info!("{} create a new changelog for '{}'", will(dry_run), package.name);
            (generated, true)
        };
        Ok((log, changed, lock))
    }

    pub fn for_crate_by_name_with_write_lock<'a>(
        crate_name: &str,
        history: &commit::History,
        ctx: &'a crate::Context,
        dry_run: bool,
    ) -> anyhow::Result<(Self, &'a Package, git::lock::File)> {
        let package = package_by_name(&ctx.meta, crate_name)?;
        let (log, _changed, lock) = Self::for_package_with_write_lock(package, history, ctx, dry_run)?;
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
