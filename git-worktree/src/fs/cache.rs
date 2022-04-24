use super::Cache;
use crate::fs::Stack;
use crate::{fs, os};
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub enum State {
    /// Useful for checkout where directories need creation, but we need to access attributes as well.
    CreateDirectoryAndAttributesStack {
        /// If there is a symlink or a file in our path, try to unlink it before creating the directory.
        unlink_on_collision: bool,

        /// just for testing
        #[cfg(debug_assertions)]
        test_mkdir_calls: usize,
        /// State to handle attribute information
        attributes: state::Attributes,
    },
    /// Used when adding files, requiring access to both attributes and ignore information, for example during add operations.
    AttributesAndIgnoreStack {
        /// State to handle attribute information
        attributes: state::Attributes,
        /// State to handle exclusion information
        ignore: state::Ignore,
    },
    /// Used when providing worktree status information.
    IgnoreStack(state::Ignore),
}

///
pub mod state {
    type AttributeMatchGroup = git_attributes::MatchGroup<git_attributes::Attributes>;
    type IgnoreMatchGroup = git_attributes::MatchGroup<git_attributes::Ignore>;

    /// State related to attributes associated with files in the repository.
    #[derive(Default, Clone)]
    #[allow(unused)]
    pub struct Attributes {
        /// Attribute patterns that match the currently set directory (in the stack).
        stack: AttributeMatchGroup,
        /// Attribute patterns which aren't tied to the repository root, hence are global. They are consulted last.
        globals: AttributeMatchGroup,
    }

    /// State related to the exclusion of files.
    #[derive(Default, Clone)]
    #[allow(unused)]
    pub struct Ignore {
        /// Ignore patterns passed as overrides to everything else, typically passed on the command-line and the first patterns to
        /// be consulted.
        overrides: IgnoreMatchGroup,
        /// Ignore patterns that match the currently set director (in the stack).
        stack: IgnoreMatchGroup,
        /// Ignore patterns which aren't tied to the repository root, hence are global. They are consulted last.
        globals: IgnoreMatchGroup,
    }

    impl Ignore {
        pub fn new(overrides: IgnoreMatchGroup, globals: IgnoreMatchGroup) -> Self {
            Ignore {
                overrides,
                globals,
                stack: Default::default(),
            }
        }
    }

    impl Attributes {
        pub fn new(globals: AttributeMatchGroup) -> Self {
            Attributes {
                globals,
                stack: Default::default(),
            }
        }
    }

    impl From<AttributeMatchGroup> for Attributes {
        fn from(group: AttributeMatchGroup) -> Self {
            Attributes::new(group)
        }
    }
}

impl State {
    /// Configure a state to be suitable for checking out files.
    pub fn for_checkout(unlink_on_collision: bool, attributes: state::Attributes) -> Self {
        State::CreateDirectoryAndAttributesStack {
            unlink_on_collision,
            #[cfg(debug_assertions)]
            test_mkdir_calls: 0,
            attributes,
        }
    }

    /// Configure a state for adding files.
    pub fn for_add(attributes: state::Attributes, ignore: state::Ignore) -> Self {
        State::AttributesAndIgnoreStack { attributes, ignore }
    }

    /// Configure a state for status retrieval.
    pub fn for_status(ignore: state::Ignore) -> Self {
        State::IgnoreStack(ignore)
    }
}

#[cfg(debug_assertions)]
impl Cache {
    pub fn num_mkdir_calls(&self) -> usize {
        match self.mode {
            State::CreateDirectoryAndAttributesStack { test_mkdir_calls, .. } => test_mkdir_calls,
            _ => 0,
        }
    }

    pub fn reset_mkdir_calls(&mut self) {
        if let State::CreateDirectoryAndAttributesStack { test_mkdir_calls, .. } = &mut self.mode {
            *test_mkdir_calls = 0;
        }
    }

    pub fn unlink_on_collision(&mut self, value: bool) {
        if let State::CreateDirectoryAndAttributesStack {
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
    pub fn path(&self) -> &'a Path {
        self.parent.stack.current()
    }

    /// See if the currently set entry is excluded as per exclude and git-ignore files.
    pub fn is_excluded(&self, case: git_glob::pattern::Case) -> bool {
        self.matching_exclude_pattern(case)
            .map_or(false, |m| m.pattern.is_negative())
    }

    /// Check all exclude patterns to see if the currently set path matches any of them.
    ///
    /// Note that this pattern might be negated, so means the opposite.
    ///
    /// # Panics
    ///
    /// If the cache was configured without exclude patterns.
    pub fn matching_exclude_pattern(&self, _case: git_glob::pattern::Case) -> Option<git_attributes::Match<'_, ()>> {
        todo!()
    }
}

impl<'a> std::fmt::Debug for Platform<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.path(), f)
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
    pub fn new(worktree_root: impl Into<PathBuf>, mode: State, buf: Vec<u8>) -> Self {
        let root = worktree_root.into();
        Cache {
            stack: fs::Stack::new(root),
            mode,
            buf,
        }
    }

    /// Append the `relative` path to the root directory the cache contains and efficiently create leading directories
    /// unless `is_dir` is known (`Some(…)`) then `relative` points to a directory itself in which case the entire resulting
    /// path is created as directory. If it's not known it is assumed to be a file.
    ///
    /// Provide access to cached information for that `relative` entry via the platform returned.
    pub fn at_entry(&mut self, relative: impl AsRef<Path>, is_dir: Option<bool>) -> std::io::Result<Platform<'_>> {
        self.assure_init()?;
        let op_mode = &mut self.mode;
        self.stack.make_relative_path_current(
            relative,
            |components, stack: &fs::Stack| {
                match op_mode {
                    State::CreateDirectoryAndAttributesStack {
                        #[cfg(debug_assertions)]
                        test_mkdir_calls,
                        unlink_on_collision,
                        attributes: _,
                    } => {
                        #[cfg(debug_assertions)]
                        {
                            create_leading_directory(components, stack, is_dir, test_mkdir_calls, *unlink_on_collision)?
                        }
                        #[cfg(not(debug_assertions))]
                        {
                            create_leading_directory(components, stack, is_dir, *unlink_on_collision)?
                        }
                    }
                    State::AttributesAndIgnoreStack { .. } => todo!(),
                    State::IgnoreStack { .. } => todo!(),
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
    target_is_dir: Option<bool>,
    #[cfg(debug_assertions)] mkdir_calls: &mut usize,
    unlink_on_collision: bool,
) -> std::io::Result<()> {
    let target_is_dir = target_is_dir.unwrap_or(false);
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
                if meta.file_type().is_symlink() {
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
