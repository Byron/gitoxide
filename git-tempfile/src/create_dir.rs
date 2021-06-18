//!
use std::path::Path;

/// The amount of retries to do during various aspects of the directory creation.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Retries {
    /// How many directories can be created in total. 1 means only the target directory itself can be created and
    /// not a single parent directory.
    /// Note that this also counts towards retries needed to combat racy behaviour from other
    /// processes trying to delete empty directories.
    pub on_create_directory: usize,
    /// How often to retry if an interrupt happens.
    pub on_interrupt: usize,
}

impl Default for Retries {
    fn default() -> Self {
        Retries {
            on_interrupt: 10,
            on_create_directory: 100,
        }
    }
}

mod error {
    use std::{fmt, path::Path};

    /// The error returned by [all()][super::all()].
    #[allow(missing_docs)]
    #[derive(Debug)]
    pub enum Error<'a> {
        /// A failure we will probably recover from by trying again.
        Intermediate { dir: &'a Path, kind: std::io::ErrorKind },
        /// A failure that ends the operation.
        Permanent {
            dir: &'a Path,
            err: std::io::Error,
            attempts: Option<usize>,
        },
    }

    impl<'a> fmt::Display for Error<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Error::Intermediate { dir, kind } => write!(
                    f,
                    "Intermediae failure creating {:?} with error: {:?}",
                    dir.display(),
                    kind
                ),
                Error::Permanent { err: _, dir, attempts } => write!(
                    f,
                    "Permanently failing to create directory {:?}{}",
                    dir,
                    match attempts {
                        Some(attempts) => std::borrow::Cow::from(format!(" after {} attempts", attempts)),
                        None => "".into(),
                    }
                ),
            }
        }
    }

    impl<'a> std::error::Error for Error<'a> {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                Error::Permanent { err, .. } => Some(err),
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
    retries: Retries,
    original_retries: Retries,
}

impl<'a> Iter<'a> {
    /// Create a new instance that creates `target` when iterated with the default amount of [`Retries`].
    pub fn new(target: &'a Path) -> Self {
        let retries = Default::default();
        Iter {
            cursors: vec![target],
            original_retries: retries,
            retries,
        }
    }

    /// Create a new instance that creates `target` when iterated with the specified amount of `retries`.
    pub fn new_with_retries(target: &'a Path, retries: Retries) -> Self {
        Iter {
            cursors: vec![target],
            original_retries: retries,
            retries,
        }
    }

    fn pernanent_failure(
        &mut self,
        dir: &'a Path,
        err: impl Into<std::io::Error>,
        attempts: impl Into<Option<usize>>,
    ) -> Option<Result<&'a Path, Error<'a>>> {
        self.cursors.clear();
        Some(Err(Error::Permanent {
            err: err.into(),
            dir,
            attempts: attempts.into(),
        }))
    }

    fn intermediate_failure(&self, dir: &'a Path, err: std::io::Error) -> Option<Result<&'a Path, Error<'a>>> {
        Some(Err(Error::Intermediate { dir, kind: err.kind() }))
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Result<&'a Path, Error<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        use std::io::ErrorKind::*;
        match self.cursors.pop() {
            Some(dir) => match std::fs::create_dir(dir) {
                Ok(()) => Some(Ok(dir)),
                Err(err) => match err.kind() {
                    AlreadyExists if dir.is_dir() => Some(Ok(dir)),
                    AlreadyExists => self.pernanent_failure(dir, err, None),
                    NotFound if self.retries.on_create_directory <= 1 => {
                        self.pernanent_failure(dir, NotFound, self.original_retries.on_create_directory)
                    }
                    NotFound => {
                        self.retries.on_create_directory -= 1;
                        self.cursors.push(dir);
                        self.cursors.push(match dir.parent() {
                            None => return self.pernanent_failure(dir, InvalidInput, 1),
                            Some(parent) => parent,
                        });
                        self.intermediate_failure(dir, err)
                    }
                    Interrupted if self.retries.on_interrupt <= 1 => {
                        self.pernanent_failure(dir, Interrupted, self.original_retries.on_interrupt)
                    }
                    Interrupted => {
                        self.retries.on_interrupt -= 1;
                        self.cursors.push(dir);
                        self.intermediate_failure(dir, err)
                    }
                    _unexpected_kind => self.pernanent_failure(dir, err, None),
                },
            },
            None => None,
        }
    }
}

/// Create all directories leading to `dir` including `dir` itself with the specified amount of `retries`.
/// Returns the input `dir` on success that make it useful in expressions.
pub fn all(dir: &Path, retries: Retries) -> std::io::Result<&Path> {
    for res in Iter::new_with_retries(dir, retries) {
        match res {
            Err(Error::Permanent { err, .. }) => return Err(err),
            Err(Error::Intermediate { .. }) | Ok(_) => continue,
        }
    }
    Ok(dir)
}
