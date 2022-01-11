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
            Entry(index: u32) {
                display("Could not parse entry at index {}", index)
            }
            UnexpectedTrailerLength { expected: usize, actual: usize } {
                display("Index trailer should have been {} bytes long, but was {}", expected, actual)
            }
        }
    }
}
use crate::util::{from_be_u32, split_at_byte_exclusive, split_at_pos};
pub use error::Error;

impl State {
    pub fn from_bytes(
        data: &[u8],
        timestamp: FileTime,
        object_hash: git_hash::Kind,
    ) -> Result<(Self, git_hash::ObjectId), Error> {
        let (version, num_entries, post_header_data) = header::decode(data, object_hash)?;
        let start_of_extensions = extension::end_of_index_entry::decode(data, object_hash);

        let path_backing_buffer_size = load_entries::estimate_path_storage_requirements_in_bytes(
            num_entries,
            data.len(),
            start_of_extensions,
            object_hash,
            version,
        );
        let (entries, ext, data) = match start_of_extensions {
            Some(offset) => {
                let (entries_res, (ext, data)) = git_features::parallel::join(
                    // TODO load all extensions in thread, then get IEOT, then possibly multi-threaded entry parsing
                    || {
                        load_entries(
                            post_header_data,
                            num_entries,
                            path_backing_buffer_size,
                            object_hash,
                            version,
                        )
                    },
                    || load_extensions(&data[offset..], object_hash),
                );
                (entries_res?.0, ext, data)
            }
            None => {
                let (entries, data) = load_entries(
                    post_header_data,
                    num_entries,
                    path_backing_buffer_size,
                    object_hash,
                    version,
                )?;
                let (ext, data) = load_extensions(data, object_hash);
                (entries, ext, data)
            }
        };

        if data.len() != object_hash.len_in_bytes() {
            return Err(Error::UnexpectedTrailerLength {
                expected: object_hash.len_in_bytes(),
                actual: data.len(),
            });
        }

        let checksum = git_hash::ObjectId::from(data);
        let load_entries::Outcome {
            entries,
            path_backing,
            is_sparse,
        } = entries;
        let Extensions { cache_tree } = ext;

        Ok((
            State {
                timestamp,
                version,
                cache_tree,
                entries,
                path_backing,
                is_sparse,
            },
            checksum,
        ))
    }
}

mod load_entries {
    use crate::decode::header;
    use crate::{Entry, Version};

    pub struct Outcome {
        pub entries: Vec<Entry>,
        pub path_backing: Vec<u8>,
        pub is_sparse: bool,
    }

    pub fn estimate_path_storage_requirements_in_bytes(
        num_entries: u32,
        on_disk_size: usize,
        offset_to_extensions: Option<usize>,
        object_hash: git_hash::Kind,
        version: Version,
    ) -> usize {
        const fn on_disk_entry_sans_path(object_hash: git_hash::Kind) -> usize {
            8 + // ctime
            8 + // mtime
            (4 * 6) +  // various stat fields
            2 + // flag, ignore extended flag as we'd rather overallocate a bit
            object_hash.len_in_bytes()
        };
        match version {
            Version::V3 | Version::V2 => {
                let size_of_entries_block = offset_to_extensions.unwrap_or(on_disk_size);
                size_of_entries_block
                    .saturating_sub(num_entries as usize * on_disk_entry_sans_path(object_hash))
                    .saturating_sub(header::SIZE)
            }
            Version::V4 => num_entries as usize * 80, /* a guess directly from git sources */
        }
    }
}

/// Note that `data` must point to the beginning of the entries, right past the header.
fn load_entries(
    mut data: &[u8],
    num_entries: u32,
    path_backing_capacity: usize,
    object_hash: git_hash::Kind,
    version: Version,
) -> Result<(load_entries::Outcome, &[u8]), Error> {
    let mut path_backing = Vec::<u8>::with_capacity(path_backing_capacity);
    let mut entries = Vec::with_capacity(num_entries as usize);
    let mut is_sparse = false;
    for idx in 0..num_entries {
        let (entry, remaining) =
            decode_entry(data, &mut path_backing, object_hash.len_in_bytes(), version).ok_or(Error::Entry(idx))?;
        data = remaining;
        if entry::mode::is_sparse(entry.stat.mode) {
            is_sparse = true;
        }
        // TODO: entries are actually in an intrusive collection, with path as key. Could be set for us. This affects 'ignore_case' which we
        //       also don't yet handle but probably could, maybe even smartly with the collection.
        //       For now it's unclear to me how they access the index, they could iterate quickly, and have fast access by path.
        entries.push(entry);
    }

    Ok((
        load_entries::Outcome {
            entries,
            path_backing,
            is_sparse,
        },
        data,
    ))
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
            let (path, data) = if (flags & entry::mask::PATH_LEN) == entry::mask::PATH_LEN {
                split_at_byte_exclusive(data, 0)?
            } else {
                let path_len = (flags & entry::mask::PATH_LEN) as usize;
                split_at_pos(data, path_len)?
            };

            (path, skip_padding(data))
        }
        Version::V4 => todo!("handle delta-paths"),
    };

    let path = {
        let start = path_backing.len();
        path_backing.extend_from_slice(path);
        start..path_backing.len()
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
    let skip = data.iter().take_while(|b| **b == 0).count();
    &data[skip..]
}

#[inline]
fn read_u32(data: &[u8]) -> Option<(u32, &[u8])> {
    split_at_pos(data, 4).map(|(num, data)| (u32::from_be_bytes(num.try_into().unwrap()), data))
}
#[inline]
fn read_u16(data: &[u8]) -> Option<(u16, &[u8])> {
    split_at_pos(data, 2).map(|(num, data)| (u16::from_be_bytes(num.try_into().unwrap()), data))
}

fn load_extensions(beginning_of_extensions: &[u8], object_hash: git_hash::Kind) -> (Extensions, &[u8]) {
    extension::Iter::new_without_checksum(beginning_of_extensions, object_hash)
        .map(|mut ext_iter| {
            let mut ext = Extensions::default();
            for (signature, ext_data) in ext_iter.by_ref() {
                match signature {
                    extension::tree::SIGNATURE => {
                        ext.cache_tree = extension::tree::decode(ext_data, object_hash);
                    }
                    extension::end_of_index_entry::SIGNATURE => {} // skip already done
                    _unknown => {}                                 // skip unknown extensions, too
                }
            }
            (ext, &beginning_of_extensions[ext_iter.consumed..])
        })
        .unwrap_or_else(|| (Extensions::default(), beginning_of_extensions))
}

#[derive(Default)]
struct Extensions {
    cache_tree: Option<extension::Tree>,
}
