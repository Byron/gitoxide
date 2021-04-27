use git_hash::ObjectId;
use git_object::{bstr::BStr, tree};

pub type PathId = usize;

pub enum Change {
    Addition {
        entry_mode: tree::EntryMode,
        oid: ObjectId,
    },
    Copy,
    Deletion {
        entry_mode: tree::EntryMode,
        oid: ObjectId,
    },
    Modification {
        previous_entry_mode: tree::EntryMode,
        previous_oid: ObjectId,

        entry_mode: tree::EntryMode,
        oid: ObjectId,
    },
    Renaming,
    Type,
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
    type PathId;

    fn set_parent(&mut self, path: PathId);
    fn push_tree_name(&mut self, component: &BStr) -> Self::PathId;
    fn push_non_tree_name(&mut self, component: &BStr);
    fn pop_path_name(&mut self);
    fn record(&mut self, change: Change) -> Action;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_change() {
        assert_eq!(
            std::mem::size_of::<Change>(),
            56,
            "this type shouldn't grow without us knowing"
        )
    }
}
