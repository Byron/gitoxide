//! The standard object store which should fit all needs.
use std::{cell::RefCell, ops::Deref};

use crate::Store;

/// This effectively acts like a handle but exists to be usable from the actual `crate::Handle` implementation which adds caches on top.
/// Each store is quickly cloned and contains thread-local state for shared packs.
pub struct Handle<S>
where
    S: Deref<Target = Store> + Clone,
{
    pub(crate) store: S,
    /// Defines what happens when there is no more indices to load.
    pub refresh: RefreshMode,
    /// The maximum recursion depth for resolving ref-delta base objects, that is objects referring to other objects within
    /// a pack.
    /// Recursive loops are possible only in purposefully crafted packs.
    /// This value doesn't have to be huge as in typical scenarios, these kind of objects are rare and chains supposedly are
    /// even more rare.
    pub max_recursion_depth: usize,

    /// If true, replacements will not be performed even if these are available.
    pub ignore_replacements: bool,

    pub(crate) token: Option<handle::Mode>,
    snapshot: RefCell<load_index::Snapshot>,
    packed_object_count: RefCell<Option<u64>>,
}

/// Decide what happens when all indices are loaded.
#[derive(Clone, Copy)]
pub enum RefreshMode {
    /// Check for new or changed pack indices (and pack data files) when the last known index is loaded.
    /// During runtime we will keep pack indices stable by never reusing them, however, there is the option for
    /// clearing internal caches which is likely to change pack ids and it will trigger unloading of packs as they are missing on disk.
    AfterAllIndicesLoaded,
    /// Use this if you expect a lot of missing objects that shouldn't trigger refreshes even after all packs are loaded.
    /// This comes at the risk of not learning that the packs have changed in the mean time.
    Never,
}

impl Default for RefreshMode {
    fn default() -> Self {
        RefreshMode::AfterAllIndicesLoaded
    }
}

impl RefreshMode {
    /// Set this refresh mode to never refresh.
    pub fn never(&mut self) {
        *self = RefreshMode::Never;
    }
}

///
pub mod find;

///
pub mod prefix;

mod header;

///
pub mod iter;

///
pub mod write;

///
pub mod init;

pub(crate) mod types;
pub use types::Metrics;

pub(crate) mod handle;

///
pub mod load_index;

///
pub mod verify;

mod load_one;

mod metrics;

mod access;

///
pub mod structure {
    use std::path::PathBuf;

    use crate::{store::load_index, types::IndexAndPacks, Store};

    /// A record of a structural element of an object database.
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub enum Record {
        /// A loose object database.
        LooseObjectDatabase {
            /// The root of the object database.
            objects_directory: PathBuf,
            /// The amount of object files.
            num_objects: usize,
        },
        /// A pack index file
        Index {
            /// The location of the index file,
            path: PathBuf,
            /// Whether or not the index is mapped into memory.
            state: IndexState,
        },
        /// A multi-index file
        MultiIndex {
            /// The location of the multi-index file,
            path: PathBuf,
            /// Whether or not the index is mapped into memory.
            state: IndexState,
        },
        /// An empty slot was encountered, this is possibly happening as the ODB changes during query with
        /// a file being removed.
        Empty,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    /// Possible stats of pack indices.
    pub enum IndexState {
        /// The index is active in memory because a mapping exists.
        Loaded,
        /// The index couldn't be unloaded as it was still in use, but that can happen another time.
        Disposable,
        /// The index isn't loaded/memory mapped.
        Unloaded,
    }

    impl Store {
        /// Return information about all files known to us as well as their loading state.
        ///
        /// Note that this call is expensive as it gathers additional information about loose object databases.
        /// Note that it may change as we collect information due to the highly volatile nature of the
        /// implementation. The likelihood of actual changes is low though as these still depend on something
        /// changing on disk and somebody reading at the same time.
        pub fn structure(&self) -> Result<Vec<Record>, load_index::Error> {
            let index = self.index.load();
            if !index.is_initialized() {
                self.consolidate_with_disk_state(true, false /*load one new index*/)?;
            }
            let index = self.index.load();
            let mut res: Vec<_> = index
                .loose_dbs
                .iter()
                .map(|db| Record::LooseObjectDatabase {
                    objects_directory: db.path.clone(),
                    num_objects: db.iter().count(),
                })
                .collect();

            for slot in index.slot_indices.iter().map(|idx| &self.files[*idx]) {
                let files = slot.files.load();
                let record = match &**files {
                    Some(index) => {
                        let state = if index.is_disposable() {
                            IndexState::Disposable
                        } else if index.index_is_loaded() {
                            IndexState::Loaded
                        } else {
                            IndexState::Unloaded
                        };
                        match index {
                            IndexAndPacks::Index(b) => Record::Index {
                                path: b.index.path().into(),
                                state,
                            },
                            IndexAndPacks::MultiIndex(b) => Record::MultiIndex {
                                path: b.multi_index.path().into(),
                                state,
                            },
                        }
                    }
                    None => Record::Empty,
                };
                res.push(record);
            }
            Ok(res)
        }
    }
}
