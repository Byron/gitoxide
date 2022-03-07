use std::io;
use std::io::ErrorKind::AlreadyExists;
use std::path::Path;

#[cfg(not(windows))]
pub fn create_symlink(original: &Path, link: &Path) -> io::Result<()> {
    std::os::unix::fs::symlink(original, link)
}

#[cfg(not(windows))]
pub fn remove_symlink(path: &Path) -> io::Result<()> {
    std::fs::remove_file(path)
}

#[cfg(windows)]
pub fn remove_symlink(path: &Path) -> io::Result<()> {
    symlink::remove_symlink_auto(path)
}

#[cfg(windows)]
pub fn create_symlink(original: &Path, link: &Path) -> io::Result<()> {
    use std::os::windows::fs::{symlink_dir, symlink_file};
    // TODO: figure out if links to links count as files or whatever they point at
    if std::fs::metadata(link.parent().expect("dir for link").join(original))?.is_dir() {
        symlink_dir(original, link)
    } else {
        symlink_file(original, link)
    }
}

#[cfg(not(windows))]
pub fn indicates_collision(err: &std::io::Error) -> bool {
    // TODO: use ::IsDirectory as well when stabilized instead of raw_os_error(), and ::FileSystemLoop respectively
    err.kind() == AlreadyExists
        || err.raw_os_error() == Some(21)
        || err.raw_os_error() == Some(62) // no-follow on symlnk on mac-os
        || err.raw_os_error() == Some(40) // no-follow on symlnk on ubuntu
}

#[cfg(windows)]
pub fn indicates_collision(err: &std::io::Error) -> bool {
    err.kind() == AlreadyExists || err.kind() == std::io::ErrorKind::PermissionDenied
}
