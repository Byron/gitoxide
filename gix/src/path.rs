use std::path::PathBuf;

pub use gix_path::*;

pub(crate) fn install_dir() -> std::io::Result<PathBuf> {
    std::env::current_exe().and_then(|exe| {
        exe.parent()
            .map(ToOwned::to_owned)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "no parent for current executable"))
    })
}
