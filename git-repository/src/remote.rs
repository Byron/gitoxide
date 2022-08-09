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
    use crate::{remote, Remote};
    use git_refspec::RefSpec;

    impl Remote<'_> {
        /// Return the name of this remote or `None` if it wasn't persisted to disk yet.
        pub fn name(&self) -> Option<&str> {
            self.name.as_deref()
        }

        /// Return the set of ref-specs used for `direction`, which may be empty, in order of occurrence in the configuration.
        pub fn refspecs(&self, direction: remote::Direction) -> &[RefSpec] {
            match direction {
                remote::Direction::Fetch => &self.fetch_specs,
                remote::Direction::Push => &self.push_specs,
            }
        }

        /// Return the url used for the given `direction`.
        /// For pushing, this is the `remote.<name>.pushUrl` or the `remote.<name>.url` used for fetching, and for fetching it's
        /// the `remote.<name>.url`.
        pub fn url(&self, direction: remote::Direction) -> Option<&git_url::Url> {
            match direction {
                remote::Direction::Fetch => self.url.as_ref(),
                remote::Direction::Push => self.push_url.as_ref().or_else(|| self.url.as_ref()),
            }
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
