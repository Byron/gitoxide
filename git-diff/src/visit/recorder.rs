use crate::visit::record;
use git_hash::ObjectId;
use git_object::{
    bstr::{BStr, BString, ByteSlice, ByteVec},
    tree,
};
use std::{collections::BTreeMap, ops::Deref, path::PathBuf};

/// A Change as observed by a call to [`record::Record::record`], enhanced with the path affected by the change.
/// Its similar to [record::Change] but adds a path.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Change {
    Addition {
        entry_mode: tree::EntryMode,
        oid: ObjectId,
        path: PathBuf,
    },
    Deletion {
        entry_mode: tree::EntryMode,
        oid: ObjectId,
        path: PathBuf,
    },
    Modification {
        previous_entry_mode: tree::EntryMode,
        previous_oid: ObjectId,

        entry_mode: tree::EntryMode,
        oid: ObjectId,

        path: PathBuf,
    },
}

/// A [record::Record] implementation to record every observed change and keep track of the changed paths.
#[derive(Clone, Debug, Default)]
pub struct Recorder {
    path_count: usize,
    path_map: BTreeMap<usize, BString>,
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

    fn path_buf(&self) -> PathBuf {
        self.path.deref().to_owned().into_path_buf_lossy()
    }
}

impl record::Record for Recorder {
    type PathId = usize;

    fn set_current_path(&mut self, path: Self::PathId) {
        self.path = self.path_map.remove(&path).expect("every parent is set only once");
    }

    fn push_tracked_path_component(&mut self, component: &BStr) -> Self::PathId {
        self.push_element(component);
        self.path_map.insert(self.path_count, self.path_clone());
        let res = self.path_count;
        self.path_count += 1;
        res
    }

    fn push_path_component(&mut self, component: &BStr) {
        self.push_element(component);
    }

    fn pop_path_component(&mut self) {
        self.pop_element();
    }

    fn record(&mut self, change: record::Change) -> record::Action {
        use record::Change::*;
        self.records.push(match change {
            Deletion { entry_mode, oid } => Change::Deletion {
                entry_mode,
                oid,
                path: self.path_buf(),
            },
            Addition { entry_mode, oid } => Change::Addition {
                entry_mode,
                oid,
                path: self.path_buf(),
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
                path: self.path_buf(),
            },
        });
        record::Action::Continue
    }
}
