use crate::{clone::PrepareFetch, Repository};

/// The error returned by [`PrepareFetch::fetch_only()`].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
#[cfg(feature = "blocking-network-client")]
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

/// Modification
impl PrepareFetch {
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
    ) -> Result<(Repository, crate::remote::fetch::Outcome), Error> {
        fn replace_changed_local_config(repo: &mut Repository, config: git_config::File<'static>) {
            let repo_config = git_features::threading::OwnShared::make_mut(&mut repo.config.resolved);
            let ids_to_remove: Vec<_> = repo_config
                .sections_and_ids()
                .filter_map(|(s, id)| (s.meta().source == git_config::Source::Local).then(|| id))
                .collect();
            for id in ids_to_remove {
                repo_config.remove_section_by_id(id);
            }
            repo_config.append(config);
        }

        fn write_remote_to_local_config(
            remote: &mut crate::Remote<'_>,
            remote_name: String,
        ) -> Result<git_config::File<'static>, Error> {
            let mut metadata = git_config::file::Metadata::from(git_config::Source::Local);
            let config_path = remote.repo.git_dir().join("config");
            metadata.path = Some(config_path.clone());
            let mut config =
                git_config::File::from_paths_metadata(Some(metadata), Default::default())?.expect("one file to load");
            remote.save_as_to(remote_name, &mut config)?;
            std::fs::write(config_path, config.to_bstring())?;
            Ok(config)
        }

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

        let config = write_remote_to_local_config(&mut remote, remote_name)?;
        let pending_pack: crate::remote::fetch::Prepare<'_, '_, _, _> = remote
            .connect(crate::remote::Direction::Fetch, progress)?
            .prepare_fetch(self.fetch_options.clone())?;
        if pending_pack.ref_map().object_hash != repo.object_hash() {
            unimplemented!("configure repository to expect a different object hash as advertised by the server")
        }
        let outcome = pending_pack.receive(should_interrupt)?;

        replace_changed_local_config(repo, config);
        Ok((self.repo.take().expect("still present"), outcome))
    }

    /// Similar to [`fetch_only()`][Self::fetch_only()`], but passes ownership to a utility type to configure a checkout operation.
    #[cfg(feature = "blocking-network-client")]
    pub fn fetch_then_checkout(
        &mut self,
        progress: impl crate::Progress,
        should_interrupt: &std::sync::atomic::AtomicBool,
    ) -> Result<(crate::clone::PrepareCheckout, crate::remote::fetch::Outcome), Error> {
        let (repo, fetch_outcome) = self.fetch_only(progress, should_interrupt)?;
        Ok((crate::clone::PrepareCheckout { repo: repo.into() }, fetch_outcome))
    }
}

/// Builder
impl PrepareFetch {
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
impl PrepareFetch {
    /// Persist the contained repository as is even if an error may have occurred when fetching from the remote.
    pub fn persist(mut self) -> Repository {
        self.repo.take().expect("present and consumed once")
    }
}

impl Drop for PrepareFetch {
    fn drop(&mut self) {
        if let Some(repo) = self.repo.take() {
            std::fs::remove_dir_all(repo.work_dir().unwrap_or_else(|| repo.path())).ok();
        }
    }
}

impl From<PrepareFetch> for Repository {
    fn from(prep: PrepareFetch) -> Self {
        prep.persist()
    }
}
