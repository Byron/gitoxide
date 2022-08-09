mod errors {
    ///
    pub mod find {
        /// The error returned by [`Repository::find_remote(…)`][crate::Repository::find_remote()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error(transparent)]
            RefSpec(#[from] git_refspec::parse::Error),
            #[error("Neither 'url` nor 'pushUrl' fields were set in the remote's configuration.")]
            UrlMissing,
            #[error("The {kind} url couldn't be parsed")]
            UrlInvalid {
                kind: &'static str,
                source: git_url::parse::Error,
            },
        }

        ///
        pub mod existing {
            /// The error returned by [`Repository::find_remote(…)`][crate::Repository::find_remote()].
            #[derive(Debug, thiserror::Error)]
            #[allow(missing_docs)]
            pub enum Error {
                #[error(transparent)]
                Find(#[from] super::Error),
                #[error("The remote named {name:?} did not exist")]
                NotFound { name: String },
            }
        }
    }
}
pub use errors::find;

mod access {
    use crate::Remote;

    impl Remote<'_> {
        /// Return the name of this remote or `None` if it wasn't persisted to disk yet.
        pub fn name(&self) -> Option<&str> {
            self.name.as_deref()
        }
    }
}

/// The direction of an operation carried out (or to be carried out) through a remote.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Direction {
    /// Push local changes to the remote.
    Push,
    /// Fetch changes from the remote to the local repository.
    Fetch,
}
