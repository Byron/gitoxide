///
pub mod find_one;

///
pub mod reflog;

mod init {
    use crate::store::file;
    use std::path::PathBuf;

    impl file::Store {
        /// Create a new instance at the given `git_dir`, which commonly is a standard git repository with a
        /// `refs/` subdirectory.
        pub fn at(git_dir: impl Into<PathBuf>, write_reflog: crate::file::WriteReflog, hash: git_hash::Kind) -> Self {
            file::Store {
                base: git_dir.into(),
                write_reflog,
                hash,
            }
        }
    }

    impl<P> From<P> for file::Store
    where
        P: Into<PathBuf>,
    {
        fn from(path: P) -> Self {
            file::Store::at(path, Default::default(), git_hash::Kind::default())
        }
    }
}
