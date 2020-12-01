use crate::Span;

// we parse leading and trailing whitespace into comments, avoiding the notion of whitespace.
// This means we auto-trim whitespace otherwise, which I consider a feature
pub(crate) type Comment = Span;

/// A section or sub-section (in case `sub_name` is `Some()`), i.e.
///
/// ```text
/// [hello]
///
/// [hello.world]
/// ```
#[derive(Clone, PartialOrd, PartialEq, Ord, Eq)]
pub(crate) struct Section {
    pub(crate) name: Span,
    pub(crate) sub_name: Option<Span>,
}

/// A key-value entry of a git-config file, like `name = value`
#[derive(Clone, PartialOrd, PartialEq, Ord, Eq)]
pub(crate) struct Entry {
    pub(crate) name: Span,
    pub(crate) value: Option<Span>,
}
