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
        pub fn at(
            git_dir: impl Into<PathBuf>,
            write_reflog: crate::file::WriteReflog,
            object_hash: git_hash::Kind,
        ) -> Self {
            file::Store {
                base: git_dir.into(),
                write_reflog,
                namespace: None,
                packed: Default::default(),
                object_hash,
            }
        }
    }
}
