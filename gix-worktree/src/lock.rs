use std::{fs, io, io::Write, path::Path};

use bstr::BStr;

/// Set or clear the lock of a linked worktree in its existing private `git_dir`.
///
/// `Some(reason)` creates or updates the `locked` file, preserving its bytes and appending a newline
/// if needed, like Git. An empty reason locks without an explanation. `None` removes the lock file;
/// an already absent lock is considered unlocked. Missing administrative directories are not created.
///
/// On Unix and Windows, locking rejects a symbolic link at the `locked` path; unlocking removes the
/// link itself. Symbolic links in parent components are followed. Updating a reason keeps the lock
/// file present, but callers must serialize concurrent changes to the lock and its reason.
pub fn set_lock(git_dir: &Path, reason: Option<&BStr>) -> io::Result<()> {
    let path = git_dir.join("locked");
    match reason {
        Some(reason) => {
            match path.symlink_metadata() {
                Ok(metadata) if !metadata.is_file() => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "worktree lock is not a regular file",
                    ));
                }
                Err(err) if err.kind() != io::ErrorKind::NotFound => return Err(err),
                _ => {}
            }
            let mut file = gix_fs::open_options_no_follow().write(true).create(true).open(path)?;
            if !file.metadata()?.is_file() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "worktree lock is not a regular file",
                ));
            }
            file.set_len(0)?;
            file.write_all(reason)?;
            if !reason.ends_with(b"\n") {
                file.write_all(b"\n")?;
            }
            Ok(())
        }
        None => match fs::remove_file(path) {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(err) => Err(err),
        },
    }
}
