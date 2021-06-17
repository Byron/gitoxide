#![allow(missing_docs)]
use std::path::Path;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Options {
    /// How often to retry if an interrupt happens.
    retries_on_interrupt: usize,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            retries_on_interrupt: 10,
        }
    }
}

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
            Permanent{ err: std::io::Error, dir: PathBuf } {
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
/// * `Some(Ok(created_directory))` is yielded once or more success, followed by `None`
/// * `Some(Err(Error::Intermediate))` is yielded zero or more times while trying to create the directory.
/// * `Some(Err(Error::Permanent))` is yielded exactly once on failure.
pub struct Iter<'a> {
    cursors: Vec<&'a Path>,
    options: Options,
}

impl<'a> Iter<'a> {
    pub fn new(target: &'a Path) -> Self {
        Iter {
            cursors: vec![target],
            options: Default::default(),
        }
    }

    fn pernanent_failure(&mut self, dir: &Path, err: impl Into<std::io::Error>) -> Option<Result<&'a Path, Error>> {
        self.cursors.clear();
        Some(Err(Error::Permanent {
            err: err.into(),
            dir: dir.to_owned(),
        }))
    }

    fn intermediate_failure(&self, err: std::io::Error) -> Option<Result<&'a Path, Error>> {
        Some(Err(Error::Intermediate(err.kind())))
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Result<&'a Path, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        use std::io::ErrorKind::*;
        match self.cursors.pop() {
            Some(dir) => match std::fs::create_dir(dir) {
                Ok(()) => Some(Ok(dir)),
                Err(err) => match err.kind() {
                    AlreadyExists => Some(Ok(dir)),
                    NotFound => {
                        self.cursors.push(dir);
                        self.cursors.push(match dir.parent() {
                            None => return self.pernanent_failure(dir, InvalidInput),
                            Some(parent) => parent,
                        });
                        self.intermediate_failure(err)
                    }
                    Interrupted if self.options.retries_on_interrupt == 0 => self.pernanent_failure(dir, Interrupted),
                    Interrupted => {
                        self.options.retries_on_interrupt -= 1;
                        self.cursors.push(dir);
                        self.intermediate_failure(err)
                    }
                    kind => todo!("{:?}", kind),
                },
            },
            None => None,
        }
    }
}
