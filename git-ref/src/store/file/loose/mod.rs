///
pub(crate) mod reflog;

///
pub(crate) mod iter;

mod init {
    use std::path::PathBuf;

    use crate::store::file;

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
