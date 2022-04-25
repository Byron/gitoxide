use super::Cache;
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
    use std::path::Path;

    type AttributeMatchGroup = git_attributes::MatchGroup<git_attributes::Attributes>;
    type IgnoreMatchGroup = git_attributes::MatchGroup<git_attributes::Ignore>;

    /// State related to attributes associated with files in the repository.
    #[derive(Default, Clone)]
    #[allow(unused)]
    pub struct Attributes {
        /// Attribute patterns that match the currently set directory (in the stack).
        pub stack: AttributeMatchGroup,
        /// Attribute patterns which aren't tied to the repository root, hence are global. They are consulted last.
        pub globals: AttributeMatchGroup,
    }

    /// State related to the exclusion of files.
    #[derive(Default, Clone)]
    #[allow(unused)]
    pub struct Ignore {
        /// Ignore patterns passed as overrides to everything else, typically passed on the command-line and the first patterns to
        /// be consulted.
        pub overrides: IgnoreMatchGroup,
        /// Ignore patterns that match the currently set director (in the stack).
        pub stack: IgnoreMatchGroup,
        /// Ignore patterns which aren't tied to the repository root, hence are global. They are consulted last.
        pub globals: IgnoreMatchGroup,
    }

    impl Ignore {
        pub fn new(overrides: IgnoreMatchGroup, globals: IgnoreMatchGroup) -> Self {
            Ignore {
                overrides,
                globals,
                stack: Default::default(),
            }
        }

        pub fn push(&mut self, root: &Path, dir: &Path, buf: &mut Vec<u8>) -> std::io::Result<()> {
            let follow_symlinks = true;
            if !self
                .stack
                .add_patterns_file(dir.join(".gitignore"), follow_symlinks, Some(root), buf)?
            {
                // Need one stack level per component so push and pop matches.
                self.stack.patterns.push(Default::default());
            }
            // TODO: from index
            Ok(())
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

impl State {
    pub(crate) fn ignore_or_panic(&self) -> &state::Ignore {
        match self {
            State::IgnoreStack(v) => v,
            State::AttributesAndIgnoreStack { ignore, .. } => ignore,
            State::CreateDirectoryAndAttributesStack { .. } => {
                unreachable!("BUG: must not try to check excludes without it being setup")
            }
        }
    }
}

#[cfg(debug_assertions)]
impl Cache {
    pub fn num_mkdir_calls(&self) -> usize {
        match self.state {
            State::CreateDirectoryAndAttributesStack { test_mkdir_calls, .. } => test_mkdir_calls,
            _ => 0,
        }
    }

    pub fn reset_mkdir_calls(&mut self) {
        if let State::CreateDirectoryAndAttributesStack { test_mkdir_calls, .. } = &mut self.state {
            *test_mkdir_calls = 0;
        }
    }

    pub fn unlink_on_collision(&mut self, value: bool) {
        if let State::CreateDirectoryAndAttributesStack {
            unlink_on_collision, ..
        } = &mut self.state
        {
            *unlink_on_collision = value;
        }
    }
}

pub struct Platform<'a> {
    parent: &'a Cache,
    is_dir: Option<bool>,
}

struct PlatformMut<'a> {
    state: &'a mut State,
    buf: &'a mut Vec<u8>,
    is_dir: Option<bool>,
}

impl<'a> Platform<'a> {
    /// The full path to `relative` will be returned for use on the file system.
    pub fn path(&self) -> &'a Path {
        self.parent.stack.current()
    }

    /// See if the currently set entry is excluded as per exclude and git-ignore files.
    ///
    /// # Panics
    ///
    /// If the cache was configured without exclude patterns.
    pub fn is_excluded(&self) -> bool {
        self.matching_exclude_pattern()
            .map_or(false, |m| m.pattern.is_negative())
    }

    /// Check all exclude patterns to see if the currently set path matches any of them.
    ///
    /// Note that this pattern might be negated, and means this path in included.
    ///
    /// # Panics
    ///
    /// If the cache was configured without exclude patterns.
    pub fn matching_exclude_pattern(&self) -> Option<git_attributes::Match<'_, ()>> {
        let ignore_groups = self.parent.state.ignore_or_panic();
        let relative_path =
            git_features::path::into_bytes_or_panic_on_windows(self.parent.stack.current_relative.as_path());
        [&ignore_groups.overrides, &ignore_groups.stack, &ignore_groups.globals]
            .iter()
            .find_map(|group| {
                group.pattern_matching_relative_path(relative_path.as_ref(), self.is_dir, self.parent.case)
            })
    }
}

impl<'a> std::fmt::Debug for Platform<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.path(), f)
    }
}

impl Cache {
    /// Create a new instance with `worktree_root` being the base for all future paths we handle, assuming it to be valid which includes
    /// symbolic links to be included in it as well.
    /// The `case` configures attribute and exclusion query case sensitivity.
    pub fn new(worktree_root: impl Into<PathBuf>, mode: State, case: git_glob::pattern::Case, buf: Vec<u8>) -> Self {
        let root = worktree_root.into();
        Cache {
            stack: fs::Stack::new(root),
            state: mode,
            case,
            buf,
        }
    }

    /// Append the `relative` path to the root directory the cache contains and efficiently create leading directories
    /// unless `is_dir` is known (`Some(…)`) then `relative` points to a directory itself in which case the entire resulting
    /// path is created as directory. If it's not known it is assumed to be a file.
    ///
    /// Provide access to cached information for that `relative` entry via the platform returned.
    pub fn at_entry(&mut self, relative: impl AsRef<Path>, is_dir: Option<bool>) -> std::io::Result<Platform<'_>> {
        let mut platform = PlatformMut {
            state: &mut self.state,
            buf: &mut self.buf,
            is_dir,
        };
        self.stack.make_relative_path_current(relative, &mut platform)?;
        Ok(Platform { parent: self, is_dir })
    }
}

impl<'a> fs::stack::Delegate for PlatformMut<'a> {
    fn push(&mut self, is_last_component: bool, stack: &fs::Stack) -> std::io::Result<()> {
        match &mut self.state {
            State::CreateDirectoryAndAttributesStack {
                #[cfg(debug_assertions)]
                test_mkdir_calls,
                unlink_on_collision,
                attributes: _,
            } => {
                #[cfg(debug_assertions)]
                {
                    create_leading_directory(
                        is_last_component,
                        stack,
                        self.is_dir,
                        test_mkdir_calls,
                        *unlink_on_collision,
                    )?
                }
                #[cfg(not(debug_assertions))]
                {
                    create_leading_directory(is_last_component, stack, self.is_dir, *unlink_on_collision)?
                }
            }
            State::AttributesAndIgnoreStack { ignore, .. } => ignore.push(
                &stack.root,
                stack.current.parent().expect("component was just pushed"),
                self.buf,
            )?,
            State::IgnoreStack(ignore) => ignore.push(
                &stack.root,
                stack.current.parent().expect("component was just pushed"),
                self.buf,
            )?,
        }
        Ok(())
    }

    fn pop(&mut self, _stack: &fs::Stack) {
        match &mut self.state {
            State::CreateDirectoryAndAttributesStack { attributes, .. } => {
                attributes.stack.patterns.pop();
            }
            State::AttributesAndIgnoreStack { attributes, ignore } => {
                attributes.stack.patterns.pop();
                ignore.stack.patterns.pop();
            }
            State::IgnoreStack(ignore) => {
                ignore.stack.patterns.pop();
            }
        }
    }
}

fn create_leading_directory(
    is_last_component: bool,
    stack: &fs::Stack,
    target_is_dir: Option<bool>,
    #[cfg(debug_assertions)] mkdir_calls: &mut usize,
    unlink_on_collision: bool,
) -> std::io::Result<()> {
    let target_is_dir = target_is_dir.unwrap_or(false);
    if is_last_component && !target_is_dir {
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
