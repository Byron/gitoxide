use super::Cache;
use crate::fs::Stack;
use crate::{fs, os};
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub enum Mode {
    /// Useful for checkout where directories need creation, but we need to access attributes as well.
    CreateDirectoryAndProvideAttributes {
        /// If there is a symlink or a file in our path, try to unlink it before creating the directory.
        unlink_on_collision: bool,

        /// just for testing
        #[cfg(debug_assertions)]
        test_mkdir_calls: usize,
        /// An additional per-user attributes file, similar to `$GIT_DIR/info/attributes`
        attributes_file: Option<PathBuf>,
    },
    /// Used when adding files, requiring access to both attributes and ignore information.
    ProvideAttributesAndIgnore {
        /// An additional per-user excludes file, similar to `$GIT_DIR/info/exclude`. It's an error if it is set but can't be read/opened.
        excludes_file: Option<PathBuf>,
        /// An additional per-user attributes file, similar to `$GIT_DIR/info/attributes`
        attributes_file: Option<PathBuf>,
    },
}

impl Mode {
    /// Configure a mode to be suitable for checking out files.
    pub fn checkout(unlink_on_collision: bool, attributes_file: Option<PathBuf>) -> Self {
        Mode::CreateDirectoryAndProvideAttributes {
            unlink_on_collision,
            #[cfg(debug_assertions)]
            test_mkdir_calls: 0,
            attributes_file,
        }
    }

    /// Configure a mode for adding files.
    pub fn add(excludes_file: Option<PathBuf>, attributes_file: Option<PathBuf>) -> Self {
        Mode::ProvideAttributesAndIgnore {
            excludes_file,
            attributes_file,
        }
    }
}

#[cfg(debug_assertions)]
impl Cache {
    pub fn num_mkdir_calls(&self) -> usize {
        match self.mode {
            Mode::CreateDirectoryAndProvideAttributes { test_mkdir_calls, .. } => test_mkdir_calls,
            _ => 0,
        }
    }

    pub fn reset_mkdir_calls(&mut self) {
        if let Mode::CreateDirectoryAndProvideAttributes { test_mkdir_calls, .. } = &mut self.mode {
            *test_mkdir_calls = 0;
        }
    }

    pub fn unlink_on_collision(&mut self, value: bool) {
        if let Mode::CreateDirectoryAndProvideAttributes {
            unlink_on_collision, ..
        } = &mut self.mode
        {
            *unlink_on_collision = value;
        }
    }
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
    fn assure_init(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Cache {
    /// Create a new instance with `worktree_root` being the base for all future paths we handle, assuming it to be valid which includes
    /// symbolic links to be included in it as well.
    pub fn new(worktree_root: impl Into<PathBuf>, mode: Mode) -> Self {
        let root = worktree_root.into();
        Cache {
            stack: fs::Stack::new(root),
            mode,
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
        self.assure_init()?;
        let op_mode = &mut self.mode;
        self.stack.make_relative_path_current(
            relative,
            |components, stack: &fs::Stack| {
                match op_mode {
                    Mode::CreateDirectoryAndProvideAttributes {
                        #[cfg(debug_assertions)]
                        test_mkdir_calls,
                        unlink_on_collision,
                        attributes_file: _,
                    } => create_leading_directory(components, stack, mode, test_mkdir_calls, *unlink_on_collision)?,
                    Mode::ProvideAttributesAndIgnore { .. } => todo!(),
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
            } else if unlink_on_collision {
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
        Err(err) => Err(err),
    }
}
