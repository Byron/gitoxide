use bstr::BString;
use git_hash::ObjectId;

use crate::{
    entry,
    extension::{Signature, UntrackedCache},
    util::{read_u32, split_at_byte_exclusive, split_at_pos, var_int},
};

pub struct OidStat {
    pub stat: entry::Stat,
    pub id: ObjectId,
}

/// A directory with information about its untracked files, and its sub-directories
pub struct Directory {
    /// The directories name, or an empty string if this is the root directory.
    pub name: BString,
    /// Untracked files and directory names
    pub untracked_entries: Vec<BString>,
    /// indices for sub-directories similar to this one.
    pub sub_directories: Vec<usize>,

    /// The directories stat data, if available or valid // TODO: or is it the exclude file?
    pub stat: Option<OidStat>,
    /// The oid of a .gitignore file, if it exists
    pub exclude_file_oid: Option<ObjectId>,
    /// TODO: figure out what this really does
    pub check_only: bool,
}

/// The first entry in the list of flattened directories is the this root directory itself.
pub struct RootDirectory {
    ///
    flattened_directories: Vec<Directory>,
}

/// Only used as an indicator
pub const SIGNATURE: Signature = *b"UNTR";

#[allow(unused)]
pub fn decode(data: &[u8], object_hash: git_hash::Kind) -> Option<UntrackedCache> {
    if !data.last().map(|b| *b == 0).unwrap_or(false) {
        return None;
    }
    let (identifier_len, data) = var_int(data)?;
    let (identifier, data) = split_at_pos(data, identifier_len.try_into().ok()?)?;

    let hash_len = object_hash.len_in_bytes();
    let (info_exclude, data) = decode_oid_stat(data, hash_len)?;
    let (excludes_file, data) = decode_oid_stat(data, hash_len)?;
    let (dir_flags, data) = read_u32(data)?;
    let (exclude_filename_per_dir, data) = split_at_byte_exclusive(data, 0)?;

    let (num_directory_blocks, data) = var_int(data)?;

    let mut res = UntrackedCache {
        identifier: identifier.into(),
        info_exclude: (!info_exclude.id.is_null()).then(|| info_exclude),
        excludes_file: (!excludes_file.id.is_null()).then(|| excludes_file),
        exclude_filename_per_dir: exclude_filename_per_dir.into(),
        dir_flags,
    };
    if num_directory_blocks == 0 {
        return data.is_empty().then(|| res);
    }

    let num_directory_blocks = num_directory_blocks.try_into().ok()?;
    let mut directories = Vec::<Directory>::with_capacity(num_directory_blocks);

    let data = decode_directory_block(data, &mut directories)?;
    if directories.len() != num_directory_blocks {
        return None;
    }
    let root_dir = RootDirectory {
        flattened_directories: directories,
    };

    let (valid, data) = git_bitmap::ewah::decode(data).ok()?;
    let (check_only, data) = git_bitmap::ewah::decode(data).ok()?;
    let (hash_valid, data) = git_bitmap::ewah::decode(data).ok()?;

    todo!("decode UNTR")
}

fn decode_directory_block<'a>(data: &'a [u8], directories: &mut Vec<Directory>) -> Option<&'a [u8]> {
    let (num_untracked, data) = var_int(data)?;
    let (num_dirs, data) = var_int(data)?;
    let (name, mut data) = split_at_byte_exclusive(data, 0)?;
    let mut untracked_entries = Vec::<BString>::with_capacity(num_untracked.try_into().ok()?);
    for _ in 0..num_untracked {
        let (name, rest) = split_at_byte_exclusive(data, 0)?;
        data = rest;
        untracked_entries.push(name.into());
    }

    let index = directories.len();
    directories.push(Directory {
        name: name.into(),
        untracked_entries,
        sub_directories: Vec::with_capacity(num_dirs.try_into().ok()?),
        // the following are set later through their bitmaps
        stat: None,
        exclude_file_oid: None,
        check_only: false,
    });

    for _ in 0..num_dirs {
        let subdir_index = directories.len();
        let rest = decode_directory_block(data, directories)?;
        data = rest;
        directories[index].sub_directories.push(subdir_index);
    }

    data.into()
}

fn decode_oid_stat(data: &[u8], hash_len: usize) -> Option<(OidStat, &[u8])> {
    let (stat, data) = crate::decode::stat(data)?;
    let (hash, data) = split_at_pos(data, hash_len)?;
    Some((
        OidStat {
            stat,
            id: ObjectId::from(hash),
        },
        data,
    ))
}
