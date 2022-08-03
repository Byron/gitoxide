use crate::{extension, State, Version};
use std::io::Write;

pub struct Options {
    /// The hash kind to use when writing the index file.
    ///
    /// It is not always possible to infer the hash kind when reading an index, so this is required.
    pub hash_kind: git_hash::Kind,
    pub version: Version,
    pub tree_cache: bool,
    pub end_of_index_entry: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            hash_kind: git_hash::Kind::default(),
            version: Version::V2,
            tree_cache: true,
            end_of_index_entry: true,
        }
    }
}

impl State {
    pub fn write_to(&self, out: &mut impl std::io::Write, options: Options) -> std::io::Result<()> {
        let mut write_counter = WriteCounter::new(out);
        let num_entries = self.entries().len() as u32;
        let header_offset = header(&mut write_counter, options.version, num_entries)?;
        let entries_offset = entries(&mut write_counter, self, header_offset)?;
        let tree_offset = if options.tree_cache {
            tree(&mut write_counter, self.tree())?
        } else {
            entries_offset
        };

        if num_entries > 0 && options.end_of_index_entry {
            end_of_index_entry(write_counter.inner, options.hash_kind, entries_offset, tree_offset)?;
        }

        Ok(())
    }
}

fn header<T: std::io::Write>(
    out: &mut WriteCounter<'_, T>,
    version: Version,
    num_entries: u32,
) -> Result<u32, std::io::Error> {
    let signature = b"DIRC";

    let version = match version {
        Version::V2 => 2_u32.to_be_bytes(),
        Version::V3 => 3_u32.to_be_bytes(),
        Version::V4 => 4_u32.to_be_bytes(),
    };

    out.write_all(signature)?;
    out.write_all(&version)?;
    out.write_all(&num_entries.to_be_bytes())?;

    Ok(out.count)
}

fn entries<T: std::io::Write>(
    out: &mut WriteCounter<'_, T>,
    state: &State,
    header_size: u32,
) -> Result<u32, std::io::Error> {
    for entry in state.entries() {
        out.write_all(&entry.stat.ctime.secs.to_be_bytes())?;
        out.write_all(&entry.stat.ctime.nsecs.to_be_bytes())?;
        out.write_all(&entry.stat.mtime.secs.to_be_bytes())?;
        out.write_all(&entry.stat.mtime.nsecs.to_be_bytes())?;
        out.write_all(&entry.stat.dev.to_be_bytes())?;
        out.write_all(&entry.stat.ino.to_be_bytes())?;
        out.write_all(&entry.mode.bits().to_be_bytes())?;
        out.write_all(&entry.stat.uid.to_be_bytes())?;
        out.write_all(&entry.stat.gid.to_be_bytes())?;
        out.write_all(&entry.stat.size.to_be_bytes())?;
        out.write_all(entry.id.as_bytes())?;
        let path = entry.path(state);
        out.write_all(&(entry.flags.to_storage().bits() | path.len() as u16).to_be_bytes())?;
        out.write_all(path)?;
        out.write_all(b"\0")?;

        match (out.count - header_size) % 8 {
            0 => {}
            n => {
                let byte_offset = 8 - n;
                for _ in 0..byte_offset {
                    out.write_all(b"\0")?;
                }
            }
        };
    }

    Ok(out.count)
}

fn tree<T: std::io::Write>(
    out: &mut WriteCounter<'_, T>,
    tree: Option<&extension::Tree>,
) -> Result<u32, std::io::Error> {
    if let Some(tree) = tree {
        let signature = b"TREE";

        // TODO: Can this work without allocating?
        let mut entries: Vec<u8> = Vec::new();
        tree_entry(&mut entries, tree)?;

        out.write_all(signature)?;
        out.write_all(&(entries.len() as u32).to_be_bytes())?;
        out.write_all(&entries)?;
    }

    Ok(out.count)
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

fn end_of_index_entry(
    out: &mut impl std::io::Write,
    hash_kind: git_hash::Kind,
    entries_offset: u32,
    tree_offset: u32,
) -> Result<(), std::io::Error> {
    let signature = b"EOIE";
    let extension_size = 4 + hash_kind.len_in_bytes() as u32;

    let mut hasher = git_features::hash::hasher(hash_kind);
    let tree_size = tree_offset - 8 - entries_offset;
    if tree_size > 0 {
        hasher.update(b"TREE");
        hasher.update(&tree_size.to_be_bytes());
    }
    let hash = hasher.digest();

    out.write_all(signature)?;
    out.write_all(&extension_size.to_be_bytes())?;
    out.write_all(&entries_offset.to_be_bytes())?;
    out.write_all(&hash)?;

    Ok(())
}

struct WriteCounter<'a, T> {
    count: u32,
    inner: &'a mut T,
}

impl<'a, T> WriteCounter<'a, T>
where
    T: std::io::Write,
{
    pub fn new(inner: &'a mut T) -> Self {
        WriteCounter { inner, count: 0 }
    }
}

impl<'a, T> std::io::Write for WriteCounter<'a, T>
where
    T: std::io::Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let written = self.inner.write(buf)?;
        self.count += written as u32;
        Ok(written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
