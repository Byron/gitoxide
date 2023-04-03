use crate::fs;
use gix_object::Blob;
use gix_path as path;
use std::borrow::Cow;
use std::fs::{read_link, File};
use std::io;
use std::io::Read;
use std::path::Path;

// TODO: tests
// TODO: module level docs to explain why this would be needed (e.g. symlinks + filters)

/// Error returned by [`blob()`] and related functions.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Could not convert symlink path to UTF8")]
    IllformedUtf8,
    #[error("IO error while reading blob")]
    Io(#[from] io::Error),
}

// TODO: what to do about precompose unicode and ignore case_here?

/// Create a blob from a file or symlink.
pub fn blob(path: &Path, capabilities: &fs::Capabilities) -> Result<Blob, Error> {
    let mut buf = Vec::new();
    let res = data_with_buf(path, &mut buf, capabilities)?;
    match res {
        Cow::Borrowed(_) => Ok(Blob { data: buf }),
        Cow::Owned(data) => Ok(Blob { data }),
    }
}

/// Create a blob from a file or symlink.
pub fn blob_with_meta(path: &Path, is_symlink: bool, capabilities: &fs::Capabilities) -> Result<Blob, Error> {
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
) -> Result<Cow<'a, [u8]>, Error> {
    data_with_buf_and_meta(path, buf, path.symlink_metadata()?.is_symlink(), capabilities)
}

/// Create a blob from a file or symlink.
pub fn data_with_buf_and_meta<'a>(
    path: &Path,
    buf: &'a mut Vec<u8>,
    is_symlink: bool,
    capabilities: &fs::Capabilities,
) -> Result<Cow<'a, [u8]>, Error> {
    // symlinks are only stored as actual symlinks if the FS supports it otherwise they are just
    // normal files with their content equal to the linked path (so can be read normally)
    //
    if is_symlink && capabilities.symlink {
        let symlink_path = path::try_into_bstr(read_link(path)?).map_err(|_| Error::IllformedUtf8)?;
        Ok(Cow::Owned(
            path::to_unix_separators_on_windows(symlink_path).into_owned().into(),
        ))
    } else {
        buf.clear();
        File::open(path)?.read_to_end(buf)?;
        // TODO apply filters
        Ok(buf.as_slice().into())
    }
}
