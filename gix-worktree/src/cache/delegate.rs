use crate::{cache::State, PathIdMapping};

/// Various aggregate numbers related to the stack delegate itself.
#[derive(Default, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Statistics {
    /// The amount of `std::fs::create_dir` calls.
    ///
    /// This only happens if we are in the respective mode to create leading directories efficiently.
    pub num_mkdir_calls: usize,
    /// Amount of calls to push a path element.
    pub push_element: usize,
    /// Amount of calls to push a directory.
    pub push_directory: usize,
    /// Amount of calls to pop a directory.
    pub pop_directory: usize,
}

pub(crate) struct StackDelegate<'a, Find> {
    pub state: &'a mut State,
    pub buf: &'a mut Vec<u8>,
    pub is_dir: bool,
    pub id_mappings: &'a Vec<PathIdMapping>,
    pub find: Find,
    pub case: gix_glob::pattern::Case,
    pub statistics: &'a mut super::Statistics,
}

impl<'a, Find, E> gix_fs::stack::Delegate for StackDelegate<'a, Find>
where
    Find: for<'b> FnMut(&gix_hash::oid, &'b mut Vec<u8>) -> Result<gix_object::BlobRef<'b>, E>,
    E: std::error::Error + Send + Sync + 'static,
{
    fn push_directory(&mut self, stack: &gix_fs::Stack) -> std::io::Result<()> {
        self.statistics.delegate.push_directory += 1;
        let dir_bstr = gix_path::into_bstr(stack.current());
        let mut rela_dir = gix_glob::search::pattern::strip_base_handle_recompute_basename_pos(
            gix_path::into_bstr(stack.root()).as_ref(),
            dir_bstr.as_ref(),
            None,
            self.case,
        )
        .expect("dir in root")
        .0;
        if rela_dir.starts_with(b"/") {
            rela_dir = &rela_dir[1..];
        }
        match &mut self.state {
            State::CreateDirectoryAndAttributesStack { attributes, .. } => {
                attributes.push_directory(
                    stack.root(),
                    stack.current(),
                    rela_dir,
                    self.buf,
                    self.id_mappings,
                    &mut self.find,
                    &mut self.statistics.attributes,
                )?;
            }
            State::AttributesAndIgnoreStack { ignore, attributes } => {
                attributes.push_directory(
                    stack.root(),
                    stack.current(),
                    rela_dir,
                    self.buf,
                    self.id_mappings,
                    &mut self.find,
                    &mut self.statistics.attributes,
                )?;
                ignore.push_directory(
                    stack.root(),
                    stack.current(),
                    rela_dir,
                    self.buf,
                    self.id_mappings,
                    &mut self.find,
                    self.case,
                    &mut self.statistics.ignore,
                )?
            }
            State::IgnoreStack(ignore) => ignore.push_directory(
                stack.root(),
                stack.current(),
                rela_dir,
                self.buf,
                self.id_mappings,
                &mut self.find,
                self.case,
                &mut self.statistics.ignore,
            )?,
        }
        Ok(())
    }

    fn push(&mut self, is_last_component: bool, stack: &gix_fs::Stack) -> std::io::Result<()> {
        self.statistics.delegate.push_element += 1;
        match &mut self.state {
            State::CreateDirectoryAndAttributesStack {
                unlink_on_collision,
                attributes: _,
            } => create_leading_directory(
                is_last_component,
                stack,
                self.is_dir,
                &mut self.statistics.delegate.num_mkdir_calls,
                *unlink_on_collision,
            )?,
            State::AttributesAndIgnoreStack { .. } | State::IgnoreStack(_) => {}
        }
        Ok(())
    }

    fn pop_directory(&mut self) {
        self.statistics.delegate.pop_directory += 1;
        match &mut self.state {
            State::CreateDirectoryAndAttributesStack { attributes, .. } => {
                attributes.pop_directory();
            }
            State::AttributesAndIgnoreStack { attributes, ignore } => {
                attributes.pop_directory();
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
    stack: &gix_fs::Stack,
    is_dir: bool,
    mkdir_calls: &mut usize,
    unlink_on_collision: bool,
) -> std::io::Result<()> {
    if is_last_component && !is_dir {
        return Ok(());
    }
    *mkdir_calls += 1;
    match std::fs::create_dir(stack.current()) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
            let meta = stack.current().symlink_metadata()?;
            if meta.is_dir() {
                Ok(())
            } else if unlink_on_collision {
                if meta.file_type().is_symlink() {
                    gix_fs::symlink::remove(stack.current())?;
                } else {
                    std::fs::remove_file(stack.current())?;
                }
                *mkdir_calls += 1;
                std::fs::create_dir(stack.current())
            } else {
                Err(err)
            }
        }
        Err(err) => Err(err),
    }
}
