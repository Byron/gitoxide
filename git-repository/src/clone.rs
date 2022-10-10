type ConfigureRemoteFn = Box<dyn FnMut(crate::Remote<'_>) -> Result<crate::Remote<'_>, crate::remote::init::Error>>;

/// A utility to collect configuration on how to fetch from a remote and possibly create a working tree locally.
pub struct Prepare {
    /// A freshly initialized repository which is owned by us, or `None` if it was handed to the user
    repo: Option<crate::Repository>,
    /// The name of the remote, which defaults to `origin` if not overridden.
    remote_name: Option<String>,
    /// A function to configure a remote prior to fetching a pack.
    configure_remote: Option<ConfigureRemoteFn>,
    /// Options for preparing a fetch operation.
    #[cfg(any(feature = "async-network-client", feature = "blocking-network-client"))]
    fetch_options: crate::remote::ref_map::Options,
    /// The url to clone from
    #[allow(dead_code)]
    url: git_url::Url,
}

///
#[cfg(feature = "blocking-network-client")]
pub mod fetch {
    /// The error returned by [`Prepare::fetch_only()`][super::Prepare::fetch_only()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Connect(#[from] crate::remote::connect::Error),
        #[error(transparent)]
        PrepareFetch(#[from] crate::remote::fetch::prepare::Error),
        #[error(transparent)]
        Fetch(#[from] crate::remote::fetch::Error),
        #[error(transparent)]
        RemoteConfiguration(#[from] crate::remote::init::Error),
        #[error("Default remote configured at `clone.defaultRemoteName` is invalid")]
        RemoteName(#[from] crate::remote::name::Error),
        #[error("Failed to load repo-local git configuration before writing")]
        LoadConfig(#[from] git_config::file::init::from_paths::Error),
        #[error("Failed to store configured remote in memory")]
        SaveConfig(#[from] crate::remote::save::AsError),
        #[error("Failed to write repository configuration to disk")]
        SaveConfigIo(#[from] std::io::Error),
    }
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
                #[cfg(any(feature = "async-network-client", feature = "blocking-network-client"))]
                fetch_options: Default::default(),
                repo: Some(repo),
                remote_name: None,
                configure_remote: None,
            })
        }
    }

    /// Modification
    impl Prepare {
        /// Fetch a pack and update local branches according to refspecs, providing `progress` and checking `should_interrupt` to stop
        /// the operation.
        /// On success, the persisted repository is returned, and this method must not be called again to avoid a **panic**.
        /// On error, the method may be called again to retry as often as needed.
        ///
        /// Note that all data we created will be removed once this instance drops if the operation wasn't successful.
        #[cfg(feature = "blocking-network-client")]
        pub fn fetch_only(
            &mut self,
            progress: impl crate::Progress,
            should_interrupt: &std::sync::atomic::AtomicBool,
        ) -> Result<(Repository, crate::remote::fetch::Outcome), super::fetch::Error> {
            let repo = self
                .repo
                .as_mut()
                .expect("user error: multiple calls are allowed only until it succeeds");

            let remote_name = match self.remote_name.as_deref() {
                Some(name) => name.to_owned(),
                None => repo
                    .config
                    .resolved
                    .string("clone", None, "defaultRemoteName")
                    .map(|n| crate::remote::name::validated(n.to_string()))
                    .unwrap_or_else(|| Ok("origin".into()))?,
            };

            let mut remote = repo
                .remote_at(self.url.clone())?
                .with_refspec("+refs/heads/*:refs/remotes/origin/*", crate::remote::Direction::Fetch)
                .expect("valid static spec");
            if let Some(f) = self.configure_remote.as_mut() {
                remote = f(remote)?;
            }

            let mut metadata = git_config::file::Metadata::from(git_config::Source::Local);
            let config_path = repo.git_dir().join("config");
            metadata.path = Some(config_path.clone());
            let mut config =
                git_config::File::from_paths_metadata(Some(metadata), Default::default())?.expect("one file to load");
            remote.save_as_to(remote_name, &mut config)?;
            std::fs::write(config_path, config.to_bstring())?;

            let outcome = remote
                .connect(crate::remote::Direction::Fetch, progress)?
                .prepare_fetch(self.fetch_options.clone())?
                .receive(should_interrupt)?;

            let repo_config = git_features::threading::OwnShared::make_mut(&mut repo.config.resolved);
            let ids_to_remove: Vec<_> = repo_config
                .sections_and_ids()
                .filter_map(|(s, id)| (s.meta().source == git_config::Source::Local).then(|| id))
                .collect();
            for id in ids_to_remove {
                repo_config.remove_section_by_id(id);
            }
            repo_config.append(config);

            Ok((self.repo.take().expect("still present"), outcome))
        }
    }

    /// Builder
    impl Prepare {
        /// Set additional options to adjust parts of the fetch operation that are not affected by the git configuration.
        #[cfg(any(feature = "async-network-client", feature = "blocking-network-client"))]
        pub fn with_fetch_options(mut self, opts: crate::remote::ref_map::Options) -> Self {
            self.fetch_options = opts;
            self
        }
        /// Use `f` to apply arbitrary changes to the remote that is about to be used to fetch a pack.
        ///
        /// The passed in `remote` will be un-named and pre-configured to be a default remote as we know it from git-clone.
        /// It is not yet present in the configuration of the repository,
        /// but each change it will eventually be written to the configuration prior to performing a the fetch operation.
        pub fn configure_remote(
            mut self,
            f: impl FnMut(crate::Remote<'_>) -> Result<crate::Remote<'_>, crate::remote::init::Error> + 'static,
        ) -> Self {
            self.configure_remote = Some(Box::new(f));
            self
        }

        /// Set the remote's name to the given value after it was configured using the function provided via
        /// [`configure_remote()`][Self::configure_remote()].
        ///
        /// If not set here, it defaults to `origin` or the value of `clone.defaultRemoteName`.
        pub fn with_remote_name(mut self, name: impl Into<String>) -> Result<Self, crate::remote::name::Error> {
            self.remote_name = Some(crate::remote::name::validated(name)?);
            Ok(self)
        }
    }

    /// Consumption
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
