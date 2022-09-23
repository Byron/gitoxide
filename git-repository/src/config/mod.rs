pub use git_config::*;
use git_features::threading::OnceCell;

use crate::{bstr::BString, remote, repository::identity, revision::spec, Repository};

pub(crate) mod cache;
mod snapshot;
pub use snapshot::{apply_cli_overrides, credential_helpers};

/// A platform to access configuration values as read from disk.
///
/// Note that these values won't update even if the underlying file(s) change.
pub struct Snapshot<'repo> {
    pub(crate) repo: &'repo Repository,
}

/// A platform to access configuration values and modify them in memory, while making them available when this platform is dropped.
/// Note that the values will only affect this instance of the parent repository, and not other clones that may exist.
///
/// Note that these values won't update even if the underlying file(s) change.
///
/// Use [`forget()`][Self::forget()] to not apply any of the changes.
// TODO: make it possible to load snapshots with reloading via .config() and write mutated snapshots back to disk which should be the way
//       to affect all instances of a repo.
pub struct SnapshotMut<'repo> {
    pub(crate) repo: &'repo mut Repository,
    pub(crate) config: git_config::File<'static>,
}

pub(crate) mod section {
    pub fn is_trusted(meta: &git_config::file::Metadata) -> bool {
        meta.trust == git_sec::Trust::Full || meta.source.kind() != git_config::source::Kind::Repository
    }
}

/// The error returned when failing to initialize the repository configuration.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Could not read configuration file")]
    Io(#[from] std::io::Error),
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
}

/// Utility type to keep pre-obtained configuration values.
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
    /// identities for later use, lazy initialization.
    pub personas: OnceCell<identity::Personas>,
    /// A lazily loaded rewrite list for remote urls
    pub url_rewrite: OnceCell<remote::url::Rewrite>,
    /// A lazily loaded mapping to know which url schemes to allow
    #[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
    pub url_scheme: OnceCell<remote::url::SchemePermission>,
    /// The config section filter from the options used to initialize this instance. Keep these in sync!
    filter_config_section: fn(&git_config::file::Metadata) -> bool,
    /// The object kind to pick if a prefix is ambiguous.
    pub object_kind_hint: Option<spec::parse::ObjectKindHint>,
    /// If true, we are on a case-insensitive file system.
    pub ignore_case: bool,
    /// The path to the user-level excludes file to ignore certain files in the worktree.
    pub excludes_file: Option<std::path::PathBuf>,
    /// Define how we can use values obtained with `xdg_config(…)` and its `XDG_CONFIG_HOME` variable.
    xdg_config_home_env: git_sec::Permission,
    /// Define how we can use values obtained with `xdg_config(…)`. and its `HOME` variable.
    home_env: git_sec::Permission,
    /// How to use git-prefixed environment variables
    git_prefix: git_sec::Permission,
    // TODO: make core.precomposeUnicode available as well.
}
