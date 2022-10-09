type ConfigureRemoteFn = Box<dyn FnOnce(crate::Remote<'_>) -> Result<crate::Remote<'_>, crate::remote::init::Error>>;

/// A utility to collect configuration on how to fetch from a remote and possibly create a working tree locally.
pub struct Prepare {
    /// A freshly initialized repository which is owned by us, or `None` if it was handed to the user
    repo: Option<crate::Repository>,
    /// The name of the remote, which defaults to `origin` if not overridden.
    #[allow(dead_code)]
    remote_name: Option<String>,
    /// A function to configure a remote prior to fetching a pack.
    configure_remote: Option<ConfigureRemoteFn>,
    /// The url to clone from
    #[allow(dead_code)]
    url: git_url::Url,
}

///
pub mod prepare {
    use crate::clone::Prepare;
    use crate::Repository;
    use std::convert::TryInto;

    /// The error returned by [`Prepare::new()`].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Init(#[from] crate::init::Error),
        #[error(transparent)]
        UrlParse(#[from] git_url::parse::Error),
    }

    /// Instantiation
    impl Prepare {
        /// Create a new repository at `path` with `crate_opts` which is ready to clone from `url`, possibly after making additional adjustments to
        /// configuration and settings.
        ///
        /// Note that this is merely a handle to perform the actual connection to the remote, and if any of it fails the freshly initialized repository
        /// will be removed automatically as soon as this instance drops.
        pub fn new<Url, E>(
            url: Url,
            path: impl AsRef<std::path::Path>,
            create_opts: crate::create::Options,
            open_opts: crate::open::Options,
        ) -> Result<Self, Error>
        where
            Url: TryInto<git_url::Url, Error = E>,
            git_url::parse::Error: From<E>,
        {
            let url = url.try_into().map_err(git_url::parse::Error::from)?;
            let repo = crate::ThreadSafeRepository::init_opts(path, create_opts, open_opts)?.to_thread_local();
            Ok(Prepare {
                url,
                repo: Some(repo),
                remote_name: None,
                configure_remote: None,
            })
        }
    }

    /// Builder
    impl Prepare {
        /// Use `f` to apply arbitrary changes to the remote that is about to be used to fetch a pack.
        ///
        /// The passed in `remote` will be un-named and pre-configured to be a default remote as we know it from git-clone.
        /// It is not yet present in the configuration of the repository,
        /// but each change it will eventually be written to the configuration prior to performing a the fetch operation.
        pub fn configure_remote(
            mut self,
            f: impl FnOnce(crate::Remote<'_>) -> Result<crate::Remote<'_>, crate::remote::init::Error> + 'static,
        ) -> Self {
            self.configure_remote = Some(Box::new(f));
            self
        }
    }

    /// Access
    impl Prepare {
        /// Persist the contained repository as is even if an error may have occurred when interacting with the remote or checking out the main working tree.
        pub fn persist(mut self) -> Repository {
            self.repo.take().expect("present and consumed once")
        }
    }

    impl Drop for Prepare {
        fn drop(&mut self) {
            if let Some(repo) = self.repo.take() {
                std::fs::remove_dir_all(repo.work_dir().unwrap_or_else(|| repo.path())).ok();
            }
        }
    }

    impl From<Prepare> for Repository {
        fn from(prep: Prepare) -> Self {
            prep.persist()
        }
    }
}
