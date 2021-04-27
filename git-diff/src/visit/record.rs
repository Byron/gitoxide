use git_hash::ObjectId;
use git_object::{bstr::BStr, tree};

pub type PathId = usize;

pub enum Change {
    Addition {
        entry_mode: tree::EntryMode,
        oid: ObjectId,
        path_id: PathId,
    },
    Copy,
    Deletion {
        previous_entry_mode: tree::EntryMode,
        previous_oid: ObjectId,
        path_id: PathId,
    },
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
    pub id: PathId,
}

impl<'a> PathComponent<'a> {
    pub(crate) fn new(name: &'a BStr, id: &mut usize) -> Self {
        let current_id = *id;
        *id += 1;
        PathComponent { id: current_id, name }
    }
}

#[derive(Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum Action {
    Continue,
    Cancel,
}

impl Action {
    pub fn cancelled(&self) -> bool {
        matches!(self, Action::Cancel)
    }
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
            32,
            "this type shouldn't grow without us knowing"
        )
    }
}
