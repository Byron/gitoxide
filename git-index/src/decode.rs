use crate::{entry, extension, Entry, State, Version};
use filetime::FileTime;
use git_hash::Kind;

pub mod header {
    pub(crate) const SIZE: usize = 4 /*signature*/ + 4 /*version*/ + 4 /* num entries */;

    mod error {
        use quick_error::quick_error;

        quick_error! {
            #[derive(Debug)]
            pub enum Error {
                Corrupt(message: &'static str) {
                    display("{}", message)
                }
                UnsupportedVersion(version: u32) {
                    display("Index version {} is not supported", version)
                }
            }
        }
    }
    use crate::{util::from_be_u32, Version};
    pub use error::Error;

    pub(crate) fn decode(data: &[u8], object_hash: git_hash::Kind) -> Result<(crate::Version, u32, &[u8]), Error> {
        if data.len() < (3 * 4) + object_hash.len_in_bytes() {
            return Err(Error::Corrupt(
                "File is too small even for header with zero entries and smallest hash",
            ));
        }

        const SIGNATURE: &[u8] = b"DIRC";
        let (signature, data) = data.split_at(4);
        if signature != SIGNATURE {
            return Err(Error::Corrupt(
                "Signature mismatch - this doesn't claim to be a header file",
            ));
        }

        let (version, data) = data.split_at(4);
        let version = match from_be_u32(version) {
            2 => Version::V2,
            3 => Version::V3,
            4 => Version::V4,
            unknown => return Err(Error::UnsupportedVersion(unknown)),
        };
        let (entries, data) = data.split_at(4);
        let entries = from_be_u32(entries);

        Ok((version, entries, data))
    }
}

mod error {
    use quick_error::quick_error;

    use crate::decode;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Header(err: decode::header::Error) {
                display("The header could not be decoded")
                source(err)
                from()
            }
        }
    }
}
use crate::util::{from_be_u32, split_at_pos};
pub use error::Error;

impl State {
    pub fn from_bytes(data: &[u8], timestamp: FileTime, object_hash: git_hash::Kind) -> Result<Self, Error> {
        let (version, num_entries, post_header_data) = header::decode(data, object_hash)?;
        let start_of_extensions = extension::end_of_index_entry::decode(data, object_hash);
        let mut ext = Extensions::default();

        // Note that we ignore all errors for optional signatures.
        match start_of_extensions {
            Some(offset) => {
                let (ext, entries) =
                    git_features::parallel::join(|| load_extensions(&data[offset..], object_hash), || ());
                todo!("load all extensions in thread, then get IEOT, then possibly multi-threaded entry parsing")
            }
            None => {
                let (entries, data) = load_entries(data, num_entries, object_hash, version)?;
                let ext = load_extensions(data, object_hash);
                todo!("load entries singlge-threaded, then extensions")
            }
        }

        Ok(State {
            timestamp,
            version,
            cache_tree: ext.cache_tree,
        })
    }
}

mod load_entries {
    use crate::Entry;

    pub struct Outcome {
        entries: Vec<Entry>,
        /// A memory area keeping all index paths, in full length, independently of the index version.
        path_backing: Vec<u8>,
        /// True if one entry in the index has a special marker mode
        is_sparse: bool,
    }
}

fn load_entries(
    beginning_of_entries: &[u8],
    num_entries: u32,
    object_hash: git_hash::Kind,
    version: Version,
) -> Result<(load_entries::Outcome, &[u8]), Error> {
    todo!("load entries")
}

fn decode_entry<'a>(
    data: &'a [u8],
    path_backing: &mut Vec<u8>,
    hash_len: usize,
    version: Version,
) -> Option<(Entry, &'a [u8])> {
    let (ctime_secs, data) = read_u32(data)?;
    let (ctime_nsecs, data) = read_u32(data)?;
    let (mtime_secs, data) = read_u32(data)?;
    let (mtime_nsecs, data) = read_u32(data)?;
    let (dev, data) = read_u32(data)?;
    let (ino, data) = read_u32(data)?;
    let (mode, data) = read_u32(data)?;
    let (uid, data) = read_u32(data)?;
    let (gid, data) = read_u32(data)?;
    let (size, data) = read_u32(data)?;
    let (hash, data) = split_at_pos(data, hash_len)?;
    let (flags, data) = read_u16(data)?;
    let flags = flags as u32;
    let (flags, data) = if flags & entry::flags::EXTENDED == entry::flags::EXTENDED {
        let (mut extended_flags, data) = read_u16(data)?;
        let extended_flags: u32 = (extended_flags as u32) << 16;
        const ALL_KNOWN_EXTENDED_FLAGS: u32 = entry::flags::INTENT_TO_ADD | entry::flags::SKIP_WORKTREE;
        assert_eq!(
            extended_flags & !ALL_KNOWN_EXTENDED_FLAGS,
            0,
            "BUG: encountered unknown extended bitflags in {:b}",
            extended_flags
        );
        (flags | extended_flags, data)
    } else {
        (flags, data)
    };

    let (path, data) = match version {
        Version::V2 | Version::V3 => {
            if (flags & entry::mask::PATH_LEN) == entry::mask::PATH_LEN {
                todo!("get to 0 byte and skip padding")
            } else {
                let path_len = (flags & entry::mask::PATH_LEN) as usize;
                let (path, data) = split_at_pos(data, path_len)?;

                let start = path_backing.len();
                path_backing.extend_from_slice(path);

                (start..path_backing.len(), skip_padding(data))
            }
        }
        Version::V4 => todo!("handle delta-paths"),
    };

    Some((
        Entry {
            stat: entry::Stat {
                ctime: entry::Time {
                    secs: ctime_secs,
                    nsecs: ctime_nsecs,
                },
                mtime: entry::Time {
                    secs: mtime_secs,
                    nsecs: mtime_nsecs,
                },
                dev,
                ino,
                mode,
                uid,
                gid,
                size,
            },
            id: git_hash::ObjectId::from(hash),
            flags: flags & !entry::mask::PATH_LEN,
        },
        data,
    ))
}

#[inline]
fn skip_padding(data: &[u8]) -> &[u8] {
    let foo = data.iter().filter(|b| **b == 0).count();
    todo!("continue")
}

#[inline]
fn read_u32(data: &[u8]) -> Option<(u32, &[u8])> {
    split_at_pos(data, 4).map(|(num, data)| (u32::from_be_bytes(num.try_into().unwrap()), data))
}
#[inline]
fn read_u16(data: &[u8]) -> Option<(u16, &[u8])> {
    split_at_pos(data, 4).map(|(num, data)| (u16::from_be_bytes(num.try_into().unwrap()), data))
}

fn load_extensions(beginning_of_extensions: &[u8], object_hash: git_hash::Kind) -> Extensions {
    extension::Iter::new_without_checksum(beginning_of_extensions, object_hash)
        .map(|extensions| {
            let mut ext = Extensions::default();
            for (signature, ext_data) in extensions {
                match signature {
                    extension::tree::SIGNATURE => {
                        ext.cache_tree = extension::tree::decode(ext_data, object_hash);
                    }
                    extension::end_of_index_entry::SIGNATURE => {} // skip already done
                    _unknown => {}                                 // skip unknown extensions, too
                }
            }
            ext
        })
        .unwrap_or_default()
}

#[derive(Default)]
struct Extensions {
    cache_tree: Option<extension::Tree>,
}
