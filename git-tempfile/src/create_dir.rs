#![allow(missing_docs)]
use std::path::Path;

mod error {
    use quick_error::quick_error;
    use std::path::PathBuf;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Intermediate(kind: std::io::ErrorKind) {
                display("Intermediate failure with error: {:?}", kind)
                from()
            }
            Permanent(err: std::io::Error, dir: PathBuf) {
                display("Permanently failing to create directory {:?}", dir)
                source(err)
            }
        }
    }

    impl Error {
        pub fn intermediate(&self) -> Option<std::io::ErrorKind> {
            match self {
                Error::Intermediate(kind) => Some(*kind),
                _ => None,
            }
        }
    }
}
pub use error::Error;

/// A special iterator which communicates its operation through results where…
///
/// * `Some(Ok(created_directory))` is yielded exactly once on success, following by `None`
/// * `Some(Err(Error::Intermediate))` is yielded zero or more times while trying to create the directory.
/// * `Some(Err(Error::Permanent))` is yielded exactly once on failure.
pub struct Iter<'a> {
    cursors: Vec<&'a Path>,
}

impl<'a> Iter<'a> {
    pub fn new(target: &'a Path) -> Self {
        Iter { cursors: vec![target] }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Result<&'a Path, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        use std::io::ErrorKind::*;
        match self.cursors.pop() {
            Some(cursor) => match std::fs::create_dir(cursor) {
                Ok(()) => Some(Ok(cursor)),
                Err(err) => match err.kind() {
                    AlreadyExists => Some(Ok(cursor)),
                    _ => todo!("other errors"),
                },
            },
            None => None,
        }
    }
}
