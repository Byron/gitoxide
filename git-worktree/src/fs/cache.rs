use super::Cache;
use crate::fs::Stack;
use crate::{fs, os};
use std::path::{Path, PathBuf};

#[derive(Copy, Clone)]
pub enum Mode {
    /// Useful for checkout where directories need creation, but we need to access attributes as well.
    CreateDirectoryAndProvideAttributes,
    /// Used when adding files, requiring access to both attributes and ignore information.
    ProvideAttributesAndIgnore,
}

pub struct Platform<'a> {
    parent: &'a Cache,
}

impl<'a> Platform<'a> {
    /// The full path to `relative` will be returned for use on the file system.
    pub fn leading_dir(&self) -> &'a Path {
        self.parent.stack.current()
    }
}

impl<'a> std::fmt::Debug for Platform<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.leading_dir(), f)
    }
}

impl Cache {
    /// Create a new instance with `root` being the base for all future paths we handle, assuming it to be valid which includes
    /// symbolic links to be included in it as well.
    pub fn new(root: impl Into<PathBuf>, mode: Mode) -> Self {
        let root = root.into();
        Cache {
            stack: fs::Stack::new(root),
            mode,
            #[cfg(debug_assertions)]
            test_mkdir_calls: 0,
            unlink_on_collision: false,
        }
    }

    /// Append the `relative` path to the root directory the cache contains and efficiently create leading directories
    /// unless `mode` indicates `relative` points to a directory itself in which case the entire resulting path is created as directory.
    ///
    /// Provide access to cached information for that `relative` entry via the platform returned.
    pub fn at_entry(
        &mut self,
        relative: impl AsRef<Path>,
        mode: git_index::entry::Mode,
    ) -> std::io::Result<Platform<'_>> {
        #[cfg(debug_assertions)]
        let mkdir_calls = &mut self.test_mkdir_calls;
        let unlink_on_collision = self.unlink_on_collision;
        let op_mode = self.mode;
        self.stack.make_relative_path_current(
            relative,
            |components, stack: &fs::Stack| {
                match op_mode {
                    Mode::CreateDirectoryAndProvideAttributes => {
                        create_leading_directory(components, stack, mode, mkdir_calls, unlink_on_collision)?
                    }
                    Mode::ProvideAttributesAndIgnore => todo!(),
                }
                Ok(())
            },
            |_| {},
        )?;
        Ok(Platform { parent: self })
    }
}

fn create_leading_directory(
    components: &mut std::iter::Peekable<std::path::Components<'_>>,
    stack: &Stack,
    mode: git_index::entry::Mode,
    mkdir_calls: &mut usize,
    unlink_on_collision: bool,
) -> std::io::Result<()> {
    let target_is_dir = mode == git_index::entry::Mode::COMMIT || mode == git_index::entry::Mode::DIR;
    if !(components.peek().is_some() || target_is_dir) {
        return Ok(());
    }
    #[cfg(debug_assertions)]
    {
        *mkdir_calls += 1;
    }
    match std::fs::create_dir(stack.current()) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
            let meta = stack.current().symlink_metadata()?;
            if meta.is_dir() {
                Ok(())
            } else {
                if unlink_on_collision {
                    if meta.is_symlink() {
                        os::remove_symlink(stack.current())?;
                    } else {
                        std::fs::remove_file(stack.current())?;
                    }
                    #[cfg(debug_assertions)]
                    {
                        *mkdir_calls += 1;
                    }
                    std::fs::create_dir(stack.current())
                } else {
                    Err(err)
                }
            }
        }
        Err(err) => Err(err),
    }
}
