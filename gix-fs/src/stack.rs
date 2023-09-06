use std::path::{Path, PathBuf};

use crate::Stack;

/// Access
impl Stack {
    /// Returns the top-level path of the stack.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Returns the absolute path the currently set path.
    pub fn current(&self) -> &Path {
        &self.current
    }

    /// Returns the currently set path relative to the [`root()`][Stack::root()].
    pub fn current_relative(&self) -> &Path {
        &self.current_relative
    }
}

/// A delegate for use in a [`Stack`].
pub trait Delegate {
    /// Called whenever we push a directory on top of the stack, after the fact.
    ///
    /// It is also called if the currently acted on path is a directory in itself.
    /// Use `stack.current()` to see the directory.
    fn push_directory(&mut self, stack: &Stack) -> std::io::Result<()>;

    /// Called after any component was pushed, with the path available at `stack.current()`.
    ///
    /// `is_last_component` is true if the path is completely built.
    fn push(&mut self, is_last_component: bool, stack: &Stack) -> std::io::Result<()>;

    /// Called right after a directory-component was popped off the stack.
    ///
    /// Use it to pop information off internal data structures.
    fn pop_directory(&mut self);
}

impl Stack {
    /// Create a new instance with `root` being the base for all future paths we handle, assuming it to be valid which includes
    /// symbolic links to be included in it as well.
    pub fn new(root: PathBuf) -> Self {
        Stack {
            current: root.clone(),
            current_relative: PathBuf::with_capacity(128),
            valid_components: 0,
            root,
            current_is_directory: true,
        }
    }

    /// Set the current stack to point to the `relative` path and call `push_comp()` each time a new path component is popped
    /// along with the stacks state for inspection to perform an operation that produces some data.
    ///
    /// The full path to `relative` will be returned along with the data returned by `push_comp`.
    /// Note that this only works correctly for the delegate's `push_directory()` and `pop_directory()` methods if
    /// `relative` paths are terminal, so point to their designated file or directory.
    pub fn make_relative_path_current(&mut self, relative: &Path, delegate: &mut dyn Delegate) -> std::io::Result<()> {
        debug_assert!(
            relative.is_relative(),
            "only index paths are handled correctly here, must be relative"
        );
        debug_assert!(!relative.to_string_lossy().is_empty(), "empty paths are not allowed");

        if self.valid_components == 0 {
            delegate.push_directory(self)?;
        }

        let mut components = relative.components().peekable();
        let mut existing_components = self.current_relative.components();
        let mut matching_components = 0;
        while let (Some(existing_comp), Some(new_comp)) = (existing_components.next(), components.peek()) {
            if existing_comp == *new_comp {
                components.next();
                matching_components += 1;
            } else {
                break;
            }
        }

        for _ in 0..self.valid_components - matching_components {
            self.current.pop();
            self.current_relative.pop();
            if self.current_is_directory {
                delegate.pop_directory();
            }
            self.current_is_directory = true;
        }
        self.valid_components = matching_components;

        if !self.current_is_directory && components.peek().is_some() {
            delegate.push_directory(self)?;
        }

        while let Some(comp) = components.next() {
            let is_last_component = components.peek().is_none();
            self.current_is_directory = !is_last_component;
            self.current.push(comp);
            self.current_relative.push(comp);
            self.valid_components += 1;
            let res = delegate.push(is_last_component, self);
            if self.current_is_directory {
                delegate.push_directory(self)?;
            }

            if let Err(err) = res {
                self.current.pop();
                self.current_relative.pop();
                self.valid_components -= 1;
                return Err(err);
            }
        }
        Ok(())
    }
}
