use git_hash::ObjectId;
use git_object::{bstr::BStr, tree};

pub enum Change {
    Addition { mode: tree::EntryMode, oid: ObjectId },
    Copy,
    Deletion,
    Modification,
    Renaming,
    Type,
}

#[derive(Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum PathComponentUpdateMode {
    Replace,
    Push,
}

#[derive(Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct PathComponent<'a> {
    pub name: &'a BStr,
    /// An ID referring uniquely to the path built thus far. Used to keep track of source paths
    /// in case of [renames][Change::Rename] and [copies][Change::Copy].
    pub id: usize,
}

#[derive(Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum Action {
    Continue,
    Cancel,
}

pub trait Record {
    fn update_path_component(&mut self, component: PathComponent<'_>, mode: PathComponentUpdateMode);
    fn pop_path_component(&mut self);
    fn record(&mut self, change: Change) -> Action;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_change() {
        assert_eq!(
            std::mem::size_of::<Change>(),
            22,
            "this type shouldn't grow without us knowing"
        )
    }
}
