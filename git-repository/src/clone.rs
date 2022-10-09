use crate::Repository;
use std::convert::TryInto;

/// A utility to collect configuration on how to fetch from a remote and possibly create a working tree locally.
pub struct Prepare {
    /// A freshly initialized repository which is owned by us, or `None` if it was handed to the user
    repo: Option<Repository>,
    /// The url to clone from
    #[allow(dead_code)]
    url: git_url::Url,
}

impl Drop for Prepare {
    fn drop(&mut self) {
        if let Some(repo) = self.repo.take() {
            std::fs::remove_dir_all(repo.work_dir().unwrap_or_else(|| repo.path())).ok();
        }
    }
}

impl Into<Repository> for Prepare {
    fn into(self) -> Repository {
        self.persist()
    }
}

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
        Ok(Prepare { url, repo: Some(repo) })
    }
}

/// Access
impl Prepare {
    /// Persist the contained repository as is even if an error may have occurred when interacting with the remote or checking out the main working tree.
    pub fn persist(mut self) -> Repository {
        self.repo.take().expect("present and consumed once")
    }
}
