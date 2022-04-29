#![deny(unsafe_code, rust_2018_idioms, missing_docs)]
//! A shared trust model for `gitoxide` crates.

use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;

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
    use crate::Access;
    use std::fmt::{Debug, Display};

    /// A marker trait to signal tags for permissions.
    pub trait Tag: Debug {}

    /// A tag indicating that a permission is applying to the contents of a configuration file.
    #[derive(Debug)]
    pub struct Config;
    impl Tag for Config {}

    /// A tag indicating that a permission is applying to the resource itself.
    #[derive(Debug)]
    pub struct Resource;
    impl Tag for Resource {}

    impl<P: Debug + Display> Access<Config, P> {
        /// Create a permission for values contained in git configuration files.
        ///
        /// This applies permissions to values contained inside of these files.
        pub fn config(permission: P) -> Self {
            Access {
                permission,
                _data: Default::default(),
            }
        }
    }

    impl<P: Debug + Display> Access<Resource, P> {
        /// Create a permission a file or directory itself.
        ///
        /// This applies permissions to a configuration file itself and whether it can be used at all, or to a directory
        /// to read from or write to.
        pub fn resource(permission: P) -> Self {
            Access {
                permission,
                _data: Default::default(),
            }
        }
    }

    /// An error to use if an operation cannot proceed due to insufficient permissions.
    ///
    /// It's up to the implementation to decide which permission is required for an operation, and which one
    /// causes errors.
    #[cfg(feature = "thiserror")]
    #[derive(Debug, thiserror::Error)]
    #[error("Not allowed to handle resource {:?}: permission {}", .resource, .permission)]
    pub struct Error<R: Debug, P: Debug + Display> {
        resource: R,
        permission: P,
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
    /// Allow loading a reasource or performing an action.
    Allow,
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
        Debug::fmt(self, f)
    }
}

/// A container to define tagged access permissions, rendering the permission read-only.
#[derive(Debug)]
pub struct Access<T: permission::Tag, P: Debug + Display> {
    /// The access permission itself.
    permission: P,
    _data: PhantomData<T>,
}

impl<T: permission::Tag, P: Debug + Display> Display for Access<T, P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.permission, f)
    }
}

impl<T: permission::Tag, P: Debug + Display> Deref for Access<T, P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.permission
    }
}

/// Various types to identify entities.
pub mod identity;
