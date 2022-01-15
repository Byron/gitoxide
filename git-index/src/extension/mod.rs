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
}

mod iter;

pub(crate) mod decode;

pub(crate) mod tree;

pub(crate) mod end_of_index_entry;

pub(crate) mod index_entry_offset_table;

pub mod link;

pub(crate) mod resolve_undo;

pub mod untracked_cache {
    use crate::entry;
    use crate::extension::{Signature, UntrackedCache};
    use crate::util::{read_u32, split_at_byte_exclusive, split_at_pos};
    use git_hash::ObjectId;

    pub struct OidStat {
        pub mtime: entry::Time,
        pub ctime: entry::Time,
        pub dev: u32,
        pub ino: u32,
        pub uid: u32,
        pub gid: u32,
        /// The size of bytes on disk. Capped to u32 so files bigger than that will need thorough checking (and hopefully never make it)
        pub size: u32,
        pub id: ObjectId,
    }

    /// Only used as an indicator
    pub const SIGNATURE: Signature = *b"UNTR";

    #[allow(unused)]
    pub fn decode(data: &[u8], object_hash: git_hash::Kind) -> Option<UntrackedCache> {
        if !data.last().map(|b| *b == 0).unwrap_or(false) {
            return None;
        }
        let (identifier_len, consumed) = git_features::decode::leb64(data);
        let (_, data) = data.split_at(consumed);
        let (identifier, data) = split_at_pos(data, identifier_len.try_into().ok()?)?;
        dbg!(String::from_utf8_lossy(identifier));

        let hash_len = object_hash.len_in_bytes();
        let (info_exclude, data) = decode_oid_stat(data, hash_len)?;
        let (excludes_file, data) = decode_oid_stat(data, hash_len)?;
        let (dir_flags, data) = read_u32(data)?;
        let (exclude_filename_per_dir, data) = split_at_byte_exclusive(data, 0)?;
        dbg!(String::from_utf8_lossy(exclude_filename_per_dir));

        todo!("decode UNTR")
    }

    fn decode_oid_stat(data: &[u8], hash_len: usize) -> Option<(OidStat, &[u8])> {
        let (ctime_secs, data) = read_u32(data)?;
        let (ctime_nsecs, data) = read_u32(data)?;
        let (mtime_secs, data) = read_u32(data)?;
        let (mtime_nsecs, data) = read_u32(data)?;
        let (dev, data) = read_u32(data)?;
        let (ino, data) = read_u32(data)?;
        let (uid, data) = read_u32(data)?;
        let (gid, data) = read_u32(data)?;
        let (size, data) = read_u32(data)?;
        let (hash, data) = split_at_pos(data, hash_len)?;
        Some((
            OidStat {
                mtime: entry::Time {
                    secs: ctime_secs,
                    nsecs: ctime_nsecs,
                },
                ctime: entry::Time {
                    secs: mtime_secs,
                    nsecs: mtime_nsecs,
                },
                dev,
                ino,
                uid,
                gid,
                size,
                id: ObjectId::from(hash),
            },
            data,
        ))
    }
}

pub mod sparse {
    use crate::extension::Signature;

    /// Only used as an indicator
    pub const SIGNATURE: Signature = *b"sdir";
}
