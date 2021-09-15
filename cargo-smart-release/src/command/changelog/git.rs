use cargo_metadata::Metadata;
use git_repository as git;

/// A head reference will all commits that are 'governed' by it, that is are in its exclusive ancestry.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Segment {
    head: git::refs::Reference,
    commits: Vec<git::hash::ObjectId>,
}

/// Return the head reference followed by all tags affecting `crate_name` as per our tag name rules, ordered by ancestry.
pub fn crate_references_descending(
    _crate_name: &str,
    _meta: &Metadata,
    _repo: &git::Easy,
) -> anyhow::Result<Vec<Segment>> {
    Ok(vec![])
}
