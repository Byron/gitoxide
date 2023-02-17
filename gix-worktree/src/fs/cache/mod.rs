#![allow(missing_docs)]
use std::path::{Path, PathBuf};

use bstr::{BStr, ByteSlice};
use gix_hash::oid;

use super::Cache;
use crate::{fs, fs::PathOidMapping};

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

#[cfg(debug_assertions)]
impl Cache {
    pub fn set_case(&mut self, case: gix_glob::pattern::Case) {
        self.case = case;
    }
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

#[must_use]
pub struct Platform<'a> {
    parent: &'a Cache,
    is_dir: Option<bool>,
}

impl Cache {
    /// Create a new instance with `worktree_root` being the base for all future paths we handle, assuming it to be valid which includes
    /// symbolic links to be included in it as well.
    /// The `case` configures attribute and exclusion query case sensitivity.
    pub fn new(
        worktree_root: impl Into<PathBuf>,
        state: State,
        case: gix_glob::pattern::Case,
        buf: Vec<u8>,
        attribute_files_in_index: Vec<PathOidMapping>,
    ) -> Self {
        let root = worktree_root.into();
        Cache {
            stack: fs::Stack::new(root),
            state,
            case,
            buf,
            attribute_files_in_index,
        }
    }

    /// Append the `relative` path to the root directory the cache contains and efficiently create leading directories
    /// unless `is_dir` is known (`Some(…)`) then `relative` points to a directory itself in which case the entire resulting
    /// path is created as directory. If it's not known it is assumed to be a file.
    ///
    /// Provide access to cached information for that `relative` entry via the platform returned.
    pub fn at_path<Find, E>(
        &mut self,
        relative: impl AsRef<Path>,
        is_dir: Option<bool>,
        find: Find,
    ) -> std::io::Result<Platform<'_>>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<gix_object::BlobRef<'a>, E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        let mut delegate = platform::StackDelegate {
            state: &mut self.state,
            buf: &mut self.buf,
            is_dir: is_dir.unwrap_or(false),
            attribute_files_in_index: &self.attribute_files_in_index,
            find,
        };
        self.stack.make_relative_path_current(relative, &mut delegate)?;
        Ok(Platform { parent: self, is_dir })
    }

    /// **Panics** on illformed UTF8 in `relative`
    // TODO: more docs
    pub fn at_entry<'r, Find, E>(
        &mut self,
        relative: impl Into<&'r BStr>,
        is_dir: Option<bool>,
        find: Find,
    ) -> std::io::Result<Platform<'_>>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<gix_object::BlobRef<'a>, E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        let relative = relative.into();
        let relative_path = gix_path::from_bstr(relative);

        self.at_path(
            relative_path,
            is_dir.or_else(|| relative.ends_with_str("/").then_some(true)),
            // is_dir,
            find,
        )
    }

    /// Return the base path against which all entries or paths should be relative to when querying.
    ///
    /// Note that this path _may_ not be canonicalized.
    pub fn base(&self) -> &Path {
        self.stack.root()
    }
}

mod platform;
///
pub mod state;
