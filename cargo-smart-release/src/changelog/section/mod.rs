use std::collections::BTreeMap;

pub mod details;
mod from_history;
pub mod segment;

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
