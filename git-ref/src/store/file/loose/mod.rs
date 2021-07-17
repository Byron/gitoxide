///
pub mod find;

///
pub mod reflog;

///
pub mod iter;

mod init {
    use crate::store::file;
    use std::path::PathBuf;

    impl file::Store {
        /// Create a new instance at the given `git_dir`, which commonly is a standard git repository with a
        /// `refs/` subdirectory.
        pub fn at(git_dir: impl Into<PathBuf>, write_reflog: crate::file::WriteReflog) -> Self {
            file::Store {
                base: git_dir.into(),
                write_reflog,
            }
        }
    }

    impl<P> From<P> for file::Store
    where
        P: Into<PathBuf>,
    {
        fn from(path: P) -> Self {
            file::Store::at(path, Default::default())
        }
    }
}
