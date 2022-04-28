#![forbid(unsafe_code)]
// #![warn(missing_docs)]
// #![warn(clippy::pedantic, clippy::nursery)]

//! # `git_config`
//!
//! This crate is a high performance `git-config` file reader and writer. It
//! exposes a high level API to parse, read, and write [`git-config` files],
//! which are loosely based on the [INI file format].
//!
//! This crate has a few primary offerings and various accessory functions. The
//! table below gives a brief explanation of all offerings, loosely in order
//! from the highest to lowest abstraction.
//!
//! | Offering      | Description                                         | Zero-copy?        |
//! | ------------- | --------------------------------------------------- | ----------------- |
//! | [`GitConfig`] | Accelerated wrapper for reading and writing values. | On some reads[^1] |
//! | [`Parser`]    | Syntactic event emitter for `git-config` files.     | Yes               |
//! | [`values`]    | Wrappers for `git-config` value types.              | Yes               |
//!
//! This crate also exposes efficient value normalization which unescapes
//! characters and removes quotes through the `normalize_*` family of functions,
//! located in the [`values`] module.
//!
//! # Zero-copy versus zero-alloc
//!
//! We follow [`nom`]'s definition of "zero-copy":
//!
//! > If a parser returns a subset of its input data, it will return a slice of
//! > that input, without copying.
//!
//! Due to the syntax of `git-config`, we must allocate at the parsing level
//! (and thus higher level abstractions must allocate as well) in order to
//! provide a meaningful event stream. That being said, all operations with the
//! parser is still zero-copy. Higher level abstractions may have operations
//! that are zero-copy, but are not guaranteed to do so.
//!
//! However, we intend to be performant as possible, so allocations are
//! limited restricted and we attempt to avoid copying whenever possible.
//!
//! [^1]: When read values do not need normalization.
//!
//! [`git-config` files]: https://git-scm.com/docs/git-config#_configuration_file
//! [INI file format]: https://en.wikipedia.org/wiki/INI_file
//! [`GitConfig`]: crate::file::GitConfig
//! [`Parser`]: crate::parser::Parser
//! [`values`]: crate::values
//! [`nom`]: https://github.com/Geal/nom

// Cargo.toml cannot have self-referential dependencies, so you can't just
// specify the actual serde crate when you define a feature called serde. We
// instead call the serde crate as serde_crate and then rename the crate to
// serde, to get around this in an intuitive manner.
#[cfg(feature = "serde")]
extern crate serde_crate as serde;

pub mod lookup {

    /// The error when looking up a value.
    #[derive(Debug, thiserror::Error)]
    pub enum Error<E> {
        #[error(transparent)]
        ValueMissing(#[from] crate::lookup::existing::Error),
        #[error(transparent)]
        FailedConversion(E),
    }

    pub mod existing {
        /// The error when looking up a value that doesn't exist.
        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error("The requested section does not exist")]
            SectionMissing,
            #[error("The requested subsection does not exist")]
            SubSectionMissing,
            #[error("The key does not exist in the requested section")]
            KeyMissing,
        }
    }
}

pub mod file;
pub mod fs;
pub mod parser;
pub mod values;
/// The future home of the `values` module (TODO).
pub mod value {
    pub mod parse {
        use bstr::BString;

        /// The error returned when creating `Integer` from byte string.
        #[derive(Debug, thiserror::Error, Eq, PartialEq)]
        #[allow(missing_docs)]
        #[error("Could not decode '{}': {}", .input, .message)]
        pub struct Error {
            pub message: &'static str,
            pub input: BString,
            #[source]
            pub utf8_err: Option<std::str::Utf8Error>,
        }

        impl Error {
            pub(crate) fn new(message: &'static str, input: impl Into<BString>) -> Self {
                Error {
                    message,
                    input: input.into(),
                    utf8_err: None,
                }
            }

            pub(crate) fn with_err(mut self, err: std::str::Utf8Error) -> Self {
                self.utf8_err = Some(err);
                self
            }
        }
    }
}

mod permissions {
    use crate::Permissions;

    impl Permissions {
        /// Allow everything which usually relates to a fully trusted environment
        pub fn all() -> Self {
            use git_sec::Permission::*;
            Permissions {
                system: Allow,
                global: Allow,
                user: Allow,
                repository: Allow,
                worktree: Allow,
                env: Allow,
                includes: Allow,
            }
        }

        /// If in doubt, this configuration can be used to safely load configuration from sources which is usually trusted,
        /// that is system and user configuration. Do load any configuration that isn't trusted as it's now owned by the current user.
        pub fn secure() -> Self {
            use git_sec::Permission::*;
            Permissions {
                system: Allow,
                global: Allow,
                user: Allow,
                repository: Deny,
                worktree: Deny,
                env: Allow,
                includes: Deny,
            }
        }
    }
}

/// Configure security relevant options when loading a git configuration.
#[derive(Copy, Clone, Ord, PartialOrd, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Permissions {
    /// How to use the system configuration.
    /// This is defined as `$(prefix)/etc/gitconfig` on unix.
    pub system: git_sec::Permission,
    /// How to use the global configuration.
    /// This is usually `~/.gitconfig`.
    pub global: git_sec::Permission,
    /// How to use the user configuration.
    /// Second user-specific configuration path; if `$XDG_CONFIG_HOME` is not
    /// set or empty, `$HOME/.config/git/config` will be used.
    pub user: git_sec::Permission,
    /// How to use the repository configuration.
    pub repository: git_sec::Permission,
    /// How to use worktree configuration from `config.worktree`.
    // TODO: figure out how this really applies and provide more information here.
    pub worktree: git_sec::Permission,
    /// How to use the configuration from environment variables.
    pub env: git_sec::Permission,
    /// What to do when include files are encountered in loaded configuration.
    pub includes: git_sec::Permission,
}

#[cfg(test)]
pub mod test_util;
