use git_repository as git;

mod from_history;
pub mod segment;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Segment {
    /// A portion of a Section that we couldn't make sense of, but which should be kept as is nonetheless.
    User {
        text: String,
    },
    Conventional {
        /// The git-conventional kind
        kind: &'static str,
        /// Whether or not the segment contains only breaking changes
        is_breaking: bool,
        /// object IDs parsed from markdown with no surrounding text. These are considered removed, so we shouldn't repopulate them.
        removed: Vec<git::hash::ObjectId>,
        /// The messages to convey
        messages: Vec<segment::conventional::Message>,
    },
    Details(Data<segment::Details>),
    Statistics(Data<segment::CommitStatistics>),
    Clippy(Data<segment::ThanksClippy>),
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
