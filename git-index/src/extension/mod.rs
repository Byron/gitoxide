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
/// if there wa sno change to them. Portions of this tree are invalidated as the index is changed.
pub struct Tree {
    name: SmallVec<[u8; 23]>,
    /// Only set if there are any entries in the index we are associated with.
    id: Option<tree::NodeId>,
    children: Vec<Tree>,
}

pub struct Link {
    pub shared_index_checksum: git_hash::ObjectId,
    pub bitmaps: Option<link::Bitmaps>,
}

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

mod iter;

pub(crate) mod decode;

pub(crate) mod tree;

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
