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

mod from_history {
    use cargo_metadata::Package;
    use git_repository as git;
    use git_repository::prelude::ObjectIdExt;

    use crate::{
        changelog,
        changelog::{section, Section},
        commit, utils,
        utils::is_top_level_package,
    };

    impl Section {
        pub fn from_history_segment(
            package: &Package,
            segment: &commit::history::Segment<'_>,
            repo: &git::Easy,
        ) -> Self {
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
                let history = &segment.history;
                segments.push(section::Segment::Statistics(section::Data::Generated(
                    section::CommitStatistics {
                        count: history.len(),
                        conventional_count: history.iter().filter(|item| item.message.kind.is_some()).count(),
                        unique_issues_count: {
                            let mut issue_names = segment
                                .history
                                .iter()
                                .map(|item| item.message.additions.iter())
                                .flatten()
                                .filter_map(|addition| match addition {
                                    commit::message::Addition::IssueId(id) => Some(id),
                                })
                                .collect::<Vec<_>>();
                            issue_names.sort();
                            issue_names.dedup();
                            issue_names.len()
                        },
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
}
