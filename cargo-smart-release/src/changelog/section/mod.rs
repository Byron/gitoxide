mod from_history;
pub mod segment;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Segment {
    /// A portion of a Section that we couldn't make sense of, but which should be kept as is nonetheless.
    User {
        markdown: String,
    },
    Conventional(segment::Conventional),
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

impl Segment {
    pub fn is_read_only(&self) -> bool {
        match self {
            Segment::User { .. } | Segment::Conventional { .. } => false,
            Segment::Clippy(_) | Segment::Statistics(_) | Segment::Details(_) => true,
        }
    }
}
