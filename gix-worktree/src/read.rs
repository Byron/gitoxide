//! This module allows creating git blobs from worktree files.
//!
//! For the most part a blob just contains the raw on-disk data. However symlinks need to be considered properly
//! and attributes/config options need to be considered.

use std::{
    fs::{read_link, File},
    io::{self, Read},
    path::Path,
};

use gix_object::Blob;
use gix_path as path;

// TODO: tests

// TODO: what to do about precompose unicode and ignore_case for symlinks

/// Create a blob from a file or symlink.
pub fn blob(path: &Path, capabilities: &gix_fs::Capabilities) -> io::Result<Blob> {
    let mut data = Vec::new();
    data_to_buf(path, &mut data, capabilities)?;
    Ok(Blob { data })
}

/// Create a blob from a file or symlink.
pub fn blob_with_meta(path: &Path, is_symlink: bool, capabilities: &gix_fs::Capabilities) -> io::Result<Blob> {
    let mut data = Vec::new();
    data_to_buf_with_meta(path, &mut data, is_symlink, capabilities)?;
    Ok(Blob { data })
}

/// Create blob data from a file or symlink.
pub fn data_to_buf<'a>(path: &Path, buf: &'a mut Vec<u8>, capabilities: &gix_fs::Capabilities) -> io::Result<&'a [u8]> {
    data_to_buf_with_meta(path, buf, path.symlink_metadata()?.is_symlink(), capabilities)
}

/// Create a blob from a file or symlink.
pub fn data_to_buf_with_meta<'a>(
    path: &Path,
    buf: &'a mut Vec<u8>,
    is_symlink: bool,
    capabilities: &gix_fs::Capabilities,
) -> io::Result<&'a [u8]> {
    buf.clear();
    // symlinks are only stored as actual symlinks if the FS supports it otherwise they are just
    // normal files with their content equal to the linked path (so can be read normally)
    //
    if is_symlink && capabilities.symlink {
        // conversion to bstr can never fail because symlinks are only used
        // on unix (by git) so no reason to use the try version here
        let symlink_path = path::into_bstr(read_link(path)?);
        buf.extend_from_slice(&symlink_path);
        // TODO: there is no reason this should be a clone
        //       std isn't great about allowing users to avoid allocations but we could
        //       simply write our own wrapper around libc::readlink which reuses the
        //       buffer. This would require unsafe code tough (obviously)
    } else {
        buf.clear();
        File::open(path)?.read_to_end(buf)?;
        // TODO apply filters
    }
    Ok(buf.as_slice())
}
