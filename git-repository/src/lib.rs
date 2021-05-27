#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]
#![allow(missing_docs)]

pub mod init;

pub mod discover {
    use quick_error::quick_error;
    use std::path::{Path, PathBuf};
    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Io(err: std::io::Error) {
                display("Failed to access a directory")
                from()
                source(err)
            }
        }
    }

    pub fn path(directory: impl AsRef<Path>) -> Result<PathBuf, Error> {
        todo!("discover path")
    }

    pub fn is_git(git_dir: impl AsRef<Path>) -> Result<bool, Error> {
        todo!("is a git repository")
    }
}
