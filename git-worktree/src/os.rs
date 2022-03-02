use std::io;
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
