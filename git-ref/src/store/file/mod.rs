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

/// A git _ref_ which is stored in a file.
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
pub struct Reference<'a> {
    parent: &'a Store,
    /// The path to uniquely identify this ref within its store.
    pub relative_path: PathBuf,
    state: reference::State,
}

/// A store for reference which uses plain files.
///
/// Each ref is represented as a single file on disk in a folder structure that follows the relative path
/// used to identify [references][Reference].
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
pub struct Store {
    /// The location at which loose references can be found as per conventions of a typical git repository.
    ///
    /// Typical base paths are `.git` repository folders.
    pub base: PathBuf,
    /// The way to handle reflog edits
    pub write_reflog: WriteReflog,
}

mod loose;
pub use loose::find_one;

///
pub mod reference;

///
pub mod log;

///
pub mod transaction {
    use crate::{edit, store::file::Store};

    /// A transaction
    pub struct Transaction {
        updates: Vec<edit::Reference>,
        state: State,
    }

    /// The state of a [`Transaction`]
    #[allow(missing_docs)]
    pub enum State {
        Open,
        Prepared,
    }

    /// Edits
    impl Store {
        /// Open a transaction with the given `edits`.
        pub fn transaction(edits: impl IntoIterator<Item = edit::Reference>) -> Transaction {
            // store.transaction(edits)?.prepare()?.commit()?;
            todo!("prepared transaction")
        }
    }
}
