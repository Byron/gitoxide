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
        }
    }

    fn push_element(&mut self, name: &BStr) {
        if !self.path.is_empty() {
            self.path.push(b'/');
        }
        self.path.push_str(name);
    }
}

impl record::Record for Recorder {
    fn update_path_component(&mut self, component: record::PathComponent<'_>, mode: record::PathComponentUpdateMode) {
        use record::PathComponentUpdateMode::*;
        match mode {
            Push => self.push_element(component.name),
            Replace => {
                self.pop_element();
                self.push_element(component.name);
            }
        }
    }

    fn pop_path_component(&mut self) {
        self.pop_element();
    }

    fn record(&mut self, change: record::Change) -> record::Action {
        use record::Change::*;
        self.records.push(match change {
            Addition {
                entry_mode,
                oid,
                path_id: _,
            } => Change::Addition {
                entry_mode,
                oid,
                path: self.path.deref().clone().into_path_buf_lossy(),
            },
            _ => todo!("record other kinds of changes"),
        });
        record::Action::Continue
    }
}
