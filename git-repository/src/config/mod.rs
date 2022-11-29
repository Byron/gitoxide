pub use git_config::*;
use git_features::threading::OnceCell;

use crate::{bstr::BString, remote, repository::identity, revision::spec, Repository};

pub(crate) mod cache;
mod snapshot;
pub use snapshot::credential_helpers;

///
pub mod overrides;

/// A platform to access configuration values as read from disk.
///
/// Note that these values won't update even if the underlying file(s) change.
pub struct Snapshot<'repo> {
    pub(crate) repo: &'repo Repository,
}

/// A platform to access configuration values and modify them in memory, while making them available when this platform is dropped
/// as form of auto-commit.
/// Note that the values will only affect this instance of the parent repository, and not other clones that may exist.
///
/// Note that these values won't update even if the underlying file(s) change.
///
/// Use [`forget()`][Self::forget()] to not apply any of the changes.
// TODO: make it possible to load snapshots with reloading via .config() and write mutated snapshots back to disk which should be the way
//       to affect all instances of a repo, probably via `config_mut()` and `config_mut_at()`.
pub struct SnapshotMut<'repo> {
    pub(crate) repo: Option<&'repo mut Repository>,
    pub(crate) config: git_config::File<'static>,
}

/// A utility structure created by [`SnapshotMut::commit_auto_rollback()`] that restores the previous configuration on drop.
pub struct CommitAutoRollback<'repo> {
    pub(crate) repo: Option<&'repo mut Repository>,
    pub(crate) prev_config: crate::Config,
}

pub(crate) mod section {
    pub fn is_trusted(meta: &git_config::file::Metadata) -> bool {
        meta.trust == git_sec::Trust::Full || meta.source.kind() != git_config::source::Kind::Repository
    }
}

/// The error returned when failing to initialize the repository configuration.
///
/// This configuration is on the critical path when opening a repository.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Could not read configuration file")]
    Io(#[from] std::io::Error),
    #[error("Could not decode configuration value at {key:?}")]
    Value {
        source: git_config::value::Error,
        key: &'static str,
    },
    #[error(transparent)]
    Init(#[from] git_config::file::init::Error),
    #[error(transparent)]
    ResolveIncludes(#[from] git_config::file::includes::Error),
    #[error(transparent)]
    FromEnv(#[from] git_config::file::init::from_env::Error),
    #[error("Cannot handle objects formatted as {:?}", .name)]
    UnsupportedObjectFormat { name: BString },
    #[error("The value for '{}' cannot be empty", .key)]
    EmptyValue { key: &'static str },
    #[error("Invalid value for 'core.abbrev' = '{}'. It must be between 4 and {}", .value, .max)]
    CoreAbbrev { value: BString, max: u8 },
    #[error("Value '{}' at key '{}' could not be decoded as boolean", .value, .key)]
    DecodeBoolean { key: String, value: BString },
    #[error(transparent)]
    PathInterpolation(#[from] git_config::path::interpolate::Error),
    #[error("{source:?} configuration overrides at open or init time could not be applied.")]
    ConfigOverrides {
        #[source]
        err: overrides::Error,
        source: git_config::Source,
    },
    #[error("Invalid value for 'core.logAllRefUpdates': \"{value}\"")]
    LogAllRefUpdates { value: BString },
}

///
pub mod diff {
    ///
    pub mod algorithm {
        use crate::bstr::BString;

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
}

///
pub mod checkout_options {
    /// The error produced when collecting all information needed for checking out files into a worktree.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("{key} could not be decoded")]
        Configuration {
            key: &'static str,
            source: git_config::value::Error,
        },
        #[error("Failed to interpolate the attribute file configured at `core.attributesFile`")]
        AttributesFileInterpolation(#[from] git_config::path::interpolate::Error),
    }
}

///
pub mod transport {
    use std::borrow::Cow;

    use crate::{bstr, bstr::BStr};

    /// The error produced when configuring a transport for a particular protocol.
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(
            "Could not interpret configuration key {key:?} as {kind} integer of desired range with value: {actual}"
        )]
        InvalidInteger {
            key: &'static str,
            kind: &'static str,
            actual: i64,
        },
        #[error("Could not interpret configuration key {key:?}")]
        ConfigValue {
            source: git_config::value::Error,
            key: &'static str,
        },
        #[error("Could not decode value at key {key:?} as UTF-8 string")]
        IllformedUtf8 {
            key: Cow<'static, BStr>,
            source: bstr::FromUtf8Error,
        },
        #[error("Invalid URL passed for configuration")]
        ParseUrl(#[from] git_url::parse::Error),
        #[error("Could obtain configuration for an HTTP url")]
        Http(#[from] http::Error),
    }

    ///
    pub mod http {
        use std::borrow::Cow;

        use crate::bstr::BStr;

        /// The error produced when configuring a HTTP transport.
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error("The proxy authentication method name {value:?} found at key `{key}` is invalid")]
            InvalidProxyAuthMethod { value: String, key: Cow<'static, BStr> },
            #[error("Could not configure the credential helpers for the authenticated proxy url")]
            ConfigureProxyAuthenticate(#[from] crate::config::snapshot::credential_helpers::Error),
        }
    }
}

/// Utility type to keep pre-obtained configuration values, only for those required during initial setup
/// and other basic operations that are common enough to warrant a permanent cache.
///
/// All other values are obtained lazily using OnceCell.
#[derive(Clone)]
pub(crate) struct Cache {
    pub resolved: crate::Config,
    /// The hex-length to assume when shortening object ids. If `None`, it should be computed based on the approximate object count.
    pub hex_len: Option<usize>,
    /// true if the repository is designated as 'bare', without work tree.
    pub is_bare: bool,
    /// The type of hash to use.
    pub object_hash: git_hash::Kind,
    /// If true, multi-pack indices, whether present or not, may be used by the object database.
    pub use_multi_pack_index: bool,
    /// The representation of `core.logallrefupdates`, or `None` if the variable wasn't set.
    pub reflog: Option<git_ref::store::WriteReflog>,
    /// The configured user agent for presentation to servers.
    pub(crate) user_agent: OnceCell<String>,
    /// identities for later use, lazy initialization.
    pub(crate) personas: OnceCell<identity::Personas>,
    /// A lazily loaded rewrite list for remote urls
    pub(crate) url_rewrite: OnceCell<remote::url::Rewrite>,
    /// A lazily loaded mapping to know which url schemes to allow
    #[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
    pub(crate) url_scheme: OnceCell<remote::url::SchemePermission>,
    /// The algorithm to use when diffing blobs
    pub(crate) diff_algorithm: OnceCell<git_diff::blob::Algorithm>,
    /// The amount of bytes to use for a memory backed delta pack cache. If `Some(0)`, no cache is used, if `None`
    /// a standard cache is used which costs near to nothing and always pays for itself.
    pub(crate) pack_cache_bytes: Option<usize>,
    /// The amount of bytes to use for caching whole objects, or 0 to turn it off entirely.
    pub(crate) object_cache_bytes: usize,
    /// The config section filter from the options used to initialize this instance. Keep these in sync!
    filter_config_section: fn(&git_config::file::Metadata) -> bool,
    /// The object kind to pick if a prefix is ambiguous.
    pub object_kind_hint: Option<spec::parse::ObjectKindHint>,
    /// If true, we are on a case-insensitive file system.
    pub ignore_case: bool,
    /// If true, we should default what's possible if something is misconfigured, on case by case basis, to be more resilient.
    /// Also available in options! Keep in sync!
    pub lenient_config: bool,
    /// Define how we can use values obtained with `xdg_config(…)` and its `XDG_CONFIG_HOME` variable.
    xdg_config_home_env: git_sec::Permission,
    /// Define how we can use values obtained with `xdg_config(…)`. and its `HOME` variable.
    home_env: git_sec::Permission,
    // TODO: make core.precomposeUnicode available as well.
}
