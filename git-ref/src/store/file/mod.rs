use crate::{store::WriteReflog, Namespace};

use crate::bstr::BStr;

use std::borrow::Cow;
use std::path::{Path, PathBuf};

use git_features::threading::{MutableOnDemand, OwnShared};

/// A store for reference which uses plain files.
///
/// Each ref is represented as a single file on disk in a folder structure that follows the relative path
/// used to identify [references][crate::Reference].
#[derive(Debug, Clone)]
pub struct Store {
    /// The location at which loose references can be found as per conventions of a typical git repository.
    ///
    /// Typical base paths are `.git` repository folders.
    base: PathBuf,
    /// The kind of hash to assume in a couple of situations. Note that currently we are able to read any valid hash from files
    /// which might want to change one day.
    object_hash: git_hash::Kind,

    /// The way to handle reflog edits
    pub write_reflog: WriteReflog,
    /// The namespace to use for edits and reads
    pub namespace: Option<Namespace>,
    /// A packed buffer which can be mapped in one version and shared as such.
    /// It's updated only in one spot, which is prior to reading it based on file stamps.
    /// Doing it like this has the benefit of being able to hand snapshots out to people without blocking others from updating it.
    packed: OwnShared<MutableOnDemand<packed::modifiable::State>>,
}

mod access {
    use std::path::Path;

    use crate::file;

    impl file::Store {
        /// Return the root at which all references are loaded.
        pub fn base(&self) -> &Path {
            &self.base
        }
    }
}

/// A transaction on a file store
pub struct Transaction<'s> {
    store: &'s Store,
    packed_transaction: Option<crate::store_impl::packed::Transaction>,
    updates: Option<Vec<transaction::Edit>>,
    packed_refs: transaction::PackedRefs,
}

pub(in crate::store_impl::file) fn path_to_name<'a>(path: impl Into<Cow<'a, Path>>) -> Cow<'a, BStr> {
    let path = git_features::path::into_bytes_or_panic_on_windows(path.into());

    #[cfg(windows)]
    let path = git_features::path::convert::to_unix_separators(path);

    git_features::path::convert::into_bstr(path)
}

///
pub mod loose;
mod overlay_iter;

///
pub mod iter {
    pub use super::{
        loose::iter::{loose, Loose},
        overlay_iter::{LooseThenPacked, Platform},
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
pub use raw_ext::ReferenceExt;
