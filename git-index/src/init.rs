use crate::{
    entry::{Flags, Mode, Stat},
    Entry, PathStorage, State, Version,
};
use bstr::{BStr, BString, ByteSlice, ByteVec};
use git_object::{
    tree::{self, EntryMode},
    TreeRefIter,
};
use git_traverse::tree::{breadthfirst, visit::Action, Visit};
use std::collections::VecDeque;

/// initialization
impl State {
    /// Takes in an oid of a tree object and creates and returns a [`State`][git_index::State] from its children.
    pub fn from_tree<Find>(tree: &git_hash::oid, mut find: Find) -> Result<Self, breadthfirst::Error>
    where
        Find: for<'a> FnMut(&git_hash::oid, &'a mut Vec<u8>) -> Option<TreeRefIter<'a>>,
    {
        let mut buf = Vec::new();
        let root = find(tree, &mut buf).expect("couldn't find a tree for given oid");
        let state = breadthfirst::State::default();
        let mut delegate = EntryBuilder::new();
        breadthfirst(root, state, &mut find, &mut delegate)?;

        Ok(State {
            timestamp: filetime::FileTime::now(),
            version: Version::V2,
            entries: delegate.entries,
            path_backing: delegate.path_backing,
            is_sparse: false,
            tree: None,
            link: None,
            resolve_undo: None,
            untracked: None,
            fs_monitor: None,
        })
    }
}

struct EntryBuilder {
    entries: Vec<Entry>,
    path_backing: PathStorage,
    path: BString,
    path_deque: VecDeque<BString>,
}

impl EntryBuilder {
    pub fn new() -> EntryBuilder {
        EntryBuilder {
            entries: Vec::new(),
            path_backing: Vec::new(),
            path: BString::default(),
            path_deque: VecDeque::new(),
        }
    }

    fn push_element(&mut self, name: &BStr) {
        if !self.path.is_empty() {
            self.path.push(b'/');
        }
        self.path.push_str(name);
    }

    pub fn add_entry(&mut self, entry: &tree::EntryRef<'_>) {
        let mode = match entry.mode {
            EntryMode::Tree => unreachable!("visit_non_tree() called us"),
            EntryMode::Blob => Mode::FILE,
            EntryMode::BlobExecutable => Mode::FILE_EXECUTABLE,
            EntryMode::Link => Mode::SYMLINK,
            EntryMode::Commit => Mode::COMMIT,
        };

        let path_start = self.path_backing.len();
        self.path_backing.extend_from_slice(&self.path);

        let new_entry = Entry {
            stat: Stat::default(),
            id: entry.oid.into(),
            flags: Flags::empty(),
            mode,
            path: path_start..self.path_backing.len(),
        };

        match self
            .entries
            .binary_search_by(|entry| Entry::cmp_filepaths(entry.path_in(&self.path_backing), self.path.as_bstr()))
        {
            Ok(pos) => self.entries[pos] = new_entry,
            Err(pos) => self.entries.insert(pos, new_entry),
        };
    }
}

impl Visit for EntryBuilder {
    fn pop_front_tracked_path_and_set_current(&mut self) {
        self.path = self
            .path_deque
            .pop_front()
            .expect("every call is matched with push_tracked_path_component");
    }

    fn push_back_tracked_path_component(&mut self, component: &bstr::BStr) {
        self.push_element(component);
        self.path_deque.push_back(self.path.clone());
    }

    fn push_path_component(&mut self, component: &bstr::BStr) {
        self.push_element(component);
    }

    fn pop_path_component(&mut self) {
        if let Some(pos) = self.path.rfind_byte(b'/') {
            self.path.resize(pos, 0);
        } else {
            self.path.clear();
        }
    }

    fn visit_tree(&mut self, _entry: &git_object::tree::EntryRef<'_>) -> git_traverse::tree::visit::Action {
        Action::Continue
    }

    fn visit_nontree(&mut self, entry: &git_object::tree::EntryRef<'_>) -> git_traverse::tree::visit::Action {
        self.add_entry(entry);
        Action::Continue
    }
}
