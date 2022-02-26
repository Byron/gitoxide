use std::{collections::BTreeMap, ops::Sub};

use cargo_metadata::Package;
use git_repository as git;
use git_repository::prelude::ObjectIdExt;
use time::OffsetDateTime;

use crate::{
    changelog,
    changelog::{
        section,
        section::{segment::Selection, Segment},
        Section,
    },
    commit, utils,
    utils::{is_top_level_package, time_to_offset_date_time},
};

impl Section {
    pub const DEFAULT_PREFIX: &'static str = "v";

    pub fn from_history_segment(
        package: &Package,
        segment: &commit::history::Segment<'_>,
        repo: &git::Repository,
        selection: section::segment::Selection,
        prev_segment: Option<&commit::history::Segment<'_>>,
    ) -> Self {
        let date_time = segment_head_time(segment, repo);
        let prev_date_time = prev_segment.map(|segment| segment_head_time(segment, repo));

        let mut segments = Vec::new();
        let history = &segment.history;
        if !history.is_empty() {
            if selection.contains(Selection::GIT_CONVENTIONAL) {
                let mut mapping = BTreeMap::default();
                for (id, kind, title, is_breaking, body) in history.iter().filter_map(|i| {
                    i.message.kind.as_ref().map(|kind| {
                        (
                            i.id,
                            kind,
                            i.message.title.clone(),
                            i.message.breaking,
                            i.message.body.clone(),
                        )
                    })
                }) {
                    mapping
                        .entry((is_breaking, kind))
                        .or_insert_with(Vec::new)
                        .push(section::segment::conventional::Message::Generated { id, title, body })
                }
                // TODO: proper sorting
                segments.extend(mapping.into_iter().map(|((is_breaking, kind), messages)| {
                    Segment::Conventional(section::segment::Conventional {
                        kind,
                        is_breaking,
                        removed: Vec::new(),
                        messages,
                    })
                }));
            }
            let message_by_category = selection
                .intersects(Selection::COMMIT_STATISTICS | Selection::COMMIT_DETAILS)
                .then(|| {
                    let mut mapping = BTreeMap::default();
                    for &item in history {
                        let mut issue_associations = 0;
                        for possibly_issue in &item.message.additions {
                            match possibly_issue {
                                commit::message::Addition::IssueId(issue) => {
                                    mapping
                                        .entry(section::segment::details::Category::Issue(issue.to_owned()))
                                        .or_insert_with(Vec::new)
                                        .push(item.into());
                                    issue_associations += 1;
                                }
                            }
                        }
                        if issue_associations == 0 {
                            mapping
                                .entry(section::segment::details::Category::Uncategorized)
                                .or_insert_with(Vec::new)
                                .push(item.into());
                        }
                    }
                    mapping
                });
            if let Some(commits_by_category) = message_by_category
                .as_ref()
                .filter(|_| selection.contains(Selection::COMMIT_STATISTICS))
            {
                let duration = history
                    .last()
                    .map(|last| date_time.sub(time_to_offset_date_time(last.commit_time)));
                segments.push(Segment::Statistics(section::Data::Generated(
                    section::segment::CommitStatistics {
                        count: history.len(),
                        duration,
                        time_passed_since_last_release: prev_date_time.map(|prev_time| date_time.sub(prev_time)),
                        conventional_count: history.iter().filter(|item| item.message.kind.is_some()).count(),
                        unique_issues: {
                            let mut v = commits_by_category
                                .keys()
                                .filter(|c| matches!(c, section::segment::details::Category::Issue(_)))
                                .cloned()
                                .collect::<Vec<_>>();
                            v.sort();
                            v
                        },
                    },
                )));
            }
            if selection.contains(Selection::CLIPPY) {
                let count = history
                    .iter()
                    .filter(|item| item.message.title.starts_with("thanks clippy"))
                    .count();
                if count > 0 {
                    segments.push(Segment::Clippy(section::Data::Generated(
                        section::segment::ThanksClippy { count },
                    )))
                }
            }
            if let Some(commits_by_category) =
                message_by_category.filter(|_| selection.contains(Selection::COMMIT_DETAILS))
            {
                segments.push(Segment::Details(section::Data::Generated(section::segment::Details {
                    commits_by_category,
                })));
            }
        }

        let version = crate::git::try_strip_tag_path(segment.head.name.to_ref())
            .map(|tag_name| {
                let package_name = (!is_top_level_package(&package.manifest_path, repo)).then(|| package.name.as_str());
                changelog::Version::Semantic(
                    utils::parse_possibly_prefixed_tag_version(package_name, tag_name)
                        .expect("here we always have a valid version as it passed a filter when creating it"),
                )
            })
            .unwrap_or_else(|| changelog::Version::Unreleased);
        let date = match version {
            changelog::Version::Unreleased => None,
            changelog::Version::Semantic(_) => Some(date_time),
        };

        Section::Release {
            name: version,
            date,
            heading_level: changelog::DEFAULT_HEADING_LEVEL,
            version_prefix: Self::DEFAULT_PREFIX.to_owned(),
            segments,
            removed_messages: Default::default(),
            unknown: Default::default(),
        }
    }
}

fn segment_head_time(segment: &commit::history::Segment<'_>, repo: &git::Repository) -> OffsetDateTime {
    let time = segment
        .head
        .peeled
        .expect("all refs here are peeled")
        .attach(repo)
        .object()
        .expect("object exists")
        .to_commit_ref()
        .committer
        .time;

    time_to_offset_date_time(time)
}
