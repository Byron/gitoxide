use std::path::PathBuf;

/// The way a file store handles the reflog
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone, Copy)]
pub enum WriteReflog {
    /// Write a ref log for ref edits according to the standard rules.
    Normal,
    /// Never write a ref log.
    Disable,
}

impl Default for WriteReflog {
    fn default() -> Self {
        WriteReflog::Normal
    }
}

/// A store for reference which uses plain files.
///
/// Each ref is represented as a single file on disk in a folder structure that follows the relative path
/// used to identify [references][crate::Reference].
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
pub struct Store {
    /// The location at which loose references can be found as per conventions of a typical git repository.
    ///
    /// Typical base paths are `.git` repository folders.
    pub base: PathBuf,
    /// The way to handle reflog edits
    pub write_reflog: WriteReflog,
    /// The namespace to use for edits and reads
    pub namespace: Option<Namespace>,
}

/// A transaction on a file store
pub struct Transaction<'s> {
    store: &'s Store,
    packed_transaction: Option<crate::store::packed::Transaction>,
    updates: Option<Vec<transaction::Edit>>,
    packed_refs: transaction::PackedRefs,
    namespace: Option<crate::Namespace>,
}

pub(in crate::store::file) fn path_to_name(path: impl Into<PathBuf>) -> git_object::bstr::BString {
    use os_str_bytes::OsStringBytes;
    let path = path.into().into_raw_vec();
    #[cfg(windows)]
    let path = {
        use git_object::bstr::ByteSlice;
        path.replace(b"\\", b"/")
    };
    path.into()
}

///
pub mod loose;
mod overlay_iter;

///
pub mod iter {
    pub use super::{
        loose::iter::{loose, Loose},
        overlay_iter::LooseThenPacked,
    };

    ///
    pub mod loose_then_packed {
        pub use super::super::overlay_iter::Error;
    }
}

///
pub mod log;

///
pub mod find;

///
pub mod transaction;

///
pub mod packed;

mod raw_ext;
use crate::Namespace;
pub use raw_ext::ReferenceExt;
