///
pub mod find {
    use crate::{bstr::BString, remote};

    /// The error returned by [`Repository::find_remote(…)`][crate::Repository::find_remote()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(
            "The value for 'remote.<name>.tagOpt` is invalid and must either be '--tags' or '--no-tags': \"{value}\""
        )]
        TagOpt { value: BString },
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
        use crate::bstr::BString;

        /// The error returned by [`Repository::find_remote(…)`][crate::Repository::find_remote()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error(transparent)]
            Find(#[from] super::Error),
            #[error("remote name could not be parsed as URL")]
            UrlParse(#[from] git_url::parse::Error),
            #[error("The remote named {name:?} did not exist")]
            NotFound { name: BString },
        }
    }
}
