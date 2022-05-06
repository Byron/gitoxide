use std::path::PathBuf;

#[cfg(all(feature = "unstable"))]
pub use git_path::*;

///
pub mod create;

pub(crate) fn install_dir() -> std::io::Result<PathBuf> {
    std::env::current_exe().and_then(|exe| {
        exe.parent()
            .map(ToOwned::to_owned)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "no parent for current executable"))
    })
}

/// Reads a path from a file that has it on the very first line, or `None` if `path` does not exist.
pub(crate) fn read_from_file(path: impl AsRef<std::path::Path>) -> Option<std::io::Result<PathBuf>> {
    use crate::bstr::ByteSlice;
    let mut buf = match std::fs::read(path) {
        Ok(buf) => buf,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return None,
        Err(err) => return Some(Err(err)),
    };
    let trimmed_len = buf.trim_end().len();
    buf.truncate(buf.len() - trimmed_len);
    Some(Ok(git_path::from_bstring(buf)))
}
