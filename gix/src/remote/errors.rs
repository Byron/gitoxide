///
pub mod find {
    use crate::{bstr::BString, config, remote};

    /// The error returned by [`Repository::find_remote(…)`][crate::Repository::find_remote()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("The value for 'remote.<name>.tagOpt` is invalid and must either be '--tags' or '--no-tags'")]
        TagOpt(#[from] config::key::GenericErrorWithValue),
        #[error("{kind} ref-spec under `remote.{remote_name}` was invalid")]
        RefSpec {
            kind: &'static str,
            remote_name: BString,
            source: config::refspec::Error,
        },
        #[error("Neither 'url` nor 'pushUrl' fields were set in the remote's configuration.")]
        UrlMissing,
        #[error("The {kind} url under `remote.{remote_name}` was invalid")]
        Url {
            kind: &'static str,
            remote_name: BString,
            source: config::url::Error,
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
            UrlParse(#[from] gix_url::parse::Error),
            #[error("The remote named {name:?} did not exist")]
            NotFound { name: BString },
        }
    }
}
