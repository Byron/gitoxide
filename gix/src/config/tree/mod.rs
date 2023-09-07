//! The tree of supported configuration values for use in [`config_overrides`][crate::open::Options::config_overrides()]
//! or for validating and transforming well-known configuration values.
//!
//! It can also be used to traverse all implemented keys and to validate values before usage as configuration overrides.
//!
//! ### Leniency
//!
//! When validating values, we don't apply leniency here which is left to the caller. Leniency is an application defined configuration
//! to ignore errors on non-security related values, which might make applications more resilient towards misconfiguration.
pub(crate) mod root {
    use super::sections;
    use crate::config::tree::Section;

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
}

mod sections;
pub use sections::{
    branch, checkout, core, credential, extensions, fetch, gitoxide, http, index, protocol, remote, ssh, Author,
    Branch, Checkout, Clone, Committer, Core, Credential, Extensions, Fetch, Gitoxide, Http, Index, Init, Pack,
    Protocol, Remote, Safe, Ssh, Url, User,
};
#[cfg(feature = "blob-diff")]
pub use sections::{diff, Diff};

/// Generic value implementations for static instantiation.
pub mod keys;

///
pub mod key {
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
}

mod traits;
pub use traits::{Key, Link, Note, Section, SubSectionRequirement};
