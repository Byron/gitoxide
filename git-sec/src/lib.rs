//! A shared trust model for `gitoxide` crates.
//!
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
// `unsafe_code` not forbidden because we need to interact with the libc
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

use std::fmt::{Display, Formatter};

/// A way to specify how 'safe' we feel about a resource, typically about a git repository.
#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Trust {
    /// Caution is warranted when using the resource.
    Reduced,
    /// We have no doubts that this resource means no harm and it can be used at will.
    Full,
}

///
pub mod trust {
    use crate::Trust;

    impl Trust {
        /// Derive `Full` trust if `path` is owned by the user executing the current process, or `Reduced` trust otherwise.
        pub fn from_path_ownership(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
            Ok(crate::identity::is_path_owned_by_current_user(path.as_ref())?
                .then(|| Trust::Full)
                .unwrap_or(Trust::Reduced))
        }
    }

    /// A trait to help creating default values based on a trust level.
    pub trait DefaultForLevel {
        /// Produce a default value for the given trust `level`.
        fn default_for_level(level: Trust) -> Self;
    }

    /// Associate instructions for how to deal with various `Trust` levels as they are encountered in the wild.
    pub struct Mapping<T> {
        /// The value for fully trusted resources.
        pub full: T,
        /// The value for resources with reduced trust.
        pub reduced: T,
    }

    impl<T> Default for Mapping<T>
    where
        T: DefaultForLevel,
    {
        fn default() -> Self {
            Mapping {
                full: T::default_for_level(Trust::Full),
                reduced: T::default_for_level(Trust::Reduced),
            }
        }
    }

    impl<T> Mapping<T> {
        /// Obtain the value for the given trust `level`.
        pub fn by_level(&self, level: Trust) -> &T {
            match level {
                Trust::Full => &self.full,
                Trust::Reduced => &self.reduced,
            }
        }

        /// Obtain the value for the given `level` once.
        pub fn into_value_by_level(self, level: Trust) -> T {
            match level {
                Trust::Full => self.full,
                Trust::Reduced => self.reduced,
            }
        }
    }
}

///
pub mod permission {
    use crate::Permission;
    use std::fmt::{Display, Formatter};

    /// An error to use if an operation cannot proceed due to insufficient permissions.
    ///
    /// It's up to the implementation to decide which permission is required for an operation, and which one
    /// causes errors.
    #[cfg(feature = "thiserror")]
    #[derive(Debug, thiserror::Error)]
    #[error("Not allowed to handle resource {:?}: permission {}", .resource, .permission)]
    pub struct Error<R: std::fmt::Debug, P: std::fmt::Debug + Display> {
        /// The resource which cannot be used.
        pub resource: R,
        /// The permission causing it to be disallowed.
        pub permission: P,
    }

    impl Permission {
        /// Check this permissions and produce a reply to indicate if the `resource` can be used and in which way.
        ///
        /// Only if this permission is set to `Allow` will the resource be usable.
        #[cfg(feature = "thiserror")]
        pub fn check<R: std::fmt::Debug>(&self, resource: R) -> Result<Option<R>, Error<R, Self>> {
            match self {
                Permission::Allow => Ok(Some(resource)),
                Permission::Deny => Ok(None),
                Permission::Forbid => Err(Error {
                    resource,
                    permission: *self,
                }),
            }
        }
    }

    impl Display for Permission {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            Display::fmt(
                match self {
                    Permission::Allow => "allowed",
                    Permission::Deny => "denied",
                    Permission::Forbid => "forbidden",
                },
                f,
            )
        }
    }
}

/// Allow, deny or forbid using a resource or performing an action.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub enum Permission {
    /// Fail outright when trying to load a resource or performing an action.
    Forbid,
    /// Ignore resources or try to avoid performing an operation.
    Deny,
    /// Allow loading a resource or performing an action.
    Allow,
}

bitflags::bitflags! {
    /// Whether something can be read or written.
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct ReadWrite: u8 {
        /// The item can be read.
        const READ = 1 << 0;
        /// The item can be written
        const WRITE = 1 << 1;
    }
}

impl Display for ReadWrite {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

/// Various types to identify entities.
pub mod identity;
