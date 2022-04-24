use crate::fs::Stack;
use std::path::{Path, PathBuf};

impl Stack {
    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn current(&self) -> &Path {
        &self.current
    }

    pub fn current_relative(&self) -> &Path {
        &self.current_relative
    }
}

pub trait Delegate {
    fn push(&mut self, is_last_component: bool, stack: &Stack) -> std::io::Result<()>;
    fn pop(&mut self, stack: &Stack);
}

impl Stack {
    /// Create a new instance with `root` being the base for all future paths we handle, assuming it to be valid which includes
    /// symbolic links to be included in it as well.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        Stack {
            current: root.clone(),
            current_relative: PathBuf::with_capacity(128),
            valid_components: 0,
            root,
        }
    }

    /// Set the current stack to point to the `relative` path and call `push_comp()` each time a new path component is popped
    /// along with the stacks state for inspection to perform an operation that produces some data.
    ///
    /// The full path to `relative` will be returned along with the data returned by push_comp.
    pub fn make_relative_path_current(
        &mut self,
        relative: impl AsRef<Path>,
        delegate: &mut impl Delegate,
    ) -> std::io::Result<()> {
        let relative = relative.as_ref();
        debug_assert!(
            relative.is_relative(),
            "only index paths are handled correctly here, must be relative"
        );

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
            delegate.pop(self);
        }
        self.valid_components = matching_components;

        while let Some(comp) = components.next() {
            self.current.push(comp);
            self.current_relative.push(comp);
            self.valid_components += 1;
            let res = delegate.push(components.peek().is_none(), self);

            if let Err(err) = res {
                self.current.pop();
                self.current_relative.pop();
                self.valid_components -= 1;
                delegate.pop(self);
                return Err(err);
            }
        }
        Ok(())
    }
}
