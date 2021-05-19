use crate::tree::visit;
use git_hash::ObjectId;
use git_object::{
    bstr::{BStr, BString, ByteSlice, ByteVec},
    tree,
};
use std::collections::{BTreeMap, VecDeque};

/// A Change as observed by a call to [`visit(…)`][visit::Visit::visit()], enhanced with the path affected by the change.
/// Its similar to [visit::Change] but includes the path that changed.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Change {
    Addition {
        entry_mode: tree::EntryMode,
        oid: ObjectId,
        path: BString,
    },
    Deletion {
        entry_mode: tree::EntryMode,
        oid: ObjectId,
        path: BString,
    },
    Modification {
        previous_entry_mode: tree::EntryMode,
        previous_oid: ObjectId,

        entry_mode: tree::EntryMode,
        oid: ObjectId,

        path: BString,
    },
}

/// A [Visit][visit::Visit] implementation to record every observed change and keep track of the changed paths.
#[derive(Clone, Debug, Default)]
pub struct Recorder {
    path_count: usize,
    path_map: BTreeMap<usize, BString>,
    path_deque: VecDeque<BString>,
    path: BString,
    /// The observed changes.
    pub records: Vec<Change>,
}

impl Recorder {
    fn pop_element(&mut self) {
        if let Some(pos) = self.path.rfind_byte(b'/') {
            self.path.resize(pos, 0);
        } else {
            self.path.clear();
        }
    }

    fn push_element(&mut self, name: &BStr) {
        if !self.path.is_empty() {
            self.path.push(b'/');
        }
        self.path.push_str(name);
    }

    fn path_clone(&self) -> BString {
        self.path.clone()
    }
}

impl visit::Visit for Recorder {
    fn pop_front_tracked_path_and_set_current(&mut self) {
        self.path = self.path_deque.pop_front().expect("every parent is set only once");
    }

    fn push_back_tracked_path_component(&mut self, component: &BStr) {
        self.push_element(component);
        self.path_deque.push_back(self.path.clone());
    }

    fn push_path_component(&mut self, component: &BStr) {
        self.push_element(component);
    }

    fn pop_path_component(&mut self) {
        self.pop_element();
    }

    fn visit(&mut self, change: visit::Change) -> visit::Action {
        use visit::Change::*;
        self.records.push(match change {
            Deletion { entry_mode, oid } => Change::Deletion {
                entry_mode,
                oid,
                path: self.path_clone(),
            },
            Addition { entry_mode, oid } => Change::Addition {
                entry_mode,
                oid,
                path: self.path_clone(),
            },
            Modification {
                previous_entry_mode,
                previous_oid,
                entry_mode,
                oid,
            } => Change::Modification {
                previous_entry_mode,
                previous_oid,
                entry_mode,
                oid,
                path: self.path_clone(),
            },
        });
        visit::Action::Continue
    }
}
