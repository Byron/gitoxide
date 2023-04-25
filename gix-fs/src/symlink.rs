use std::{io, io::ErrorKind::AlreadyExists, path::Path};

#[cfg(not(windows))]
/// Create a new symlink at `link` which points to `original`.
pub fn create(original: &Path, link: &Path) -> io::Result<()> {
    std::os::unix::fs::symlink(original, link)
}

#[cfg(not(windows))]
/// Remove a symlink.
///
/// Note that on only on windows this is special.
pub fn remove(path: &Path) -> io::Result<()> {
    std::fs::remove_file(path)
}

// TODO: use the `symlink` crate once it can delete directory symlinks
/// Remove a symlink.
#[cfg(windows)]
pub fn remove(path: &Path) -> io::Result<()> {
    if let Ok(meta) = std::fs::metadata(path) {
        if meta.is_file() {
            std::fs::remove_file(path) // this removes the link itself
        } else {
            std::fs::remove_dir(path) // however, this sees the destination directory, which isn't the right thing actually
        }
    } else {
        std::fs::remove_file(path).or_else(|_| std::fs::remove_dir(path))
    }
}

#[cfg(windows)]
/// Create a new symlink at `link` which points to `original`.
pub fn create(original: &Path, link: &Path) -> io::Result<()> {
    use std::os::windows::fs::{symlink_dir, symlink_file};
    // TODO: figure out if links to links count as files or whatever they point at
    if std::fs::metadata(link.parent().expect("dir for link").join(original))?.is_dir() {
        symlink_dir(original, link)
    } else {
        symlink_file(original, link)
    }
}

#[cfg(not(windows))]
/// Return true if `err` indicates that a file collision happened, i.e. a symlink couldn't be created as the `link`
/// already exists as filesystem object.
pub fn is_collision_error(err: &std::io::Error) -> bool {
    // TODO: use ::IsDirectory as well when stabilized instead of raw_os_error(), and ::FileSystemLoop respectively
    err.kind() == AlreadyExists
            || err.raw_os_error() == Some(21)
            || err.raw_os_error() == Some(62) // no-follow on symlnk on mac-os
            || err.raw_os_error() == Some(40) // no-follow on symlnk on ubuntu
}

#[cfg(windows)]
/// Return true if `err` indicates that a file collision happened, i.e. a symlink couldn't be created as the `link`
/// already exists as filesystem object.
pub fn is_collision_error(err: &std::io::Error) -> bool {
    err.kind() == AlreadyExists || err.kind() == std::io::ErrorKind::PermissionDenied
}
