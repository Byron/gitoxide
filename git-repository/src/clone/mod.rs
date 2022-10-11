use std::convert::TryInto;

type ConfigureRemoteFn = Box<dyn FnMut(crate::Remote<'_>) -> Result<crate::Remote<'_>, crate::remote::init::Error>>;

/// A utility to collect configuration on how to fetch from a remote and possibly create a working tree locally.
pub struct PrepareFetch {
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
impl PrepareFetch {
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
        Ok(PrepareFetch {
            url,
            #[cfg(any(feature = "async-network-client", feature = "blocking-network-client"))]
            fetch_options: Default::default(),
            repo: Some(repo),
            remote_name: None,
            configure_remote: None,
        })
    }
}

///
pub mod fetch;
