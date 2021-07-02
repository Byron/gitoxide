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
    #![allow(unused)]
    use crate::{edit, store::file::Store};

    mod error {
        use quick_error::quick_error;
        quick_error! {
            /// The error returned by various [`Transaction`][super::Transaction] methods.
            #[derive(Debug)]
            #[allow(missing_docs)]
            pub enum Error {
                Tbd
            }
        }
    }
    use crate::edit::RefEdit;
    pub use error::Error;

    struct Edit {
        update: RefEdit,
        lock: Option<git_lock::Marker>,
        /// Set if this update is coming from a symbolic reference and used to make it appear like it is the one that is handled,
        /// instead of the referent reference.
        parent_index: Option<usize>,
    }

    /// A transaction
    pub struct Transaction {
        updates: Vec<Edit>,
        state: State,
    }

    impl Transaction {
        /// Discard the transaction and re-obtain the initial edits
        pub fn into_edits(self) -> Vec<RefEdit> {
            self.updates.into_iter().map(|e| e.update).collect()
        }

        /// Prepare for calling [`commit(…)`][Transaction::commit()] in a way that can be rolled back perfectly.
        ///
        /// If the operation succeeds, the transaction can be committed or dropped to cause a rollback automatically.
        /// Rollbacks happen automatically on failure.
        /// This method is idempotent.
        pub fn prepare(mut self) -> Result<Self, Error> {
            Ok(match self.state {
                State::Prepared => self,
                State::Open => todo!("transaction prep"),
            })
        }

        /// Make all [prepared][Transaction::prepare()] permanent and return the performed edits which represent the current
        /// state of the affected refs in the ref store in that instant. Please note that the obtained edits may have been
        /// adjusted to contain more dependent edits or additional information.
        ///
        /// On error the transaction may have been performed partially and can be retried, depending on the nature of the error.
        ///
        /// Note that transactions will be prepared automatically as needed.
        pub fn commit(mut self) -> Result<Vec<RefEdit>, Error> {
            match self.state {
                State::Open => self.prepare()?.commit(),
                State::Prepared => {
                    todo!("transaction commit")
                }
            }
        }
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
        pub fn transaction(&self, edits: impl IntoIterator<Item = RefEdit>) -> Transaction {
            Transaction {
                updates: edits
                    .into_iter()
                    .map(|update| Edit {
                        update,
                        lock: None,
                        parent_index: None,
                    })
                    .collect(),
                state: State::Open,
            }
        }
    }
}
