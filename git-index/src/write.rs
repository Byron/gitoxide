use std::{
    collections::{hash_map, HashMap},
    ops::Range,
};

use bstr::ByteVec;

use crate::{extension, State, Version};

impl State {
    pub fn write_to(&self, options: Options) -> Vec<u8> {
        let mut writer = Writer::new(self, options);
        writer.generate();
        writer.data
    }
}

#[derive(Default)]
pub struct Options {
    hash_kind: git_hash::Kind,
}

struct Writer<'a> {
    state: &'a State,
    options: Options,
    data: Vec<u8>,
    index_table: HashMap<&'static str, Range<usize>>,
}

impl<'a> Writer<'a> {
    pub fn new(state: &'a State, options: Options) -> Self {
        Self {
            state,
            options,
            data: Vec::default(),
            index_table: Default::default(),
        }
    }

    pub fn generate(&mut self) {
        self.header();
        self.entries();

        // TODO: Tree extension is always included, I think
        if let Some(t) = self.state.tree() {
            self.tree(t)
        }

        self.end_of_index();
    }

    fn push(&mut self, data: &[u8], key: &'static str) {
        let start = self.data.len();
        let end = start + data.len();

        match self.index_table.entry(key) {
            hash_map::Entry::Occupied(mut e) => e.get_mut().end = end,
            hash_map::Entry::Vacant(e) => {
                e.insert(start..end);
            }
        };

        self.data.push_str(data);
    }

    fn header(&mut self) {
        let signature = b"DIRC";
        let version = match self.state.version() {
            Version::V2 => 2_u32.to_be_bytes(),
            Version::V3 => 3_u32.to_be_bytes(),
            Version::V4 => 4_u32.to_be_bytes(),
        };
        let num_entries = self.state.entries().len() as u32;

        self.push(signature, "header");
        self.push(&version, "header");
        self.push(&(num_entries).to_be_bytes(), "header");
    }

    fn entries(&mut self) {
        for e in self.state.entries() {
            self.push(&e.stat.ctime.secs.to_be_bytes(), "entries");
            self.push(&e.stat.ctime.nsecs.to_be_bytes(), "entries");
            self.push(&e.stat.mtime.secs.to_be_bytes(), "entries");
            self.push(&e.stat.mtime.nsecs.to_be_bytes(), "entries");
            self.push(&e.stat.dev.to_be_bytes(), "entries");
            self.push(&e.stat.ino.to_be_bytes(), "entries");
            self.push(&e.mode.bits().to_be_bytes(), "entries");
            self.push(&e.stat.uid.to_be_bytes(), "entries");
            self.push(&e.stat.gid.to_be_bytes(), "entries");
            self.push(&e.stat.size.to_be_bytes(), "entries");
            self.push(e.id.as_bytes(), "entries");
            //FIXME: correct flag values
            // probably convert 'in-memory' Flags to at_rest::Flags
            // self.push(&e.flags.bits().to_be_bytes(), "entries");
            self.push(b"\x00\x01\x61\x00", "entries");

            println!("{:?}", e.flags.bits());
        }
    }

    fn tree(&mut self, tree: &extension::Tree) {
        let signature = b"TREE";
        let mut size: u32 = 0;

        self.push(signature, "tree");
        self.push(&size.to_be_bytes(), "tree");

        self.tree_entry(tree);

        if let Some(range) = self.index_table.get("tree") {
            size = (range.end - (range.start + 8)) as u32;
            self.data[range.start + 4..range.start + 8].copy_from_slice(&size.to_be_bytes());
        }
    }

    fn tree_entry(&mut self, tree: &extension::Tree) {
        let path = [tree.name.as_slice(), b"\0"].concat();

        let num_entries_ascii = tree.num_entries.to_string();
        let num_children_ascii = tree.children.len().to_string();

        self.push(path.as_slice(), "tree");
        self.push(num_entries_ascii.as_bytes(), "tree");
        self.push(b" ", "tree");
        self.push(num_children_ascii.as_bytes(), "tree");
        self.push(b"\n", "tree");
        self.push(tree.id.as_bytes(), "tree");

        for child in &tree.children {
            self.tree_entry(child);
        }
    }

    fn end_of_index(&mut self) {
        match self.index_table.get("entries") {
            Some(range) => {
                let signature = b"EOIE";
                let extension_size = 4 + self.options.hash_kind.len_in_bytes() as u32;
                let offset: u32 = range.end as u32;

                let mut hasher = git_features::hash::hasher(self.options.hash_kind);

                match self.index_table.get("tree") {
                    Some(range) => {
                        hasher.update(b"TREE");
                        hasher.update(&self.data[range.start + 4..range.start + 8]);
                    }
                    None => {}
                }

                let hash = hasher.digest();

                self.data.push_str(signature);
                self.data.push_str(extension_size.to_be_bytes());
                self.data.push_str(offset.to_be_bytes());
                self.data.push_str(hash);
            }
            None => {}
        }
    }
}
