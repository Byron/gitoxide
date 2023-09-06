use crate::{FullName, Kind, Target};

/// A git _ref_ which is stored in a file.
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Clone)]
pub struct Reference {
    /// The path to uniquely identify this ref within its store.
    pub name: FullName,
    /// The target of the reference, either a symbolic reference by full name or an object by its id.
    pub target: Target,
}

impl Reference {
    /// Return the kind of ref.
    pub fn kind(&self) -> Kind {
        self.target.kind()
    }
}

///
pub(crate) mod reflog;

///
pub(crate) mod iter;
///
pub mod reference;

mod init {
    use std::path::PathBuf;

    use crate::store_impl::file;

    impl file::Store {
        /// Create a new instance at the given `git_dir`, which commonly is a standard git repository with a
        /// `refs/` subdirectory.
        /// The `object_hash` defines which kind of hash we should recognize.
        pub fn at(git_dir: PathBuf, write_reflog: file::WriteReflog, object_hash: gix_hash::Kind) -> Self {
            file::Store {
                git_dir,
                common_dir: None,
                write_reflog,
                namespace: None,
                packed: gix_fs::SharedFileSnapshotMut::new().into(),
                object_hash,
            }
        }

        /// Like [`at()`][file::Store::at()], but for _linked_ work-trees which use `git_dir` as private ref store and `common_dir` for
        /// shared references.
        pub fn for_linked_worktree(
            git_dir: PathBuf,
            common_dir: PathBuf,
            write_reflog: file::WriteReflog,
            object_hash: gix_hash::Kind,
        ) -> Self {
            file::Store {
                git_dir,
                common_dir: Some(common_dir),
                write_reflog,
                namespace: None,
                packed: gix_fs::SharedFileSnapshotMut::new().into(),
                object_hash,
            }
        }
    }
}
