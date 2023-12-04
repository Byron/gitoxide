//! The tree of supported configuration values for use in [`config_overrides`][crate::open::Options::config_overrides()]
//! or for validating and transforming well-known configuration values.
//!
//! It can also be used to traverse all implemented keys and to validate values before usage as configuration overrides.
//!
//! ### Leniency
//!
//! When validating values, we don't apply leniency here which is left to the caller. Leniency is an application defined configuration
//! to ignore errors on non-security related values, which might make applications more resilient towards misconfiguration.

/// The root of the configuration tree, suitable to discover all sub-sections at runtime or compile time.
#[derive(Copy, Clone, Default)]
pub struct Tree;

impl Tree {
    /// The `author` section.
    pub const AUTHOR: sections::Author = sections::Author;
    /// The `branch` section.
    pub const BRANCH: sections::Branch = sections::Branch;
    /// The `checkout` section.
    pub const CHECKOUT: sections::Checkout = sections::Checkout;
    /// The `clone` section.
    pub const CLONE: sections::Clone = sections::Clone;
    /// The `committer` section.
    pub const COMMITTER: sections::Committer = sections::Committer;
    /// The `core` section.
    pub const CORE: sections::Core = sections::Core;
    /// The `credential` section.
    pub const CREDENTIAL: sections::Credential = sections::Credential;
    /// The `diff` section.
    #[cfg(feature = "blob-diff")]
    pub const DIFF: sections::Diff = sections::Diff;
    /// The `extensions` section.
    pub const EXTENSIONS: sections::Extensions = sections::Extensions;
    /// The `fetch` section.
    pub const FETCH: sections::Fetch = sections::Fetch;
    /// The `gitoxide` section.
    pub const GITOXIDE: sections::Gitoxide = sections::Gitoxide;
    /// The `http` section.
    pub const HTTP: sections::Http = sections::Http;
    /// The `index` section.
    pub const INDEX: sections::Index = sections::Index;
    /// The `init` section.
    pub const INIT: sections::Init = sections::Init;
    /// The `pack` section.
    pub const PACK: sections::Pack = sections::Pack;
    /// The `protocol` section.
    pub const PROTOCOL: sections::Protocol = sections::Protocol;
    /// The `remote` section.
    pub const REMOTE: sections::Remote = sections::Remote;
    /// The `safe` section.
    pub const SAFE: sections::Safe = sections::Safe;
    /// The `ssh` section.
    pub const SSH: sections::Ssh = sections::Ssh;
    /// The `user` section.
    pub const USER: sections::User = sections::User;
    /// The `url` section.
    pub const URL: sections::Url = sections::Url;

    /// List all available sections.
    pub fn sections(&self) -> &[&dyn Section] {
        &[
            &Self::AUTHOR,
            &Self::BRANCH,
            &Self::CHECKOUT,
            &Self::CLONE,
            &Self::COMMITTER,
            &Self::CORE,
            &Self::CREDENTIAL,
            #[cfg(feature = "blob-diff")]
            &Self::DIFF,
            &Self::EXTENSIONS,
            &Self::FETCH,
            &Self::GITOXIDE,
            &Self::HTTP,
            &Self::INDEX,
            &Self::INIT,
            &Self::PACK,
            &Self::PROTOCOL,
            &Self::REMOTE,
            &Self::SAFE,
            &Self::SSH,
            &Self::USER,
            &Self::URL,
        ]
    }
}

mod remote_name;
pub mod sections;
pub use sections::{
    // TODO: What is the canonical way to access config values?
    // branch, checkout, core, credential, extensions, fetch, gitoxide, http, index, protocol, remote, ssh, 
    Author, Branch, Checkout, Clone, Committer, Core, Credential, Extensions, Fetch, Gitoxide, Http, Index, Init, Pack,
    Protocol, Remote, Safe, Ssh, Url, User,
};
#[cfg(feature = "blob-diff")]
pub use sections::Diff;

/// Generic value implementations for static instantiation.
pub mod keys;

mod traits;
pub use traits::{Key, Link, Note, Section, SubSectionRequirement};

///
pub mod key {
    use bstr::BString;

    ///
    pub mod validate {
        /// The error returned by [`Key::validate()`][crate::config::tree::Key::validate()].
        #[derive(Debug, thiserror::Error)]
        #[error(transparent)]
        #[allow(missing_docs)]
        pub struct Error {
            #[from]
            source: Box<dyn std::error::Error + Send + Sync + 'static>,
        }
    }
    ///
    pub mod validate_assignment {
        /// The error returned by [`Key::validated_assignment`*()][crate::config::tree::Key::validated_assignment_fmt()].
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error("Failed to validate the value to be assigned to this key")]
            Validate(#[from] super::validate::Error),
            #[error("{message}")]
            Name { message: String },
        }
    }

    const fn prefix(kind: char) -> &'static str {
        match kind {
            'n' => "",                         // nothing
            'k' => "The value of key",         // generic key
            't' => "The date format at key",   // time
            'i' => "The timeout at key",       // timeout
            'd' => "The duration [ms] at key", // duration
            'b' => "The boolean at key",       // boolean
            'v' => "The key",                  // generic key with value
            'r' => "The refspec at",           // refspec
            's' => "The ssl version at",       // ssl-version
            'u' => "The url at",               // url
            'w' => "The utf-8 string at",      // string
            _ => panic!("BUG: invalid prefix kind - add a case for it here"),
        }
    }

    const fn suffix(kind: char) -> &'static str {
        match kind {
            'd' => "could not be decoded",                    // decoding
            'i' => "was invalid",                             // invalid
            'u' => "could not be parsed as unsigned integer", // unsigned integer
            'p' => "could not be parsed",                     // parsing
            _ => panic!("BUG: invalid suffix kind - add a case for it here"),
        }
    }

    /// A generic error suitable to produce decent messages for all kinds of configuration errors with config-key granularity.
    ///
    /// This error is meant to be reusable and help produce uniform error messages related to parsing any configuration key.
    #[derive(Debug, thiserror::Error)]
    #[error("{} \"{key}{}\"{} {}", prefix(PREFIX), value.as_ref().map(|v| format!("={v}")).unwrap_or_default(), environment_override.as_deref().map(|var| format!(" (possibly from {var})")).unwrap_or_default(), suffix(SUFFIX))]
    pub struct Error<E: std::error::Error + Send + Sync + 'static, const PREFIX: char, const SUFFIX: char> {
        /// The configuration key that contained the value.
        pub key: BString,
        /// The value that was assigned to `key`.
        pub value: Option<BString>,
        /// The associated environment variable that would override this value.
        pub environment_override: Option<&'static str>,
        /// The source of the error if there was one.
        pub source: Option<E>,
    }

    /// Initialization
    /// Instantiate a new error from the given `key`.
    ///
    /// Note that specifics of the error message are defined by the `PREFIX` and `SUFFIX` which is usually defined by a typedef.
    impl<T, E, const PREFIX: char, const SUFFIX: char> From<&'static T> for Error<E, PREFIX, SUFFIX>
    where
        E: std::error::Error + Send + Sync + 'static,
        T: super::Key,
    {
        fn from(key: &'static T) -> Self {
            Error {
                key: key.logical_name().into(),
                value: None,
                environment_override: key.environment_override(),
                source: None,
            }
        }
    }

    /// Initialization
    impl<E, const PREFIX: char, const SUFFIX: char> Error<E, PREFIX, SUFFIX>
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        /// Instantiate an error with all data from `key` along with the `value` of the key.
        pub fn from_value(key: &'static impl super::Key, value: BString) -> Self {
            Error::from(key).with_value(value)
        }
    }

    /// Builder
    impl<E, const PREFIX: char, const SUFFIX: char> Error<E, PREFIX, SUFFIX>
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        /// Attach the given `err` as source.
        pub fn with_source(mut self, err: E) -> Self {
            self.source = Some(err);
            self
        }

        /// Attach the given `value` as value we observed when the error was produced.
        pub fn with_value(mut self, value: BString) -> Self {
            self.value = Some(value);
            self
        }
    }

    /// A generic key error for use when it doesn't seem worth it say more than 'key is invalid' along with meta-data.
    pub type GenericError<E = gix_config_value::Error> = Error<E, 'k', 'i'>;

    /// A generic key error which will also contain a value.
    pub type GenericErrorWithValue<E = gix_config_value::Error> = Error<E, 'v', 'i'>;
}

// TODO Copied from gix/src/config/mod.rs
///
pub mod diff {
    ///
    pub mod algorithm {
        use bstr::BString;

        /// The error produced when obtaining `diff.algorithm`.
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error("Unknown diff algorithm named '{name}'")]
            Unknown { name: BString },
            #[error("The '{name}' algorithm is not yet implemented")]
            Unimplemented { name: BString },
        }
    }

    pub mod renames {
        // TODO Copied from gix/src/diff.rs
        /// Determine how to do rename tracking.
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub enum Tracking {
            /// Do not track renames at all, the fastest option.
            Disabled,
            /// Track renames.
            Renames,
            /// Track renames and copies.
            ///
            /// This is the most expensive option.
            RenamesAndCopies,
        }
    }
}

pub mod disambiguate {
    // TODO Copied from gix/src/revision/spec/parse/types.rs
    /// A hint to know which object kind to prefer if multiple objects match a prefix.
    ///
    /// This disambiguation mechanism is applied only if there is no disambiguation hints in the spec itself.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ObjectKindHint {
        /// Pick objects that are commits themselves.
        Commit,
        /// Pick objects that can be peeled into a commit, i.e. commits themselves or tags which are peeled until a commit is found.
        Committish,
        /// Pick objects that are trees themselves.
        Tree,
        /// Pick objects that can be peeled into a tree, i.e. trees themselves or tags which are peeled until a tree is found or commits
        /// whose tree is chosen.
        Treeish,
        /// Pick objects that are blobs.
        Blob,
    }
}

///
pub mod encoding {
    use bstr::BString;

    /// The error produced when failing to parse the `core.checkRoundTripEncoding` key.
    #[derive(Debug, thiserror::Error)]
    #[error("The encoding named '{encoding}' seen in key '{key}={value}' is unsupported")]
    pub struct Error {
        /// The configuration key that contained the value.
        pub key: BString,
        /// The value that was assigned to `key`.
        pub value: BString,
        /// The encoding that failed.
        pub encoding: BString,
    }
}

///
pub mod checkout {
    ///
    pub mod workers {
        /// The error produced when failing to parse the `checkout.workers` key.
        pub type Error = super::super::key::Error<gix_config_value::Error, 'n', 'd'>;
    }
}

///
pub mod abbrev {
    use bstr::BString;

    /// The error describing an incorrect `core.abbrev` value.
    #[derive(Debug, thiserror::Error)]
    #[error("Invalid value for 'core.abbrev' = '{}'. It must be between 4 and {}", .value, .max)]
    pub struct Error {
        /// The value found in the git configuration
        pub value: BString,
        /// The maximum abbreviation length, the length of an object hash.
        pub max: u8,
    }
}

///
pub mod remote {
    ///
    pub mod symbolic_name {
        /// The error produced when failing to produce a symbolic remote name from configuration.
        pub type Error = super::super::key::Error<crate::remote_name::Error, 'v', 'i'>;
    }
}

///
pub mod time {
    /// The error produced when failing to parse time from configuration.
    pub type Error = super::key::Error<gix_date::parse::Error, 't', 'i'>;
}

///
pub mod lock_timeout {
    /// The error produced when failing to parse timeout for locks.
    pub type Error = super::key::Error<gix_config_value::Error, 'i', 'i'>;
}

///
pub mod duration {
    /// The error produced when failing to parse durations (in milliseconds).
    pub type Error = super::key::Error<gix_config_value::Error, 'd', 'i'>;
}

///
pub mod boolean {
    /// The error produced when failing to parse time from configuration.
    pub type Error = super::key::Error<gix_config_value::Error, 'b', 'i'>;
}

///
pub mod unsigned_integer {
    /// The error produced when failing to parse a signed integer from configuration.
    pub type Error = super::key::Error<gix_config_value::Error, 'k', 'u'>;
}

///
pub mod url {
    /// The error produced when failing to parse a url from the configuration.
    pub type Error = super::key::Error<gix_url::parse::Error, 'u', 'p'>;
}

///
pub mod string {
    /// The error produced when failing to interpret configuration as UTF-8 encoded string.
    pub type Error = super::key::Error<bstr::Utf8Error, 'w', 'd'>;
}

///
pub mod refspec {
    /// The error produced when failing to parse a refspec from the configuration.
    pub type Error = super::key::Error<gix_refspec::parse::Error, 'r', 'p'>;
}

///
pub mod refs_namespace {
    /// The error produced when failing to parse a refspec from the configuration.
    pub type Error = super::key::Error<gix_validate::reference::name::Error, 'v', 'i'>;
}

///
pub mod ssl_version {
    /// The error produced when failing to parse a refspec from the configuration.
    pub type Error = super::key::Error<std::convert::Infallible, 's', 'i'>;
}
