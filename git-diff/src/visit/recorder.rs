use crate::visit::record;
use git_hash::ObjectId;
use git_object::{
    bstr::{BStr, BString, ByteSlice, ByteVec},
    tree,
};
use std::{ops::Deref, path::PathBuf};

#[derive(Clone, Debug, PartialEq, Eq)]
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

pub type Changes = Vec<Change>;

#[derive(Clone, Debug, Default)]
pub struct Recorder {
    path: BString,
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

    fn path_buf(&self) -> PathBuf {
        self.path.deref().to_owned().into_path_buf_lossy()
    }
}

impl record::Record for Recorder {
    fn push_path_component(&mut self, component: record::PathComponent<'_>) {
        self.push_element(component.name)
    }

    fn pop_path_component(&mut self) {
        self.pop_element();
    }

    fn record(&mut self, change: record::Change) -> record::Action {
        use record::Change::*;
        self.records.push(match change {
            Deletion {
                entry_mode,
                oid,
                path_id: _,
            } => Change::Deletion {
                entry_mode,
                oid,
                path: self.path_buf(),
            },
            Addition {
                entry_mode,
                oid,
                path_id: _,
            } => Change::Addition {
                entry_mode,
                oid,
                path: self.path_buf(),
            },
            Modification {
                previous_entry_mode,
                previous_oid,
                entry_mode,
                oid,
                path_id: _,
            } => Change::Modification {
                previous_entry_mode,
                previous_oid,
                entry_mode,
                oid,
                path: self.path_buf(),
            },
            _ => todo!("record other kinds of changes"),
        });
        record::Action::Continue
    }
}
