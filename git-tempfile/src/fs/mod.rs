pub mod create_dir;
pub mod remove_dir;

mod error {
    use crate::fs::create_dir::Retries;
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
            /// The retries left after running the operation
            retries_left: Retries,
            /// The original amount of retries to allow determining how many were actually used
            retries: Retries,
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
                Error::Permanent {
                    err: _,
                    dir,
                    retries_left,
                    retries,
                } => write!(
                    f,
                    "Permanently failing to create directory {:?} ({:?} of {:?})",
                    dir, retries_left, retries
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
