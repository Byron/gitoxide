use gix_hash::ObjectId;
use gix_object::{
    bstr::{BStr, BString, ByteSlice, ByteVec},
    tree,
};

use crate::tree::{visit, Recorder};

/// Describe how to track the location of a change.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Location {
    /// Track the entire path, relative to the repository.
    Path,
    /// Keep only the file-name as location, which may be enough for some calculations.
    ///
    /// This is less expensive than tracking the entire `Path`.
    FileName,
}

/// A Change as observed by a call to [`visit(…)`][visit::Visit::visit()], enhanced with the path affected by the change.
/// Its similar to [`visit::Change`] but includes the path that changed.
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

impl Default for Recorder {
    fn default() -> Self {
        Recorder {
            path_deque: Default::default(),
            path: Default::default(),
            location: Some(Location::Path),
            records: vec![],
        }
    }
}

/// Builder
impl Recorder {
    /// Obtain a copy of the currently tracked, full path of the entry.
    pub fn track_location(mut self, location: Option<Location>) -> Self {
        self.location = location;
        self
    }
}

/// Access
impl Recorder {
    /// Obtain a copy of the currently tracked, full path of the entry.
    pub fn path_clone(&self) -> BString {
        self.path.clone()
    }

    /// Return the currently set path.
    pub fn path(&self) -> &BStr {
        self.path.as_ref()
    }
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
}

impl visit::Visit for Recorder {
    fn pop_front_tracked_path_and_set_current(&mut self) {
        if let Some(Location::Path) = self.location {
            self.path = self.path_deque.pop_front().expect("every parent is set only once");
        }
    }

    fn push_back_tracked_path_component(&mut self, component: &BStr) {
        if let Some(Location::Path) = self.location {
            self.push_element(component);
            self.path_deque.push_back(self.path.clone());
        }
    }

    fn push_path_component(&mut self, component: &BStr) {
        match self.location {
            None => {}
            Some(Location::Path) => {
                self.push_element(component);
            }
            Some(Location::FileName) => {
                self.path.clear();
                self.path.extend_from_slice(component);
            }
        }
    }

    fn pop_path_component(&mut self) {
        if let Some(Location::Path) = self.location {
            self.pop_element();
        }
    }

    fn visit(&mut self, change: visit::Change) -> visit::Action {
        self.records.push(match change {
            visit::Change::Deletion { entry_mode, oid } => Change::Deletion {
                entry_mode,
                oid,
                path: self.path_clone(),
            },
            visit::Change::Addition { entry_mode, oid } => Change::Addition {
                entry_mode,
                oid,
                path: self.path_clone(),
            },
            visit::Change::Modification {
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
