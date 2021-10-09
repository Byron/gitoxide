use std::{collections::BTreeMap, ops::Sub};

use cargo_metadata::Package;
use git_repository as git;
use git_repository::prelude::ObjectIdExt;

use crate::{
    changelog,
    changelog::{section, section::segment::Selection, Section},
    commit, utils,
    utils::{is_top_level_package, time_to_offset_date_time},
};

impl Section {
    pub fn from_history_segment(
        package: &Package,
        segment: &commit::history::Segment<'_>,
        repo: &git::Easy,
        selection: section::segment::Selection,
    ) -> Self {
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

        let date_time = time_to_offset_date_time(time);
        let mut segments = Vec::new();
        let history = &segment.history;
        if !history.is_empty() {
            if selection.contains(Selection::GIT_CONVENTIONAL) {
                let mut mapping = BTreeMap::default();
                for (id, kind, title, is_breaking) in history.iter().filter_map(|i| {
                    i.message
                        .kind
                        .as_ref()
                        .map(|kind| (i.id, kind, i.message.title.clone(), i.message.breaking))
                }) {
                    mapping
                        .entry((is_breaking, kind))
                        .or_insert_with(Vec::new)
                        .push(section::segment::conventional::Message::Generated { id, title })
                }
                // TODO: proper sorting
                segments.extend(mapping.into_iter().map(|((is_breaking, kind), messages)| {
                    section::Segment::Conventional(section::segment::Conventional {
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
                segments.push(section::Segment::Statistics(section::Data::Generated(
                    section::segment::CommitStatistics {
                        count: history.len(),
                        duration,
                        conventional_count: history.iter().filter(|item| item.message.kind.is_some()).count(),
                        unique_issues_count: commits_by_category.len(),
                    },
                )));
            }
            if selection.contains(Selection::CLIPPY) {
                let count = history
                    .iter()
                    .filter(|item| item.message.title.starts_with("thanks clippy"))
                    .count();
                if count > 0 {
                    segments.push(section::Segment::Clippy(section::Data::Generated(
                        section::segment::ThanksClippy { count },
                    )))
                }
            }
            if let Some(commits_by_category) =
                message_by_category.filter(|_| selection.contains(Selection::COMMIT_DETAILS))
            {
                segments.push(section::Segment::Details(section::Data::Generated(
                    section::segment::Details { commits_by_category },
                )));
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
            heading_level: 2,
            segments,
            unknown: Default::default(),
        }
    }
}
