use crate::fs;
use crate::fs::cache::{Platform, State};
use crate::fs::PathOidMapping;
use bstr::ByteSlice;
use git_hash::oid;
use std::path::Path;

impl<'a, 'paths> Platform<'a, 'paths> {
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
        let ignore = self.parent.state.ignore_or_panic();
        let relative_path =
            git_path::to_unix_separators_on_windows(git_path::into_bstr(self.parent.stack.current_relative.as_path()));
        ignore.matching_exclude_pattern(relative_path.as_bstr(), self.is_dir, self.parent.case)
    }
}

impl<'a, 'paths> std::fmt::Debug for Platform<'a, 'paths> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.path(), f)
    }
}

pub struct StackDelegate<'a, 'paths, Find> {
    pub state: &'a mut State,
    pub buf: &'a mut Vec<u8>,
    pub is_dir: Option<bool>,
    pub attribute_files_in_index: &'a Vec<PathOidMapping<'paths>>,
    pub find: Find,
}

impl<'a, 'paths, Find, E> fs::stack::Delegate for StackDelegate<'a, 'paths, Find>
where
    Find: for<'b> FnMut(&oid, &'b mut Vec<u8>) -> Result<git_object::BlobRef<'b>, E>,
    E: std::error::Error + Send + Sync + 'static,
{
    fn init(&mut self, stack: &fs::Stack) -> std::io::Result<()> {
        match &mut self.state {
            State::CreateDirectoryAndAttributesStack { attributes: _, .. } => {
                // TODO: attribute init
            }
            State::AttributesAndIgnoreStack { ignore, attributes: _ } => {
                // TODO: attribute init
                ignore.push_directory(
                    &stack.root,
                    &stack.root,
                    self.buf,
                    self.attribute_files_in_index,
                    &mut self.find,
                )?
            }
            State::IgnoreStack(ignore) => ignore.push_directory(
                &stack.root,
                &stack.root,
                self.buf,
                self.attribute_files_in_index,
                &mut self.find,
            )?,
        }
        Ok(())
    }

    fn push_directory(&mut self, stack: &fs::Stack) -> std::io::Result<()> {
        match &mut self.state {
            State::CreateDirectoryAndAttributesStack { attributes: _, .. } => {
                // TODO: attributes
            }
            State::AttributesAndIgnoreStack { ignore, attributes: _ } => {
                // TODO: attributes
                ignore.push_directory(
                    &stack.root,
                    &stack.current,
                    self.buf,
                    self.attribute_files_in_index,
                    &mut self.find,
                )?
            }
            State::IgnoreStack(ignore) => ignore.push_directory(
                &stack.root,
                &stack.current,
                self.buf,
                self.attribute_files_in_index,
                &mut self.find,
            )?,
        }
        Ok(())
    }

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
            State::AttributesAndIgnoreStack { .. } | State::IgnoreStack(_) => {}
        }
        Ok(())
    }

    fn pop_directory(&mut self) {
        match &mut self.state {
            State::CreateDirectoryAndAttributesStack { attributes: _, .. } => {
                // TODO: attributes
            }
            State::AttributesAndIgnoreStack { attributes: _, ignore } => {
                // TODO: attributes
                ignore.pop_directory();
            }
            State::IgnoreStack(ignore) => {
                ignore.pop_directory();
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
                    crate::os::remove_symlink(stack.current())?;
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
