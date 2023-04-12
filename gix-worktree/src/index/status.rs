//!

///
pub mod worktree;

mod index;

///
pub mod visit;

///
pub mod recorder;

///
pub struct IndexStatus<'index> {
    index: &'index gix_index::State,
}

impl<'index> From<&'index gix_index::File> for IndexStatus<'index> {
    fn from(file: &'index gix_index::File) -> Self {
        Self { index: file }
    }
}

impl<'index> From<&'index gix_index::State> for IndexStatus<'index> {
    fn from(index: &'index gix_index::State) -> Self {
        Self { index }
    }
}
