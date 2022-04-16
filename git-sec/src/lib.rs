#![deny(unsafe_code, rust_2018_idioms, missing_docs)]
//! A shared trust model for `gitoxide` crates.

use std::path::Path;

/// A way to specify how 'safe' we feel about a resource, typically about a git repository.
pub enum Trust {
    /// We have no doubts that this resource means no harm and it can be used at will.
    Full,
    /// Caution is warranted when using the resource.
    Reduced,
}

impl Trust {
    /// Derive `Full` trust if `path` is owned by the user executing the current process, or `Reduced` trust otherwise.
    pub fn from_path_ownership(path: impl AsRef<Path>) -> std::io::Result<Self> {
        Ok(identity::is_path_owned_by_current_user(path.as_ref())?
            .then(|| Trust::Full)
            .unwrap_or(Trust::Reduced))
    }
}

///
pub mod trust {
    use crate::Trust;

    /// Associate instructions for how to deal with various `Trust` levels as they are encountered in the wild.
    pub struct Mapping<T> {
        /// The value for fully trusted resources.
        pub full: T,
        /// The value for resources with reduced trust.
        pub reduced: T,
    }

    impl<T> Mapping<T> {
        /// Obtain the value for the given trust `level`.
        pub fn by_trust(&self, level: &Trust) -> &T {
            match level {
                Trust::Full => &self.full,
                Trust::Reduced => &self.reduced,
            }
        }
    }
}

///
pub mod resource {
    /// Permissions related to resources at _locations_, like configuration files, executables or destinations for operations.
    ///
    /// Note that typically the permission refers to the place where the _location_ is configured, not to the _location_ itself.
    /// For example, we may trust an owned configuration file, and by relation all the _locations_ inside of it even though
    /// these are not owned by us.
    pub enum Permission {
        /// The greatest permission level without any restrictions, all _locations_ are permitted.
        ///
        /// For _locations_ to executables, it can be found in the `PATH` or configured from any git config file.
        ///
        /// Note that, however, some executables still won't be picked up from repository-local configuration
        /// for safety reasons.
        Allow,
        /// For _locations_ to executables, only run these if these have been configured by git config files
        /// that are owned by the user executing the application, or if these are in the `PATH`.
        /// Resources or write destinations adhere to the same rules.
        AllowIfOwned {
            /// If true, if a _location_ is not under user control, instead of failing, fallback to a configuration setting that
            /// is or try to not fail by using suitable defaults. For executables this may mean to search for them in the `PATH`
            /// or fall back to another configuration value from configuration files under user control.
            allow_fallback: bool,
        },
        /// Do not use any _location_ unless it's required for git to function by using defaults.
        ///
        /// If such a resource is encountered, the operation may fail.
        Deny {
            /// If true, operations that would fail may proceed by ignoring the resource if possible or using
            /// defaults that are deemed safe.
            ///
            /// For executables this means using those in the `PATH` only.
            allow_fallback: bool,
        },
    }
}

/// Various types to identify entities.
pub mod identity;
