use crate::{extension, State, Version};

pub struct Options {
    /// The hash kind to use when writing the index file.
    ///
    /// It is not always possible to infer the hash kind when reading an index, so this is required.
    pub hash_kind: git_hash::Kind,
    pub version: Version,
}

impl State {
    pub fn write_to(&self, mut out: impl std::io::Write, options: Options) -> std::io::Result<()> {
        let num_entries = self.entries().len() as u32;
        header(&mut out, options.version, num_entries)?;
        entries(&mut out, self)?;

        if let Some(t) = self.tree() {
            tree(&mut out, t)?;
        }

        // if num_entries > 0 {
        //     _end_of_index(&mut out, options.hash_kind)?;
        // }

        Ok(())
    }
}

fn header(out: &mut impl std::io::Write, version: Version, num_entries: u32) -> Result<(), std::io::Error> {
    let signature = b"DIRC";
    let version = match version {
        Version::V2 => 2_u32.to_be_bytes(),
        Version::V3 => 3_u32.to_be_bytes(),
        Version::V4 => 4_u32.to_be_bytes(),
    };
    out.write_all(signature)?;
    out.write_all(&version)?;
    out.write_all(&num_entries.to_be_bytes())?;
    Ok(())
}

fn entries(mut out: impl std::io::Write, state: &State) -> Result<(), std::io::Error> {
    for e in state.entries() {
        out.write_all(&e.stat.ctime.secs.to_be_bytes())?;
        out.write_all(&e.stat.ctime.nsecs.to_be_bytes())?;
        out.write_all(&e.stat.mtime.secs.to_be_bytes())?;
        out.write_all(&e.stat.mtime.nsecs.to_be_bytes())?;
        out.write_all(&e.stat.dev.to_be_bytes())?;
        out.write_all(&e.stat.ino.to_be_bytes())?;
        out.write_all(&e.mode.bits().to_be_bytes())?;
        out.write_all(&e.stat.uid.to_be_bytes())?;
        out.write_all(&e.stat.gid.to_be_bytes())?;
        out.write_all(&e.stat.size.to_be_bytes())?;
        out.write_all(e.id.as_bytes())?;
        let path = e.path(state);
        out.write_all(&(e.flags.to_storage().bits() | path.len() as u16).to_be_bytes())?;
        out.write_all(path)?;
        out.write_all(b"\0")?;

        // TODO: make this dynamic
        let size_of_previously_written_bytes = 63 + path.len();
        let byte_offset = match size_of_previously_written_bytes % 8 {
            0 => 0,
            n => 8 - n,
        };

        for _i in 0..byte_offset {
            out.write_all(b"\0")?;
        }
    }
    Ok(())
}

fn tree(out: &mut impl std::io::Write, tree: &extension::Tree) -> Result<(), std::io::Error> {
    let signature = b"TREE";

    // TODO: Can this work without allocating?
    let mut entries: Vec<u8> = Vec::new();
    tree_entry(&mut entries, tree)?;

    out.write_all(signature)?;
    out.write_all(&(entries.len() as u32).to_be_bytes())?;
    out.write_all(&entries)?;

    Ok(())
}

fn tree_entry(out: &mut impl std::io::Write, tree: &extension::Tree) -> Result<(), std::io::Error> {
    let num_entries_ascii = tree.num_entries.to_string();
    let num_children_ascii = tree.children.len().to_string();

    out.write_all(tree.name.as_slice())?;
    out.write_all(b"\0")?;
    out.write_all(num_entries_ascii.as_bytes())?;
    out.write_all(b" ")?;
    out.write_all(num_children_ascii.as_bytes())?;
    out.write_all(b"\n")?;
    out.write_all(tree.id.as_bytes())?;

    for child in &tree.children {
        tree_entry(out, child)?;
    }

    Ok(())
}

// TODO:
// - offset for the end of the index entries
// - active extensions and their content length for the hash
fn _end_of_index(out: &mut impl std::io::Write, hash_kind: git_hash::Kind) -> Result<(), std::io::Error> {
    let signature = b"EOIE";
    let extension_size = 4 + hash_kind.len_in_bytes() as u32;
    let offset: u32 = 0;

    let mut hasher = git_features::hash::hasher(hash_kind);

    hasher.update(b"TREE");
    hasher.update(&19_u32.to_be_bytes());
    let hash = hasher.digest();

    out.write_all(signature)?;
    out.write_all(&extension_size.to_be_bytes())?;
    out.write_all(&offset.to_be_bytes())?;
    out.write_all(&hash)?;

    Ok(())
}
