use std::{io, io::ErrorKind::AlreadyExists, path::Path};

/// Create a new symlink at `link` which points to `original`.
///
/// Note that `original` doesn't have to exist.
#[cfg(unix)]
pub fn create(original: &Path, link: &Path) -> io::Result<()> {
    std::os::unix::fs::symlink(original, link)
}

/// Create a new symlink at `link` which points to `original`.
///
/// Note that `original` doesn't have to exist.
#[cfg(target_os = "wasi")]
pub fn create(original: &Path, link: &Path) -> io::Result<()> {
    std::fs::soft_link(original, link)
}

/// Create a new symlink at `link` which points to `original`.
///
/// Note that symbolic links are unsupported on this platform.
#[cfg(not(any(unix, windows, target_os = "wasi")))]
pub fn create(_original: &Path, _link: &Path) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "symbolic links are not supported on this platform",
    ))
}

/// Remove a symlink.
///
/// Note that on only on windows this is special.
#[cfg(any(unix, target_os = "wasi"))]
pub fn remove(path: &Path) -> io::Result<()> {
    std::fs::remove_file(path)
}

/// Remove a symlink.
#[cfg(not(any(unix, windows, target_os = "wasi")))]
pub fn remove(_path: &Path) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "symbolic links are not supported on this platform",
    ))
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

/// Create a new symlink at `link` which points to `original`.
///
/// Note that if a symlink target (the `original`) isn't present on disk, it's assumed to be a
/// file, creating a dangling file symlink. This is similar to a dangling symlink on Unix,
/// which doesn't have to care about the target type though.
/// Targets starting with `/` or `\` are also treated as files without probing them, as Windows
/// can interpret these as network shares and probing them could disclose credentials.
#[cfg(windows)]
pub fn create(original: &Path, link: &Path) -> io::Result<()> {
    create_inner(original, link, false).map(|_| ())
}

/// Create a new symlink at `link` pointing to `original`, returning whether it was created.
///
/// On Windows, return `false` without creating a link if the target's type cannot be determined.
/// Callers can retry after creating other links, then use [`create()`] for any unresolved targets
/// once no more progress is possible. Targets starting with `/` or `\` are never probed and are
/// immediately created as file links, just like in [`create()`].
/// On other platforms, this is equivalent to [`create()`] and always returns `true` on success.
pub fn create_or_delay(original: &Path, link: &Path) -> io::Result<bool> {
    #[cfg(windows)]
    {
        create_inner(original, link, true)
    }
    #[cfg(not(windows))]
    {
        create(original, link).map(|()| true)
    }
}

#[cfg(windows)]
fn create_inner(original: &Path, link: &Path, delay_if_unknown: bool) -> io::Result<bool> {
    use std::os::windows::fs::{symlink_dir, symlink_file};

    // Match Git's `process_phantom_symlink()`: a leading separator can introduce a UNC
    // share or an NT namespace path. Check the original spelling before any target probe.
    let starts_with_slash = matches!(original.as_os_str().as_encoded_bytes().first(), Some(b'/' | b'\\'));
    let is_dir = if starts_with_slash {
        false
    } else {
        let parent = link
            .parent()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "symlink path has no parent directory"))?;
        match parent.join(original).metadata() {
            Ok(meta) => meta.is_dir(),
            Err(_) if delay_if_unknown => return Ok(false),
            Err(_) => false,
        }
    };
    if is_dir {
        symlink_dir(original, link)?;
    } else {
        symlink_file(original, link)?;
    }
    Ok(true)
}

/// Return true if `err` indicates that a file collision happened, i.e. a symlink couldn't be created as the `link`
/// already exists as filesystem object.
#[cfg(any(unix, target_os = "wasi"))]
pub fn is_collision_error(err: &std::io::Error) -> bool {
    // TODO: use `ErrorKind::FilesystemLoop` once stabilized.
    err.kind() == AlreadyExists
            || err.kind() == std::io::ErrorKind::IsADirectory
            || err.raw_os_error() == Some(62) // no-follow on symlnk on mac-os
            || err.raw_os_error() == Some(40) // no-follow on symlnk on ubuntu
}

/// Return true if `err` indicates that a file collision happened, i.e. a symlink couldn't be created as the `link`
/// already exists as filesystem object.
#[cfg(windows)]
pub fn is_collision_error(err: &std::io::Error) -> bool {
    err.kind() == AlreadyExists || err.kind() == std::io::ErrorKind::PermissionDenied
}

/// Return true if `err` indicates that a file collision happened, i.e. a symlink couldn't be created as the `link`
/// already exists as filesystem object.
#[cfg(not(any(unix, windows, target_os = "wasi")))]
pub fn is_collision_error(err: &std::io::Error) -> bool {
    err.kind() == AlreadyExists
}
