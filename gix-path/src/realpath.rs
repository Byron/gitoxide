/// The default amount of symlinks we may follow when resolving a path in [`realpath()`][crate::realpath()].
pub const MAX_SYMLINKS: u8 = 32;

pub(crate) mod function {
    #[cfg(windows)]
    use bstr::ByteSlice;
    use gix_error::{Result, ResultExt, bail, validation};
    #[cfg(windows)]
    use std::path::Prefix as PathPrefix;
    use std::path::{
        Component::{CurDir, Normal, ParentDir, Prefix, RootDir},
        Path, PathBuf,
    };
    #[cfg(windows)]
    use std::{
        ffi::OsString,
        os::windows::ffi::{OsStrExt, OsStringExt},
    };

    use crate::realpath::MAX_SYMLINKS;

    /// Check each component of `path` and see if it is a symlink. If so, resolve it.
    /// Do not fail for non-existing components, but assume these are as is.
    ///
    /// If `path` is relative, the current working directory is used to make it absolute. On Windows, drive-relative
    /// paths such as `C:repo` use the current directory on the specified drive.
    /// Note that the returned path will be verbatim, and repositories with `core.precomposeUnicode`
    /// set will probably want to precompose the paths unicode.
    pub fn realpath(path: impl AsRef<Path>) -> Result<PathBuf> {
        let path = path.as_ref();
        let cwd = path
            .is_relative()
            .then(std::env::current_dir)
            .unwrap_or_else(|| Ok(PathBuf::default()))
            .or_error()?;
        realpath_opts(path, &cwd, MAX_SYMLINKS)
    }

    /// The same as [`realpath()`], but allow to configure `max_symlinks` to configure how many symbolic links we are going to follow.
    /// This serves to avoid running into cycles or doing unreasonable amounts of work.
    ///
    /// `cwd` supplies the base for relative paths and should be absolute. On Windows, a drive-relative path uses
    /// `cwd` if its drive matches; otherwise Windows supplies that drive's current directory which queries
    /// the CWD from the operating system independently.
    /// Virtual drive prefixes created with `subst`, such as `1:` or `ä:`, are recognized too.
    pub fn realpath_opts(path: &Path, cwd: &Path, max_symlinks: u8) -> Result<PathBuf> {
        if path.as_os_str().is_empty() {
            bail!(validation("Empty is not a valid path"));
        }

        let mut real_path = PathBuf::new();
        if path.is_relative() {
            real_path.push(cwd);
        }

        let mut num_symlinks = 0;
        let mut path_backing: PathBuf;
        let mut components = path.components();
        #[cfg(windows)]
        let virtual_drive_root = {
            let path_bytes = path.as_os_str().as_encoded_bytes().as_bstr();
            if let Some(prefix) = crate::windows_drive_prefix(path_bytes)
                .filter(|_| !matches!(components.clone().next(), Some(Prefix(_))))
            {
                // Rust only recognizes ASCII letters as drives, whereas Git also accepts virtual drives.
                let mut root = crate::from_bstr(prefix)?.into_owned();
                root.as_mut_os_string().push("\\");
                real_path = if path_bytes.get(prefix.len()).is_some_and(|b| matches!(b, b'/' | b'\\')) {
                    root.clone()
                } else {
                    std::path::absolute(crate::from_bstr(prefix)?.as_ref()).or_error()?
                };
                if let Some(Normal(first)) = components.next() {
                    let prefix_len = crate::from_bstr(prefix)?.as_os_str().encode_wide().count();
                    let suffix: Vec<_> = first.encode_wide().skip(prefix_len).collect();
                    path_backing = PathBuf::from(OsString::from_wide(&suffix));
                    path_backing.extend(components);
                    components = path_backing.components();
                }
                Some(root)
            } else {
                None
            }
        };
        const MAX_SYMLINK_CHECKS: usize = 2048;
        let mut symlink_checks = 0;
        while let Some(component) = components.next() {
            match component {
                #[cfg(windows)]
                Prefix(prefix)
                    if matches!(prefix.kind(), PathPrefix::Disk(_)) && components.clone().next() != Some(RootDir) =>
                {
                    // For input `C:repo`, match bases like `C:\work` and `\\?\C:\work`, but not
                    // `D:\work` or `\\server\share\work`. This compares only the drive; a base like
                    // `C:work` also matches, so absolute-path validation follows below.
                    let same_drive = matches!(real_path.components().next(), Some(Prefix(base))
                        if matches!(base.kind(), PathPrefix::Disk(drive) | PathPrefix::VerbatimDisk(drive)
                            if prefix.kind() == PathPrefix::Disk(drive)));
                    if !same_drive || !real_path.is_absolute() {
                        // Resolve only the drive, preserving subsequent `..` for symlink resolution.
                        real_path = std::path::absolute(prefix.as_os_str()).or_error()?;
                    }
                }
                part @ (RootDir | Prefix(_)) => real_path.push(part),
                CurDir => {}
                ParentDir => {
                    #[cfg(windows)]
                    let at_virtual_root = virtual_drive_root.as_ref() == Some(&real_path);
                    #[cfg(not(windows))]
                    let at_virtual_root = false;
                    if at_virtual_root || !real_path.pop() {
                        bail!(validation(
                            "Ran out of path components while following parent component '..'",
                        ));
                    }
                }
                Normal(part) => {
                    real_path.push(part);
                    symlink_checks += 1;
                    if real_path.is_symlink() {
                        num_symlinks += 1;
                        if num_symlinks > max_symlinks {
                            bail!(validation(format!(
                                "The maximum allowed number {max_symlinks} of symlinks in path is exceeded"
                            )));
                        }
                        let mut link_destination = std::fs::read_link(real_path.as_path()).or_error()?;
                        if link_destination.is_absolute() {
                            // pushing absolute path to real_path resets it to the pushed absolute path
                        } else {
                            assert!(real_path.pop(), "we just pushed a component");
                        }
                        link_destination.extend(components);
                        path_backing = link_destination;
                        components = path_backing.components();
                    }
                    if symlink_checks > MAX_SYMLINK_CHECKS {
                        bail!(
                            "Cannot resolve symlinks in path with more than {MAX_SYMLINK_CHECKS} components (takes too long)"
                                .validation()
                        );
                    }
                }
            }
        }
        Ok(real_path)
    }
}
