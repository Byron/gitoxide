use crate::SymlinkCheck;
use gix_fs::Stack;
use std::path::{Path, PathBuf};

impl SymlinkCheck {
    /// Create a new stack that starts operating at `root`.
    pub fn new(root: PathBuf) -> Self {
        Self {
            inner: gix_fs::Stack::new(root),
        }
    }

    /// Return a valid filesystem path located in our root by appending `relative_path`, which is guaranteed to
    /// not pass through a symbolic link. That way the caller can be sure to not be misled by an attacker that
    /// tries to make us reach outside of the repository.
    ///
    /// Note that the file pointed to by `relative_path` may still be a symbolic link, or not exist at all,
    /// and that an error may also be produced if directories on the path leading to the leaf
    /// component of `relative_path` are missing.
    ///
    /// ### Note
    ///
    /// On windows, no verification is performed, instead only the combined path is provided as usual.
    pub fn verified_path(&mut self, relative_path: &Path) -> std::io::Result<&Path> {
        self.inner.make_relative_path_current(relative_path, &mut Delegate)?;
        Ok(self.inner.current())
    }
}

struct Delegate;

impl gix_fs::stack::Delegate for Delegate {
    fn push_directory(&mut self, _stack: &Stack) -> std::io::Result<()> {
        Ok(())
    }

    fn push(&mut self, is_last_component: bool, stack: &Stack) -> std::io::Result<()> {
        #[cfg(windows)]
        {
            Ok(())
        }
        #[cfg(not(windows))]
        {
            if is_last_component {
                return Ok(());
            }

            if stack.current().symlink_metadata()?.is_symlink() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Cannot step through symlink to perform an lstat",
                ));
            }
            Ok(())
        }
    }

    fn pop_directory(&mut self) {}
}
