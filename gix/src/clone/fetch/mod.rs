use crate::clone::PrepareFetch;

/// The error returned by [`PrepareFetch::fetch_only()`].
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
    RemoteInit(#[from] crate::remote::init::Error),
    #[error("Custom configuration of remote to clone from failed")]
    RemoteConfiguration(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("Custom configuration of connection to use when cloning failed")]
    RemoteConnection(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error(transparent)]
    RemoteName(#[from] crate::config::remote::symbolic_name::Error),
    #[error("Failed to load repo-local git configuration before writing")]
    LoadConfig(#[from] gix_config::file::init::from_paths::Error),
    #[error("Failed to store configured remote in memory")]
    SaveConfig(#[from] crate::remote::save::AsError),
    #[error("Failed to write repository configuration to disk")]
    SaveConfigIo(#[from] std::io::Error),
    #[error("The remote HEAD points to a reference named {head_ref_name:?} which is invalid.")]
    InvalidHeadRef {
        source: gix_validate::reference::name::Error,
        head_ref_name: crate::bstr::BString,
    },
    #[error("Failed to update HEAD with values from remote")]
    HeadUpdate(#[from] crate::reference::edit::Error),
}

/// Modification
impl PrepareFetch {
    /// Fetch a pack and update local branches according to refspecs, providing `progress` and checking `should_interrupt` to stop
    /// the operation.
    /// On success, the persisted repository is returned, and this method must not be called again to avoid a **panic**.
    /// On error, the method may be called again to retry as often as needed.
    ///
    /// If the remote repository was empty, that is newly initialized, the returned repository will also be empty and like
    /// it was newly initialized.
    ///
    /// Note that all data we created will be removed once this instance drops if the operation wasn't successful.
    ///
    /// ### Note for users of `async`
    ///
    /// Even though
    #[gix_protocol::maybe_async::maybe_async]
    pub async fn fetch_only<P>(
        &mut self,
        mut progress: P,
        should_interrupt: &std::sync::atomic::AtomicBool,
    ) -> Result<(crate::Repository, crate::remote::fetch::Outcome), Error>
    where
        P: crate::NestedProgress,
        P::SubProgress: 'static,
    {
        self.fetch_only_inner(&mut progress, should_interrupt).await
    }

    #[gix_protocol::maybe_async::maybe_async]
    async fn fetch_only_inner(
        &mut self,
        progress: &mut dyn crate::DynNestedProgress,
        should_interrupt: &std::sync::atomic::AtomicBool,
    ) -> Result<(crate::Repository, crate::remote::fetch::Outcome), Error> {
        use crate::{bstr::ByteVec, remote, remote::fetch::RefLogMessage};

        let repo = self
            .repo
            .as_mut()
            .expect("user error: multiple calls are allowed only until it succeeds");

        let remote_name = match self.remote_name.as_ref() {
            Some(name) => name.to_owned(),
            None => repo
                .config
                .resolved
                .string("clone", None, crate::config::tree::Clone::DEFAULT_REMOTE_NAME.name)
                .map(|n| crate::config::tree::Clone::DEFAULT_REMOTE_NAME.try_into_symbolic_name(n))
                .transpose()?
                .unwrap_or_else(|| "origin".into()),
        };

        let mut remote = repo
            .remote_at(self.url.clone())?
            .with_refspecs(
                Some(format!("+refs/heads/*:refs/remotes/{remote_name}/*").as_str()),
                remote::Direction::Fetch,
            )
            .expect("valid static spec");
        let mut clone_fetch_tags = None;
        if let Some(f) = self.configure_remote.as_mut() {
            remote = f(remote).map_err(Error::RemoteConfiguration)?;
        } else {
            clone_fetch_tags = remote::fetch::Tags::All.into();
        }

        let config = util::write_remote_to_local_config_file(&mut remote, remote_name.clone())?;

        // Now we are free to apply remote configuration we don't want to be written to disk.
        if let Some(fetch_tags) = clone_fetch_tags {
            remote = remote.with_fetch_tags(fetch_tags);
        }

        // Add HEAD after the remote was written to config, we need it to know what to checkout later, and assure
        // the ref that HEAD points to is present no matter what.
        let head_refspec = gix_refspec::parse(
            format!("HEAD:refs/remotes/{remote_name}/HEAD").as_str().into(),
            gix_refspec::parse::Operation::Fetch,
        )
        .expect("valid")
        .to_owned();
        let pending_pack: remote::fetch::Prepare<'_, '_, _> = {
            let mut connection = remote.connect(remote::Direction::Fetch).await?;
            if let Some(f) = self.configure_connection.as_mut() {
                f(&mut connection).map_err(Error::RemoteConnection)?;
            }
            connection
                .prepare_fetch(&mut *progress, {
                    let mut opts = self.fetch_options.clone();
                    if !opts.extra_refspecs.contains(&head_refspec) {
                        opts.extra_refspecs.push(head_refspec)
                    }
                    opts
                })
                .await?
        };
        if pending_pack.ref_map().object_hash != repo.object_hash() {
            unimplemented!("configure repository to expect a different object hash as advertised by the server")
        }
        let reflog_message = {
            let mut b = self.url.to_bstring();
            b.insert_str(0, "clone: from ");
            b
        };
        let outcome = pending_pack
            .with_write_packed_refs_only(true)
            .with_reflog_message(RefLogMessage::Override {
                message: reflog_message.clone(),
            })
            .with_shallow(self.shallow.clone())
            .receive_inner(progress, should_interrupt)
            .await?;

        util::append_config_to_repo_config(repo, config);
        util::update_head(
            repo,
            &outcome.ref_map.remote_refs,
            reflog_message.as_ref(),
            remote_name.as_ref(),
        )?;

        Ok((self.repo.take().expect("still present"), outcome))
    }

    /// Similar to [`fetch_only()`][Self::fetch_only()`], but passes ownership to a utility type to configure a checkout operation.
    #[cfg(all(feature = "worktree-mutation", feature = "blocking-network-client"))]
    pub fn fetch_then_checkout<P>(
        &mut self,
        progress: P,
        should_interrupt: &std::sync::atomic::AtomicBool,
    ) -> Result<(crate::clone::PrepareCheckout, crate::remote::fetch::Outcome), Error>
    where
        P: crate::NestedProgress,
        P::SubProgress: 'static,
    {
        let (repo, fetch_outcome) = self.fetch_only(progress, should_interrupt)?;
        Ok((crate::clone::PrepareCheckout { repo: repo.into() }, fetch_outcome))
    }
}

mod util;
