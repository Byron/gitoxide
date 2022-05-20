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

pub mod file;
pub mod fs;
pub mod lookup;
pub mod parser;
mod permissions;
/// The future home of the `values` module (TODO).
pub mod value;
pub mod values;
mod types {
    use crate::file::{LookupTreeNode, SectionBody, SectionId};
    use crate::parser::{ParsedSectionHeader, SectionHeaderName};
    use std::collections::{HashMap, VecDeque};

    /// High level `git-config` reader and writer.
    ///
    /// This is the full-featured implementation that can deserialize, serialize,
    /// and edit `git-config` files without loss of whitespace or comments. As a
    /// result, it's lot more complex than it's read-only variant,
    /// [`ResolvedGitConfig`] that exposes a [`HashMap`]-like interface. Users that
    /// only need to read `git-config` files should use that instead.
    ///
    /// Internally, this uses various acceleration data structures to improve
    /// performance of the typical usage behavior of many lookups and relatively
    /// fewer insertions.
    ///
    /// # Multivar behavior
    ///
    /// `git` is flexible enough to allow users to set a key multiple times in
    /// any number of identically named sections. When this is the case, the key
    /// is known as a "multivar". In this case, `raw_value` follows the
    /// "last one wins" approach that `git-config` internally uses for multivar
    /// resolution.
    ///
    /// Concretely, the following config has a multivar, `a`, with the values
    /// of `b`, `c`, and `d`, while `e` is a single variable with the value
    /// `f g h`.
    ///
    /// ```text
    /// [core]
    ///     a = b
    ///     a = c
    /// [core]
    ///     a = d
    ///     e = f g h
    /// ```
    ///
    /// Calling methods that fetch or set only one value (such as [`raw_value`])
    /// key `a` with the above config will fetch `d` or replace `d`, since the last
    /// valid config key/value pair is `a = d`:
    ///
    /// ```
    /// # use std::borrow::Cow;
    /// # use std::convert::TryFrom;
    /// # let git_config = git_config::File::try_from("[core]a=b\n[core]\na=c\na=d").unwrap();
    /// assert_eq!(git_config.raw_value("core", None, "a").unwrap(), Cow::Borrowed("d".as_bytes()));
    /// ```
    ///
    /// Consider the `multi` variants of the methods instead, if you want to work
    /// with all values instead.
    ///
    /// [`ResolvedGitConfig`]: crate::file::ResolvedGitConfig
    /// [`raw_value`]: Self::raw_value
    #[derive(PartialEq, Eq, Clone, Debug, Default)]
    pub struct File<'event> {
        /// The list of events that occur before an actual section. Since a
        /// `git-config` file prohibits global values, this vec is limited to only
        /// comment, newline, and whitespace events.
        pub(crate) frontmatter_events: SectionBody<'event>,
        /// Section name and subsection name to section id lookup tree. This is
        /// effectively a n-tree (opposed to a binary tree) that can have a height
        /// of at most three (including an implicit root node).
        pub(crate) section_lookup_tree: HashMap<SectionHeaderName<'event>, Vec<LookupTreeNode<'event>>>,
        /// SectionId to section mapping. The value of this HashMap contains actual
        /// events.
        ///
        /// This indirection with the SectionId as the key is critical to flexibly
        /// supporting `git-config` sections, as duplicated keys are permitted.
        pub(crate) sections: HashMap<SectionId, SectionBody<'event>>,
        pub(crate) section_headers: HashMap<SectionId, ParsedSectionHeader<'event>>,
        /// Internal monotonically increasing counter for section ids.
        pub(crate) section_id_counter: usize,
        /// Section order for output ordering.
        pub(crate) section_order: VecDeque<SectionId>,
    }
}
pub use types::File;

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
