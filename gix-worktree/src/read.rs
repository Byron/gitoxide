//! This module allows creating git blobs from worktree files.
//! For the most part a blob just contains the raw on-disk data.
//! However symlinks need to be considered poperly and attributes/config options need to be considered

use crate::fs;
use gix_object::Blob;
use gix_path as path;
use std::borrow::Cow;
use std::fs::{read_link, File};
use std::io::{self, Read};
use std::path::Path;

// TODO: tests

// TODO: what to do about precompose unicode and ignore_case for symlinks

/// Create a blob from a file or symlink.
pub fn blob(path: &Path, capabilities: &fs::Capabilities) -> io::Result<Blob> {
    let mut buf = Vec::new();
    let res = data_with_buf(path, &mut buf, capabilities)?;
    match res {
        Cow::Borrowed(_) => Ok(Blob { data: buf }),
        Cow::Owned(data) => Ok(Blob { data }),
    }
}

/// Create a blob from a file or symlink.
pub fn blob_with_meta(path: &Path, is_symlink: bool, capabilities: &fs::Capabilities) -> io::Result<Blob> {
    let mut buf = Vec::new();
    let res = data_with_buf_and_meta(path, &mut buf, is_symlink, capabilities)?;
    match res {
        Cow::Borrowed(_) => Ok(Blob { data: buf }),
        Cow::Owned(data) => Ok(Blob { data }),
    }
}

// TODO: there is no reason this should be a Cow
//       std isn't great about allowing users to avoid allocations but we could
//       simply write our own wrapper around libc::readlink which reuses the
//       buffer. This would require unsafe code tough (obviously)

/// Create blob data from a file or symlink.
pub fn data_with_buf<'a>(
    path: &Path,
    buf: &'a mut Vec<u8>,
    capabilities: &fs::Capabilities,
) -> io::Result<Cow<'a, [u8]>> {
    data_with_buf_and_meta(path, buf, path.symlink_metadata()?.is_symlink(), capabilities)
}

/// Create a blob from a file or symlink.
pub fn data_with_buf_and_meta<'a>(
    path: &Path,
    buf: &'a mut Vec<u8>,
    is_symlink: bool,
    capabilities: &fs::Capabilities,
) -> io::Result<Cow<'a, [u8]>> {
    // symlinks are only stored as actual symlinks if the FS supports it otherwise they are just
    // normal files with their content equal to the linked path (so can be read normally)
    //
    if is_symlink && capabilities.symlink {
        // conversion to bstr can never fail because symlinks are only used
        // on unix (by git) so no reason to use the try version here
        let symlink_path = path::into_bstr(read_link(path)?);
        Ok(Cow::Owned(symlink_path.into_owned().into()))
    } else {
        buf.clear();
        File::open(path)?.read_to_end(buf)?;
        // TODO apply filters
        Ok(buf.as_slice().into())
    }
}
