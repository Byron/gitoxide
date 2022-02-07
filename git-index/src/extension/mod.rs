use bstr::BString;
use smallvec::SmallVec;

const MIN_SIZE: usize = 4 /* signature */ + 4 /* size */;

pub type Signature = [u8; 4];

pub struct Iter<'a> {
    data: &'a [u8],
    pub consumed: usize,
}

/// A structure to associate object ids of a tree with sections in the index entries list.
///
/// It allows to more quickly build trees by avoiding as it can quickly re-use portions of the index and its associated tree ids
/// if there was no change to them. Portions of this tree are invalidated as the index is changed.
pub struct Tree {
    pub name: SmallVec<[u8; 23]>,
    /// The id of the directory tree of the associated tree object.
    pub id: git_hash::ObjectId,
    /// The amount of non-tree items in this directory tree, including sub-trees, recursively.
    /// The value of the top-level tree is thus equal to the value of the total amount of entries.
    pub num_entries: u32,
    pub children: Vec<Tree>,
}

pub struct Link {
    pub shared_index_checksum: git_hash::ObjectId,
    pub bitmaps: Option<link::Bitmaps>,
}

#[allow(dead_code)]
pub struct UntrackedCache {
    /// Something identifying the location and machine that this cache is for.
    /// Should the repository be copied to a different machine, the entire cache can immediately be invalidated.
    identifier: BString,
    /// Stat for the .git/info/exclude file
    info_exclude: Option<untracked_cache::OidStat>,
    /// Stat for the `core.excludesfile`
    excludes_file: Option<untracked_cache::OidStat>,
    /// Usually `.gitignore`
    exclude_filename_per_dir: BString,
    dir_flags: u32,

    /// A list of directories and sub-directories, with `directories[0]` being the root.
    directories: Vec<untracked_cache::Directory>,
}

#[allow(dead_code)]
pub struct FsMonitor {
    token: fs_monitor::Token,
    /// if a bit is true, the resepctive entry is NOT valid as per the fs monitor.
    entry_dirty: git_bitmap::ewah::Vec,
}

mod iter;

pub(crate) mod fs_monitor;

pub(crate) mod decode;

pub mod tree;

pub(crate) mod end_of_index_entry;

pub(crate) mod index_entry_offset_table;

pub mod link;

pub(crate) mod resolve_undo;

pub mod untracked_cache;

pub mod sparse {
    use crate::extension::Signature;

    /// Only used as an indicator
    pub const SIGNATURE: Signature = *b"sdir";
}
