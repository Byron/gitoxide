#![allow(missing_docs, unused)]
use filebuffer::FileBuffer;

/// A representation of an index file for multiple packs at the same time, typically stored in a file
/// named 'multi-pack-index'.
pub struct File {
    data: FileBuffer,
    path: std::path::PathBuf,
}

///
pub mod init {
    use crate::multi_index::File;
    use std::convert::TryFrom;
    use std::path::Path;

    mod error {
        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error(transparent)]
            Io(#[from] std::io::Error),
        }
    }
    pub use error::Error;

    impl File {
        pub fn at(path: impl AsRef<Path>) -> Result<Self, Error> {
            Self::try_from(path.as_ref())
        }
    }

    impl TryFrom<&Path> for File {
        type Error = Error;

        fn try_from(path: &Path) -> Result<Self, Self::Error> {
            todo!()
        }
    }
}
