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
        mode: tree::EntryMode,
        oid: ObjectId,
        path: PathBuf,
    },
}

pub type Changes = Vec<Change>;

#[derive(Clone, Debug, Default)]
pub struct Recorder {
    current_component: BString,
    pub records: Vec<Change>,
}

impl Recorder {
    fn pop_element(&mut self) {
        if let Some(pos) = self.current_component.rfind_byte(b'/') {
            self.current_component.resize(pos, 0);
        }
    }

    fn push_element(&mut self, name: &BStr) {
        self.current_component.push(b'/');
        self.current_component.push_str(name);
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
            Addition { mode, oid } => Change::Addition {
                mode,
                oid,
                path: self.current_component.deref().clone().into_path_buf_lossy(),
            },
            _ => todo!("record other kinds of changes"),
        });
        record::Action::Continue
    }
}
