///
pub mod init {
    use crate::bstr::BString;

    /// The error returned by [`Repository::remote_at(…)`][crate::Repository::remote_at()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Url(#[from] git_url::parse::Error),
        #[error("The rewritten {kind} url {rewritten_url:?} failed to parse")]
        RewrittenUrlInvalid {
            kind: &'static str,
            rewritten_url: BString,
            source: git_url::parse::Error,
        },
    }
}

///
pub mod find {
    use crate::bstr::BString;
    use crate::remote;

    /// The error returned by [`Repository::find_remote(…)`][crate::Repository::find_remote()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("{spec:?} {kind} ref-spec failed to parse")]
        RefSpec {
            spec: BString,
            kind: &'static str,
            source: git_refspec::parse::Error,
        },
        #[error("Neither 'url` nor 'pushUrl' fields were set in the remote's configuration.")]
        UrlMissing,
        #[error("The {kind} url couldn't be parsed")]
        Url {
            kind: &'static str,
            url: BString,
            source: git_url::parse::Error,
        },
        #[error(transparent)]
        Init(#[from] remote::init::Error),
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
