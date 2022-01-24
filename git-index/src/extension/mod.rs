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
    /// Only set if there are any entries in the index we are associated with.
    pub id: Option<tree::NodeId>,
    pub children: Vec<Tree>,
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

pub struct FsMonitor {
    token: fs_monitor::Token,
    /// if a bit is true, the resepctive entry is NOT valid as per the fs monitor.
    entry_dirty: git_bitmap::ewah::Vec,
}

mod iter;

pub(crate) mod fs_monitor {
    use bstr::BString;

    use crate::{
        extension::{FsMonitor, Signature},
        util::{read_u32, read_u64, split_at_byte_exclusive},
    };

    pub enum Token {
        V1 { nanos_since_1970: u64 },
        V2 { token: BString },
    }

    pub const SIGNATURE: Signature = *b"FSMN";

    pub fn decode(data: &[u8]) -> Option<FsMonitor> {
        let (version, data) = read_u32(data)?;
        let (token, data) = match version {
            1 => {
                let (nanos_since_1970, data) = read_u64(data)?;
                (Token::V1 { nanos_since_1970 }, data)
            }
            2 => {
                let (token, data) = split_at_byte_exclusive(data, 0)?;
                (Token::V2 { token: token.into() }, data)
            }
            _ => return None,
        };

        let (ewah_size, data) = read_u32(data)?;
        let (entry_dirty, data) = git_bitmap::ewah::decode(&data[..ewah_size as usize]).ok()?;

        if !data.is_empty() {
            return None;
        }

        FsMonitor { token, entry_dirty }.into()
    }
}

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
