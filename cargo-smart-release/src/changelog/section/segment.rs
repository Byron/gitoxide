use std::collections::BTreeMap;

use bitflags::bitflags;

pub mod conventional {

    /// A message that is associated with a Segment for a particular git-conventional segment
    #[derive(PartialEq, Eq, Debug, Clone)]
    pub enum Message {
        User {
            /// The user text for direct markdown-to-markdown copy
            markdown: String,
        },
        Generated {
            /// The id of the message/commit the data is coming from, useful to identify the markdown associate with this message.
            id: gix::ObjectId,
            title: String,
            body: Option<String>,
        },
    }

    /// Note that this depends on `crate::commit::message::to_static()`,
    /// Not having a headline means it won't be written back unless it contains breaking changes.
    pub fn as_headline(kind: &str) -> Option<&'static str> {
        // NOTE: adding one here needs additions to parse.rs
        Some(match kind {
            "fix" => "Bug Fixes",
            "add" | "added" => "Added",
            "feat" => "New Features",
            "revert" => "Reverted",
            "remove" => "Removed",
            "change" => "Changed",
            "docs" => "Documentation",
            "perf" => "Performance",
            "chore" => "Chore",
            "test" => "Test",
            "refactor" => "Refactor",
            "other" => "Other",
            "style" => "Style",
            _unknown => return None,
        })
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Conventional {
    /// The git-conventional kind
    pub kind: &'static str,
    /// Whether or not the segment contains only breaking changes
    pub is_breaking: bool,
    /// object IDs parsed from markdown with no surrounding text. These are considered removed, so we shouldn't repopulate them.
    pub removed: Vec<gix::ObjectId>,
    /// The messages to convey
    pub messages: Vec<conventional::Message>,
}

impl Conventional {
    pub const REMOVED_HTML_PREFIX: &'static str = "<csr-id-";
    pub const BREAKING_TITLE: &'static str = "BREAKING";
    pub const BREAKING_TITLE_ENCLOSED: &'static str = "(BREAKING)";
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
        pub id: gix::ObjectId,
    }

    impl From<&crate::commit::history::Item> for Message {
        fn from(v: &crate::commit::history::Item) -> Self {
            Message {
                title: v.message.title.to_owned(),
                id: v.id,
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Details {
    pub commits_by_category: BTreeMap<details::Category, Vec<details::Message>>,
}

impl Details {
    pub const TITLE: &'static str = "Commit Details";
    pub const HTML_PREFIX: &'static str = "<details><summary>view details</summary>";
    pub const HTML_PREFIX_END: &'static str = "</details>";
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
    pub unique_issues: Vec<details::Category>,
    /// The duration from the release before this one, if this isn't the first release.
    pub time_passed_since_last_release: Option<time::Duration>,
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

bitflags! {
    #[derive(Debug, Copy, Clone)]
    pub struct Selection: u8 {
        const CLIPPY = 1<<0;
        const COMMIT_DETAILS = 1<<1;
        const COMMIT_STATISTICS = 1<<2;
        const GIT_CONVENTIONAL = 1<<3;
    }
}
