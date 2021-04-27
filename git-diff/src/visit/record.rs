use git_hash::ObjectId;
use git_object::{bstr::BStr, tree};

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
    type PathId: Clone + Default;

    fn set_current_path(&mut self, path: Self::PathId);
    fn push_tracked_path_component(&mut self, component: &BStr) -> Self::PathId;
    fn push_path_component(&mut self, component: &BStr);
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
            46,
            "this type shouldn't grow without us knowing"
        )
    }
}
