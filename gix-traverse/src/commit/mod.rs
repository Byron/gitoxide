use gix_object::FindExt;
use smallvec::SmallVec;

/// An iterator over the ancestors one or more starting commits
pub struct Ancestors<Find, Predicate> {
    objects: Find,
    cache: Option<gix_commitgraph::Graph>,
    predicate: Predicate,
    state: ancestors::State,
    parents: Parents,
    sorting: Sorting,
}

/// Simple ancestors traversal
pub mod ancestors;
pub use ancestors::Sorting;

// Topological traversal
pub mod topo;

/// Specify how to handle commit parents during traversal.
#[derive(Default, Copy, Clone)]
pub enum Parents {
    /// Traverse all parents, useful for traversing the entire ancestry.
    #[default]
    All,
    /// Only traverse along the first parent, which commonly ignores all branches.
    First,
}

/// The collection of parent ids we saw as part of the iteration.
///
/// Note that this list is truncated if [`Parents::First`] was used.
pub type ParentIds = SmallVec<[gix_hash::ObjectId; 1]>;

/// Information about a commit that we obtained naturally as part of the iteration.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Info {
    /// The id of the commit.
    pub id: gix_hash::ObjectId,
    /// All parent ids we have encountered. Note that these will be at most one if [`Parents::First`] is enabled.
    pub parent_ids: ParentIds,
    /// The time at which the commit was created. It's only `Some(_)` if sorting is not [`Sorting::BreadthFirst`], as the walk
    /// needs to require the commit-date.
    pub commit_time: Option<gix_date::SecondsSinceUnixEpoch>,
}

enum Either<'buf, 'cache> {
    CommitRefIter(gix_object::CommitRefIter<'buf>),
    CachedCommit(gix_commitgraph::file::Commit<'cache>),
}

fn find<'cache, 'buf, Find>(
    cache: Option<&'cache gix_commitgraph::Graph>,
    objects: Find,
    id: &gix_hash::oid,
    buf: &'buf mut Vec<u8>,
) -> Result<Either<'buf, 'cache>, gix_object::find::existing_iter::Error>
where
    Find: gix_object::Find,
{
    match cache.and_then(|cache| cache.commit_by_id(id).map(Either::CachedCommit)) {
        Some(c) => Ok(c),
        None => objects.find_commit_iter(id, buf).map(Either::CommitRefIter),
    }
}
