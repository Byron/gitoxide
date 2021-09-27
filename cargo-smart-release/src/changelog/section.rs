use std::collections::BTreeMap;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Segment {
    /// A portion of a Section that we couldn't make sense of, but which should be kept as is nonetheless.
    User {
        text: String,
    },
    Details(Data<Details>),
    Statistics(Data<CommitStatistics>),
    Clippy(Data<ThanksClippy>),
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

pub mod details {
    use std::fmt;

    #[derive(PartialEq, Eq, Ord, PartialOrd, Debug, Clone)]
    pub enum Category {
        Issue(String),
        Uncategorized,
    }

    impl fmt::Display for Category {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Category::Uncategorized => f.write_str("Uncategorized"),
                Category::Issue(issue) => write!(f, "#{}", issue),
            }
        }
    }

    #[derive(PartialEq, Eq, Debug, Clone)]
    pub struct Message {
        pub title: String,
        pub body: Option<String>,
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Details {
    pub commits_by_category: BTreeMap<details::Category, Vec<details::Message>>,
}

impl Details {
    pub const TITLE: &'static str = "Commit Details";
    pub const PREFIX: &'static str = "<details><summary>view details</summary>";
    pub const END: &'static str = "</details>";
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CommitStatistics {
    /// Amount of commits that contributed to the release
    pub count: usize,
    /// The time span from first to last commit, if there is more than one.
    pub duration: Option<time::Duration>,
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
    pub const TITLE: &'static str = "Thanks Clippy";
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
    use time::OffsetDateTime;

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
            let date_time = time_to_offset_date_time(time);
            let date = match version {
                changelog::Version::Unreleased => None,
                changelog::Version::Semantic(_) => Some(date_time),
            };
            let mut segments = Vec::new();
            let history = &segment.history;
            {
                let duration = history
                    .last()
                    .map(|last| date_time - time_to_offset_date_time(last.commit_time));
                segments.push(section::Segment::Statistics(section::Data::Generated(
                    section::CommitStatistics {
                        count: history.len(),
                        duration,
                        conventional_count: history.iter().filter(|item| item.message.kind.is_some()).count(),
                        unique_issues_count: {
                            let mut issue_names = history
                                .iter()
                                .map(|item| item.message.additions.iter())
                                .flatten()
                                .map(|addition| match addition {
                                    commit::message::Addition::IssueId(id) => id,
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
                let count = history
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

    fn time_to_offset_date_time(time: git::actor::Time) -> OffsetDateTime {
        time::OffsetDateTime::from_unix_timestamp(time.time as i64)
            .expect("always valid unix time")
            .replace_offset(time::UtcOffset::from_whole_seconds(time.offset).expect("valid offset"))
    }
}
